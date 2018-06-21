use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::CharIndices;

// Types of Tokens
pub enum TokKind<'a>{
    Error,
    Ident,
    ValName,
    Comma,
    Equal,
    Int,
    UntypedInt,
    KnownBits,
    OpenParen,
    CloseParen,
    Comment(&'a str),
    Eof,
}

// Error type
pub enum Error {
    InvalidChar,
}

// Error with location and error string
pub struct LocatedError {
    pub error: Error,
    pub errmsg: String,
    pub location: Location,
}

// Token with type and location
pub struct LocatedToken<'a> {
    pub kind: TokKind<'a>,
    pub location: Location,
}

// Line number specifies the location
// of token or error
#[derive(Clone)]
pub struct Location {     
    pub line_num: usize,
}

// Build the token with all attributes
fn token<'a>(token: TokKind<'a>, loc: Location) -> Result<LocatedToken<'a>, LocatedError> {
    Ok(LocatedToken {
        kind: token,
        location: loc,
    })
}

// Build the error with all attributes
fn error(err: Error, msg: String, loc: Location) -> Result<(), LocatedError> {
    Err(LocatedError {
        error: err,
        errmsg: msg,
        location: loc,
    })
}

// Lexer
#[derive(Clone)]
pub struct Lexer<'a> {
    source: &'a str,

    // Iterator into `source`.
    chars: CharIndices<'a>,

    // Next character to be processed, or `None` at the end.
    lookahead: Option<char>,

    // Index into `source` of lookahead character.
    pos: usize,

    // Current line number.
    line_number: usize,
}

impl<'a> Lexer<'a> {
    // Initialize the lexer.
    pub fn new(s: &str) -> Lexer {
        let mut lex = Lexer {
            source: s,
            chars: s.char_indices(),
            lookahead: None,
            pos: 0,
            line_number: 1,
        };
        lex.next_ch();
        lex
    }

    // Is the currnt character alphabet(a-z | A-Z)) or digit (0-9)?
    pub fn is_alphadigit(&mut self, ch: Option<char>) -> bool {
        match ch {
            Some('0' ... '9') => {
                true
            },
            Some('a' ... 'z') => {
                true
            },
            Some('A' ... 'Z') => {
                true
            },
            Some(_) => {
                false
            },
            _ => {
                false
            },
         }
    }

    // Is the current character a digit?
    pub fn is_digit(&mut self, ch: Option<char>) -> bool {
        match ch {
            Some('0' ... '9') => {
                true
            },
            Some(_) => {
                false
            },
            _ => {
                false
            },
         }
    }

    // Get the next character.
    fn next_ch(&mut self) -> Option<char> {
        // TODO: do we need to look for '\n' at the first?
        // If so, then handle it and increment line_number
         if self.lookahead == Some('\n') {
             self.line_number += 1;
        }
        match self.chars.next() {
            Some((idx, ch)) => {
                self.lookahead = Some(ch);
                self.pos = idx;
            },
            None => {
                self.pos = self.source.len();
                self.lookahead = None;
            }
        }
        self.lookahead
    }

    // Scan rest of the commented line starting with ';'.
    pub fn rest_of_line(&mut self) -> &'a str {
        let begin = self.pos;
        loop {
            match self.next_ch() {
                None | Some('\n') => return &self.source[begin..self.pos],
                _ => {}
            }
        }
    }

    // Scan a comment extending to the end of the current line.
    fn scan_comment(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let loc = self.loc();
        let text = self.rest_of_line();
        token(TokKind::Comment(text), loc)
    }

    // Scan instructions in Souper IR.
    // Examples:
    // %0:i32 = var
    // %a:i64 = add %0, 1:i64
    // infer %a
    // result %a
    fn scan_rest(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let loc = self.loc();
        match self.lookahead {
            // FIXME: ideally there won't be None here, because
            // its handled by getNextToken()
            //None => None,
            Some(',') => {
                self.next_ch();
                println!("Token: Comma");
                token(TokKind::Comma, loc)
            },
            Some('=') => {
                self.next_ch();
                println!("Token: Eq");
                token(TokKind::Equal, loc)
            },
            Some('%') => {
                self.next_ch();
                let mut startPos = self.pos;
                let mut current_ch = self.lookahead.clone();

                // scan the LHS identifier
                while self.is_alphadigit(current_ch) {
                    self.next_ch();
                    current_ch = self.lookahead.clone();
                }

                //FIXME: we want to exit here if error occurs
                // FIXME: we have to eventually return that error kind of token
                // do we have to break with a value even though there is no loop?
                if self.pos - startPos == 0 {
                    error(Error::InvalidChar, "expected an identifier".to_string(), loc.clone());
                    //token(TokKind::Error, loc)
                }

                // Look for bitwidth specifications, if any
                if self.lookahead == Some(':') {
                    self.next_ch();
                    if self.lookahead != Some('i') {
                        error(Error::InvalidChar, "expected 'i' to specify bitwidth".to_string(), loc.clone());
                        //token(TokKind::Error, loc)
                    }

                    // scan the width
                    self.next_ch();
                    let mut widthBegin = self.pos;
                    let mut width = 0;
                    current_ch = self.lookahead.clone();
                    while self.is_digit(current_ch) {
                        self.next_ch();
                        current_ch = self.lookahead.clone();
                    }
                    //FIXME: get the sliced width string and convert
                    //it to int value

                    // Make sure you got something in the width
                    if self.pos - widthBegin == 0 {
                        error(Error::InvalidChar, "expected an integer".to_string(), loc.clone());
                        //token(TokKind::Error, loc)
                    }
                    // FIXME: enable this width = 0 check once width
                    // is actually computed (str -> int)
                    //if width == 0 {
                    //    error(Error::InvalidChar, "width must be atleast 1".to_string(), loc.clone());
                    //    //token(TokKind::Error, loc)
                    //}
                }
                println!("Token: ValName");
                token(TokKind::ValName, loc)
            },
            Some('a' ... 'z') | Some('A' ... 'Z') => {
                self.next_ch();
                println!("Token: Ident");
                token(TokKind::Ident, loc)
            },
            Some('0' ... '9') => {
                self.next_ch();
                println!("Token: Int");
                token(TokKind::Int, loc)
            },
            _ => {
                // FIXME: I think this is not required, do something else
                // with this case.
                self.next_ch();
                // FIXME: random token type was added for the time being
                println!("Token not handled");
                token(TokKind::ValName, loc)
            }
        }
    }

    // Build the location for tokens or errors
    fn loc(&self) -> Location {
        Location {
            line_num: self.line_number,
        }
    }

    // Get next token. This function is a driver to invoke the token generator
    // (scan_rest) to scan the meaningful characters.
    pub fn getNextToken(&mut self) -> Option<Result<LocatedToken<'a>, LocatedError>> {
        loop {
            let loc = self.loc();
            match self.lookahead {
                None => {
                    // Break with an EOF token.
                    break Some(token(TokKind::Eof, loc));
                },
                Some('\n') => {
                    self.next_ch();
                    // FIXME: Do we need to do it here as well?
                    //self.line_number += 1;
                    continue;
                },
                Some(' ') => {
                    self.next_ch();
                    continue;
                },
                Some(':') => {
                    self.next_ch();
                    continue;
                },
                Some('\t') => {
                    self.next_ch();
                    continue;
                },
                Some('\r') => {
                    self.next_ch();
                    continue;
                },
                Some(';') => {
                    Some(self.scan_comment());
                    self.next_ch();
                    continue;
                },
                _ => {
                    Some(self.scan_rest());
                    continue;
                },
            }
        }
    }
}

// Lexer Driver
fn startLexer(text: & str) {
    let mut inputLex = Lexer::new(text);
    // Lex until EOF token is found.
    loop {
        let tok = inputLex.getNextToken();
        match tok {
            Some(Ok(LocatedToken { kind, location })) => {
                break;
            },
            Some(_) => {},
            _ => {}
        }
    }
}

fn main () {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: not enough arguments passed to souper parser");
    }

    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    startLexer(&contents);
}
