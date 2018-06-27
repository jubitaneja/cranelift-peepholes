// Parser for souper tokens

use lexer::{self, Lexer, TokKind, Location, LocatedToken, LocatedError};

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,

    /// Current lookahead token.
    lookahead: Option<TokKind<'a>>,

    /// Location of lookahead.
    loc: Location,

    lex_error: Option<lexer::Error>,
}

impl<'a> Parser<'a> {
    // Initialize the parser.
    pub fn new(s: &str) -> Parser {
        Parser {
            lex: Lexer::new(s),
            lookahead: None,
            loc: Location { line_num: 0 },
            lex_error: None,
        }
    }

    // print token name
    fn get_token_name(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName) => println!("ValName "),
            Some(TokKind::Ident) => println!("Ident "),
            Some(TokKind::Comma) => println!("Comma "),
            Some(TokKind::Equal) => println!("Eq "),
            Some(TokKind::Int) => println!("Int "),
            Some(TokKind::Eof) => println!("EOF "),
            Some(TokKind::Error) => println!("Error "),
            Some(TokKind::UntypedInt) => println!("Untypedint "),
            _ => println!("Token type not handled "),
        }
    }

    // returns the current token
    fn consume_token(&mut self) -> Option<TokKind<'a>> {
        let x = self.lookahead.clone();
        match x {
            Some(TokKind::Error) => {
                // do something here to build an error msg
            },
            _=> {
                match self.lex.get_next_token() {
                    Some(Ok(LocatedToken {kind, location})) => {
                        self.lookahead = Some(kind);
                        self.loc = location;
                    },
                    Some(Err(LocatedError{error, errmsg, location})) => {
                        self.lex_error = Some(error);
                        self.loc = location;
                    },
                    _ => {
                        println!(" *** lexer tokem is any other");
                    },
                }
            },
        }
        self.lookahead.clone()
    }

//    fn consume_token(&mut self) -> Option<TokKind<'a>> {
//        loop {
//            let x = self.lookahead.clone();
//            match x {
//                None => {
//                    println!("parser lookahead initially is None");
//                    match self.lex.get_next_token() {
//                        Some(Ok(LocatedToken { kind, location })) => {
//                            println!(" *** lexer token is OK!");
//                            self.lookahead = Some(kind);
//                            self.loc = location;
//                            continue;
//                        },
//                        Some(Err(LocatedError { error, errmsg, location })) => {
//                            println!(" *** lexer token is ERR!");
//                            self.lex_error = Some(error);
//                            self.loc = location;
//                            break;
//                        },
//                        _ => {
//                            println!(" *** lexer tokem is any other");
//                            break;
//                        }
//                    }
//                },
////                Some(x) => break,
//                Some(x) => {
//                    self.lookahead = Some(x);
//                    break;
//                },
//            }
//        }
//        self.lookahead.clone()
//    }

    pub fn is_eof(&mut self) -> bool {
        match self.lookahead {
            Some(TokKind::Eof) => true,
            _ => false,
        }
    }

    // parse each instruction
    fn parse_inst(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName) => {
                println!("ValName in parseInst");
                self.consume_token();
                match self.lookahead {
                    Some(TokKind::Equal) => {
                        println!("Parser: GOOD! We found Valname -> Eq");
                        self.consume_token();
                    },
                    _ => {
                        println!("unexpected token after valname tok -> ??");
                    },
                }
            },
            Some(TokKind::Ident) => {
                println!("Ident in parseInst");
                self.consume_token();
            },
            _ => {
                println!("Default case");
            },
        }
    }

    // parse line
    fn parse_lines(&mut self) {
        // Loop until EOF token
        loop {
            println!("Before");
            self.get_token_name();
            let mut x = self.consume_token();
            println!("After");
            self.get_token_name();
            match x {
                Some(TokKind::Eof) => {
                    break;
                },
                _ => {
                    self.parse_inst();
                    continue;
                },
            }
        }
        if let Some(ref mut err) = self.lex_error {
            println!("Error case");
            return match err {
                //lexer::Error::InvalidChar => err!(self.loc, "invalid character"),
                lexer::Error::InvalidChar => println!("Error case in parse_lines"),
            };
        }
    }
}


pub fn parse(text: &str) {
    let mut p = Parser::new(text);

    p.consume_token();

    // FIXMEL we want a ret value from parse_line() to
    // be used later for code gen purpose
    p.parse_lines();
}
