// Lexer module

use std::str::CharIndices;

// Types of Tokens
#[derive(Clone)]
pub enum TokKind {
    Error,
    Ident(String),
    ValName(String, u32),
    Comma,
    Equal,
    Implies,
    Int(u32, i32),
    Comment(String),
    Eof,

    #[allow(dead_code)]
    UntypedInt,
}

// Error type
#[derive(Debug, Clone)]
pub enum Error {
    InvalidChar,
}

// Error with location and error string
#[derive(Debug, Clone)]
pub struct LocatedError {
    pub error: Error,
    pub errmsg: String,
    pub location: Location,
}

// Token with type and location
pub struct LocatedToken {
    pub kind: TokKind,
    pub location: Location,
}

// Line number specifies the location
// of token or error
#[derive(Clone, Debug)]
pub struct Location {
    pub line_num: usize,
}

// Build the token with all attributes
fn token<'a>(token: TokKind, loc: Location) -> Result<LocatedToken, LocatedError> {
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

    // Is the current character a digit?
    pub fn is_digit(&mut self, ch: Option<char>) -> bool {
        match ch {
            Some('0'..='9') => true,
            Some(_) => false,
            _ => false,
        }
    }

    pub fn evaluate_const_value(&mut self, ch: Option<char>, neg: bool, const_val: &mut i32) {
        match ch {
            Some(c) => {
                if c >= '0' && c <= '9' {
                    let cons = c as i32;
                    let zero_val = '0' as i32;
                    // split the addition to avoid add with overflow
                    // issues for INT_MAX values.
                    let x = *const_val * 10;
                    let y = cons - zero_val;
                    if neg {
                        *const_val = x - y;
                    } else {
                        *const_val = x + y;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn evaluate_width(&mut self, ch: Option<char>, width: &mut u32) {
        match ch {
            Some(w) => {
                if w >= '0' && w <= '9' {
                    let width_val = w as u32;
                    let zero_val = '0' as u32;
                    *width = *width * 10 + width_val - zero_val;
                }
            }
            _ => {}
        }
    }

    pub fn scan_bitwidth(&mut self) -> u32 {
        let loc = self.loc();
        self.next_ch();
        if self.lookahead != Some('i') {
            error(
                Error::InvalidChar,
                "expected 'i' to specify bitwidth".to_string(),
                loc.clone(),
            )
            .unwrap();
            //token(TokKind::Error, loc)
        }

        // scan the width value
        self.next_ch();
        let width_begin = self.pos;
        let mut current_ch = self.lookahead.clone();
        let mut width_value: u32 = 0;
        while self.is_digit(current_ch) {
            self.evaluate_width(current_ch, &mut width_value);
            self.next_ch();
            current_ch = self.lookahead.clone();
        }

        // Make sure you got something in the width
        if self.pos - width_begin == 0 {
            error(
                Error::InvalidChar,
                "expected an integer".to_string(),
                loc.clone(),
            )
            .unwrap();
            //token(TokKind::Error, loc)
        }
        //println!("-------------- width = {} -------------\n", width_value);
        if width_value == 0 {
            error(
                Error::InvalidChar,
                "width must be atleast 1".to_string(),
                loc.clone(),
            )
            .unwrap();
            //token(TokKind::Error, loc)
        }

        width_value
    }

    // Is the current character an alphabet?
    pub fn is_alphabet(&mut self, ch: Option<char>) -> bool {
        match ch {
            Some('a'..='z') | Some('A'..='Z') => true,
            Some(_) => false,
            _ => false,
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
            }
            None => {
                self.pos = self.source.len();
                self.lookahead = None;
            }
        }
        self.lookahead
    }

    // Scan rest of the commented line starting with ';'.
    pub fn rest_of_line(&mut self) -> String {
        let begin = self.pos;
        loop {
            match self.next_ch() {
                None | Some('\n') => return self.source[begin..self.pos].to_owned(),
                _ => {}
            }
        }
    }

    //scan implies symbol in (LHS -> RHS)
    fn scan_implies(&mut self) -> Result<LocatedToken, LocatedError> {
        let loc = self.loc();
        let current_ch = self.lookahead;
        match current_ch {
            Some('>') => {
                //println!("lexer: implies second char '>'\n");
                self.next_ch();
                token(TokKind::Implies, loc)
            }
            _ => token(TokKind::Error, loc),
        }
    }

    // Scan a comment extending to the end of the current line.
    fn scan_comment(&mut self) -> Result<LocatedToken, LocatedError> {
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
    fn scan_rest(&mut self) -> Result<LocatedToken, LocatedError> {
        let loc = self.loc();
        match self.lookahead {
            // FIXME: ideally there won't be None here, because
            // its handled by get_next_token()
            //None => None,
            Some(',') => {
                self.next_ch();
                //println!("Token: Comma");
                token(TokKind::Comma, loc)
            }
            Some('=') => {
                self.next_ch();
                //println!("Token: Eq");
                token(TokKind::Equal, loc)
            }
            Some('%') => {
                self.next_ch();
                let start_pos = self.pos;
                let mut current_ch = self.lookahead.clone();

                // scan the LHS identifier
                while self.is_alphabet(current_ch) | self.is_digit(current_ch) {
                    self.next_ch();
                    current_ch = self.lookahead.clone();
                }

                //FIXME: we want to exit here if error occurs
                // FIXME: we have to eventually return that error kind of token
                // do we have to break with a value even though there is no loop?
                if self.pos - start_pos == 0 {
                    error(
                        Error::InvalidChar,
                        "expected an identifier".to_string(),
                        loc.clone(),
                    )
                    .unwrap();
                    //token(TokKind::Error, loc)
                }
                let lhs_val_name = self.source[start_pos - 1..self.pos].to_owned();

                // Look for bitwidth specifications, if any
                let mut width: u32 = 0;
                if self.lookahead == Some(':') {
                    width = self.scan_bitwidth();
                }

                token(TokKind::ValName(lhs_val_name, width), loc)
            }
            // FIXME: modularize all these cases
            Some('a'..='z') | Some('A'..='Z') => {
                let begin_pos = self.pos;
                let mut current_ch = self.lookahead.clone();
                while self.is_alphabet(current_ch) {
                    self.next_ch();
                    current_ch = self.lookahead.clone();
                }
                let text = self.source[begin_pos..self.pos].to_owned();
                //println!("Token: Ident");
                token(TokKind::Ident(text), loc)
            }
            // FIXME: Take care of negative int too
            Some('0'..='9') | Some('-') => {
                let mut current_ch = self.lookahead.clone();
                let mut negative = false;
                if current_ch == Some('-') {
                    negative = true;
                    self.next_ch();
                    current_ch = self.lookahead.clone();
                    if !self.is_digit(current_ch) {
                        panic!("Expected a digit to follow after negative sign\n");
                    }
                }
                let const_begin = self.pos;
                let mut const_val: i32 = 0;
                while self.is_digit(current_ch) {
                    self.next_ch();
                    self.evaluate_const_value(current_ch, negative, &mut const_val);
                    current_ch = self.lookahead.clone();
                }
                let const_end = self.pos;

                if const_end - const_begin == 0 {
                    error(
                        Error::InvalidChar,
                        "expected an integer".to_string(),
                        loc.clone(),
                    )
                    .unwrap();
                }

                let mut width: u32 = 0;
                if self.lookahead == Some(':') {
                    width = self.scan_bitwidth();
                }
                token(TokKind::Int(width, const_val), loc)
            }
            _ => {
                // FIXME: I think this is not required, do something else
                // with this case.
                self.next_ch();
                // FIXME: random token type was added for the time being
                //println!("Token not handled");
                token(TokKind::Error, loc)
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
    pub fn get_next_token(&mut self) -> Option<Result<LocatedToken, LocatedError>> {
        loop {
            let loc = self.loc();
            match self.lookahead {
                None => {
                    // Break with an EOF token.
                    break Some(token(TokKind::Eof, loc));
                }
                Some('\n') => {
                    self.next_ch();
                    // FIXME: Do we need to do it here as well?
                    //self.line_number += 1;
                    continue;
                }
                Some(' ') => {
                    self.next_ch();
                    continue;
                }
                Some(':') => {
                    self.next_ch();
                    continue;
                }
                Some('\t') => {
                    self.next_ch();
                    continue;
                }
                Some('\r') => {
                    self.next_ch();
                    continue;
                }
                Some(';') => {
                    Some(self.scan_comment());
                    self.next_ch();
                    continue;
                }
                Some('#') => {
                    // delimiter token for separating
                    // the optimization patterns in input file
                    self.next_ch();
                    break Some(token(TokKind::Eof, loc));
                }
                Some('~') => {
                    //println!("lexer: found implies starting char '-'\n");
                    self.next_ch();
                    break Some(self.scan_implies());
                }
                _ => {
                    break Some(self.scan_rest());
                }
            }
        }
    }
}

// Lexer Driver
#[allow(dead_code)]
pub fn start_lexer(text: &str) {
    let mut input_lex = Lexer::new(text);
    // Lex until EOF token is found.
    loop {
        let tok = input_lex.get_next_token();
        if let Some(Ok(LocatedToken {
            kind: TokKind::Eof, ..
        })) = tok
        {
            break;
        }
    }
}
