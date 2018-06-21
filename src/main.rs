use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::CharIndices;

pub enum TokKind<'a>{
//pub enum TokKind {
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

//pub struct Token {
//    //kind: Kind,
//    name: &str,
//    width: u32,
//}

pub enum Error {
    InvalidChar,
}

/// An `Error` with an associated Location.
pub struct LocatedError {
    pub error: Error,
    pub errmsg: String,
    pub location: Location,
}

pub struct LocatedToken<'a> {
//pub struct LocatedToken {
    pub kind: TokKind<'a>,
    pub location: Location,
}

#[derive(Clone)]
pub struct Location {     
    pub line_num: usize,
}

//jubi: ret type result<locatedtoken<'a>>
fn token<'a>(token: TokKind<'a>, loc: Location) -> Result<LocatedToken<'a>, LocatedError> {
    Ok(LocatedToken {
        kind: token,
        location: loc,
    })
}

fn error(err: Error, msg: String, loc: Location) -> Result<(), LocatedError> {
    Err(LocatedError {
        error: err,
        errmsg: msg,
        location: loc,
    })
}

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
    pub fn new(s: &str) -> Lexer {
        let mut lex = Lexer {
            source: s,
            chars: s.char_indices(),
            lookahead: None,
            pos: 0,
            line_number: 1,
        };
        //get to the first char of source text
        println!("****** Call next_ch() from new() ******");
        lex.next_ch();
        lex
    }

    pub fn is_alphadigit(&mut self, ch: Option<char>) -> bool {
        println!("\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t in while starting current ch == {:?}", ch);
        match ch {
            Some('0' ... '9') => {
                println!("ch in 0-9 is = {:?}", self.lookahead);
                true
            },
            Some('a' ... 'z') => {
                println!("ch in a-z is = {:?}", self.lookahead);
                true
            },
            Some('A' ... 'Z') => {
                println!("ch in A-Z is = {:?}", self.lookahead);
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

    fn next_ch(&mut self) -> Option<char> {
        // TODO: do we need to look for '\n' at the first?
        // If so, then handle it and increment line_number
        println!("ch in next_ch starting is:::::: {:?}", self.lookahead);
         if self.lookahead == Some('\n') {
             println!("Am I newline char??????????");
             self.line_number += 1;
        }
        match self.chars.next() {
            // look for all posible chars in souper IR
            Some((idx, ch)) => {
                self.lookahead = Some(ch);
                self.pos = idx;
                println!("No specific match of particular char here --- ");
                println!("pos of ch === {}", self.pos);
                println!("first match in ch ==== {:?}", self.lookahead);
            },
            None => {
                println!("++++++++++++ In None match in next_ch() func ++++++");
                self.pos = self.source.len();
                println!("Pos on eof None matcher === {}", self.pos);
                self.lookahead = None;
            }
        }
        self.lookahead
    }

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
    // //jubi: ret type result<locatedtoken<'a>>
    fn scan_comment(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let loc = self.loc();
        let text = self.rest_of_line();
        token(TokKind::Comment(text), loc)
    }

    // scan all chars except newline, comments, and
    // return a valid token type
    // //jubi: ret type result<locatedtoken<'a>>
    fn scan_rest(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let loc = self.loc();
        println!("specific match begins now for % - , - = - [a-z] ...");
        match self.lookahead {
            // FIXME: ideally there won't be None here, because
            // its handled by getNextToken()
            //None => None,
            Some(',') => {
                self.next_ch();
                token(TokKind::Comma, loc)
            },
            Some('=') => {
                self.next_ch();
                token(TokKind::Equal, loc)
            },
            Some('%') => {
                println!("%%%%%%%%%%%%%");
                println!("ch == {:?}", self.lookahead);
                println!("call to get next_ch now");
                self.next_ch();
                println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
                println!("Lookahead char is == {:?}", self.lookahead);
                println!("\t\t\t\t\t\t\t\tpos begin == {}", self.pos);
                println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
                let mut startPos = self.pos;
                // TODO: refactor this loop (make a func: self.is_alpha_digit(sef.lookahead))
                let mut current_ch = self.lookahead.clone();
                println!("\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t current ch == {:?}", current_ch);
                while self.is_alphadigit(current_ch) {
                    self.next_ch();
                    current_ch = self.lookahead.clone();
                }
//                loop {
//                    match self.lookahead {
//                        Some('0' ... '9') => {
//                            println!("ch in 0-9 is = {:?}", self.lookahead);
//                            self.next_ch();
//                            continue;
//                        },
//                        Some('a' ... 'z') => {
//                            println!("ch in a-z is = {:?}", self.lookahead);
//                            self.next_ch();
//                            continue;
//                        },
//                        Some('A' ... 'Z') => {
//                            println!("ch in A-Z is = {:?}", self.lookahead);
//                            self.next_ch();
//                            continue;
//                        },
//                        Some(_) => {
//                            break;
//                        },
//                        _ => {
//                            break;
//                        },
//                    };
//                }
                println!("\t\t\t\t\t\t\t\tpos end == {}", self.pos);
//                let errloc = loc;
                if self.pos - startPos == 0 {
                    //FIXME: we want to exit here if error occurs
                    error(Error::InvalidChar, "expected an identifier".to_string(), loc.clone());
                }
                //while (self.is_alphadigit(self.lookahead)) {
                  //  self.next_ch();
                //}
                token(TokKind::Ident, loc)
            },
            _ => {
                println!("Faltu ka case -- hata do isko");
                println!("ch == {:?}", self.lookahead);
                self.next_ch();
                println!("returned from next_ch __________");
                token(TokKind::ValName, loc)
            }
        }
    }

    fn loc(&self) -> Location {
        Location {
            line_num: self.line_number,
        }
    }

    pub fn getNextToken(&mut self) -> Option<Result<LocatedToken<'a>, LocatedError>> {
        println!("------------- In getnexttoken() ----------------");
        loop {
            println!("----------------------- Inner loop in next -------------------------");
            let loc = self.loc();
            println!("Line number === {}", loc.line_num);
            println!("__________ Lookahead after everything is :::::::::: {:?}", self.lookahead);
            match self.lookahead {
                None => {
                    println!("---------- eof is reached ---");
                    break Some(token(TokKind::Eof, loc));
                    //break Some(error(Error::InvalidChar, loc));
                },
                Some('\n') => {
                    println!("********************************what??? *****");
                    self.next_ch();
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
                    println!("in specific case matching now, but nothing for % yet");
                    Some(self.scan_rest());
                    println!("returned call from scan_rest _______");
                    continue;
                },
            }
        }
    }
}

fn startLexer(text: & str) {
    let mut inputLex = Lexer::new(text);
    loop {
        println!("---- Major loop begins ------");
        let tok = inputLex.getNextToken();
        match tok {
            Some(Ok(LocatedToken { kind, location })) => {
                println!("In the finale");
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

    println!("With text:{}", contents);

    startLexer(&contents);
}
