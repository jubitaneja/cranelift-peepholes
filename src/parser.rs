// Parser for souper tokens

use lexer::{self, Lexer, TokKind, Location, LocatedToken, LocatedError};

pub enum InstKind {
    Var,
    Const,
    UntypedConst,
    Add,
    AddNW,
    AddNSW,
    AddNUW,
    Sub,
    Mul,
    Zext,
    NoneType,
}

pub struct Inst {
    pub kind: InstKind,
    // pub instWidth: u32,
    // ops: Vec<Inst>
}

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

    // return inst Kind for the given inst names
    fn get_inst_kind(&mut self, name: &str) -> InstKind {
        match name {
            "var" => InstKind::Var,
            "add" => InstKind::Add,
            "mul" => InstKind::Mul,
            _ => InstKind::NoneType,
        }
    }

    // print token name
    fn get_token_name(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName) => println!("ValName "),
            Some(TokKind::Ident(text)) => println!("Ident "),
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
                        println!("Error: in consume_token(), invalid token type");
                    },
                }
            },
        }
        self.lookahead.clone()
    }

    pub fn is_eof(&mut self) -> bool {
        match self.lookahead {
            Some(TokKind::Eof) => true,
            _ => false,
        }
    }

    //fn parse_ops(&mut self) -> Option<Inst> {
    fn parse_ops(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName) => {
                // error checking: self.width == 0 => error unexpected width of op

                // Inst I = createInst with inst width, instvalname
                // InstContext IC; IC.getInst()
                // if I is None => error "%<x> is not an inst"

                println!("Op: Valname");
                self.consume_token();

                //return I
            },
            Some(TokKind::Int) => {
                // get the value of const
                // build const inst
                // Inst I = IC.getConst()
                println!("Op: Int");
                self.consume_token();

                // return I
            },
            Some(TokKind::UntypedInt) => {
                // get the value of const
                // build untyped const inst
                // Inst I = IC.getUntypedConst()
                println!("Op: Untyped Int");
                self.consume_token();

                // return I
            },
            _ => {
                // build error
                println!("unexpected token type of Op");
            },
        }
    }
    //}

    fn parse_inst_types(&mut self) {
        if let Some(TokKind::Ident(text)) = self.lookahead {
            match self.get_inst_kind(text) {
                InstKind::Var => {
                    // TODO: error checking
                    // instwidth == 0 => error "var inst expects atleast width=1"

                    self.consume_token();
                    // Deal with dataflow facts later here!

                    // create Var instruction and return that
                    // self.createVar(instValName, instWidth);
                    println!("Build Var Instruction");
                },
                _ => {
                    let instKind = self.get_inst_kind(text);
                    // Start parsing Ops
                    self.consume_token();

                    loop {
                        //op = self.parse_ops();
                        self.parse_ops();

                        // parse_ops() already consumed next token, so look
                        // for comma token now.
                        match self.lookahead {
                            Some(TokKind::Comma) => {
                                self.consume_token();
                            },
                            _ => break,
                        }
                    }
                    println!("Build {} instruction", text);
                    // TODO: return the build instruction
                    // IC.getInst(instwidth, instkind, ops)
                },
            }
        }
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
            Some(TokKind::Equal) => {
                self.consume_token();

                // Look for ident tokens like, var; add; phi; etc.
                match self.lookahead {
                    Some(TokKind::Ident(text)) => {
                        // Deal with actual part of inst, like:
                        // var
                        // add %0, %1
                        // phi %0, 1, 2
                        self.parse_inst_types();
                    },
                    _ => {
                        // build error "expected identifier here:Valname -> Eq -> Ident"
                        println!("Error: Expected valname -> Eq -> ??? Ident");
                    },
                }
            },
            _ => {
                // Build error "expected ="
                println!("Error: Expected ValName -> ???? Eq");
            },
        }
    }

    fn parse_ident_inst(&mut self) {
        // extend this later
        println!("Ident type instructions are not yet handled, like infer, cand, result, pc, blockpc");
        // FIXME: For now, I am simply cnsuming further tokens
        self.consume_token();
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
            },
            Some(TokKind::Ident(text)) => {
                self.parse_ident_inst();
            },
            _ => {
                println!("Error: Instruction either start with ValName token or Ident token");
                // FIXME: Jubi: Build an error
            },
        }
    }
}


pub fn parse(text: &str) {
    let mut p = Parser::new(text);

    p.consume_token();

    // FIXMEL we want a ret value from parse_line() to
    // be used later for code gen purpose

    loop {
        match p.lookahead {
            Some(TokKind::Eof) => break,
            //_ => p.parse_inst(),
            _ => {
                p.parse_inst();
            },
        }
    }
}
