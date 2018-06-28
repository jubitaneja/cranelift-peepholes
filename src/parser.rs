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

    fn parse_ops(&mut self) {
//    fn parse_ops(&mut self) -> Option<Inst> {
//        match self.lookahead {
//            Some(TokKind::ValName) => {
//                // createInst with inst width, instvalname
//            },
//            Some(TokKind::Int) => {
//            },
//            Some(TokKind::UntypedInt) => {
//            },
//            _ => {
//                // build error
//                println!("unexpected ops type");
//            },
        }
    }

    fn parse_inst_types(&mut self) {
//        instKind = self.instkind;
//
//        match instkind {
//            Inst::None => {
//                // deal with "block" inst here
//            },
//            Inst::Var => {
//                // error checking:
//                // if instwidth == 0 => error (non-zero width expected for var inst)
//                self.consume_token();
//                // Later deal with data flow facts here
//
//                // build var inst and return that
//                self.createVar(instValName, instWidth);
//            },
//            Inst::Phi => {
//                // Deal with it Later
//            },
//            _ => {
//                //Start parsing Ops here
//                instkind = self.instKind;
//                self.consume_token();
//
//                // here we have first op of binary/unary insts
//                loop {
//                    op = self.parse_ops();
//                    match op {
//                        Error => break,
//                        _ => {
//                           // push current op to a vector of Ops
//                           self.consume_token();
//
//                           // now you are the token comma or you move onto the next inst
//                           match self.lookahead {
//                               Some(TokKind::Comma) => {
//                                   self.consume_token();
//                               },
//                               _ => break,
//                           }
//                        },
//                    }
//                }
//                self.createInst(instKind, InstWidth, OpsList);
//            },
//        }
    }

    fn parse_valname_inst(&mut self) {
        // FIXME: Jubi: Add this info to token struct and get it
        // instwidth = self.width
        // instValName = self.instValname

        // Do error handling:
        // check if %1 (instValName) is already declared as an Inst?
        // context.getInst()

        self.consume_token();

        // Look for Equal token now
        match self.lookahead {
            _ => {
                // Build error "expected ="
                println!("Expected ValName -> ???? Eq");
            },
            Some(TokKind::Equal) => {
                self.consume_token();

                // Look for ident tokens like, var; add; phi; etc.
                match self.lookahead {
                    _ => {
                        // build error "expected identifier here:Valname -> Eq -> Ident"
                        println!("Expected valname -> Eq -> ??? Ident");
                    },
                    Some(TokKind::Ident) => {
                        // Deal with actual part of inst, like:
                        // var
                        // add %0, %1
                        // phi %0, 1, 2
                        self.parse_inst_types();
                    },
                }
            },
        }
    }

    fn parse_ident_inst(&mut self) {
    }

    // parse each instruction
    fn parse_inst(&mut self) {
        // Instructions start either with valname or Ident
        // Example:
        // %1:i32 = .... 
        // cand ... , infer ... , result ...
        // pc ... , blockpc ... , 

        match self.lookahead {
            Some(TokKind::ValName) => {
                self.parse_valname_inst();
//                self.consume_token();
//                match self.lookahead {
//                    Some(TokKind::Equal) => {
//                        println!("Parser: GOOD! We found Valname -> Eq");
//                        self.consume_token();
//                    },
//                    _ => {
//                        println!("unexpected token after valname tok -> ??");
//                    },
//                }
            },
            Some(TokKind::Ident) => {
                self.parse_ident_inst();
//                println!("Ident in parseInst");
//                self.consume_token();
            },
            _ => {
                println!("Error: Instruction either start with ValName token or Ident token");
                // FIXME: Jubi: Build an error
            },
        }
    }

    // parse line
///    fn parse_lines(&mut self) {
///        // Loop until EOF token
///        loop {
///            println!("Before");
///            self.get_token_name();
///            let mut x = self.consume_token();
///            println!("After");
///            self.get_token_name();
///            match x {
///                Some(TokKind::Eof) => {
///                    break;
///                },
///                _ => {
///                    self.parse_inst();
///                    continue;
///                },
///            }
///        }
///        if let Some(ref mut err) = self.lex_error {
///            println!("Error case");
///            return match err {
///                //lexer::Error::InvalidChar => err!(self.loc, "invalid character"),
///                lexer::Error::InvalidChar => println!("Error case in parse_lines"),
///            };
///        }
///    }
}


pub fn parse(text: &str) {
    let mut p = Parser::new(text);

    p.consume_token();

    // FIXMEL we want a ret value from parse_line() to
    // be used later for code gen purpose

    loop {
        match self.lookahead {
            Some(TokKind::Eof) => break,
            _ => p.parse_inst(),
        }
    }
}
