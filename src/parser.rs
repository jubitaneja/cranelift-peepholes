// Parser for souper tokens

use lexer::{self, Lexer, LocatedError, LocatedToken, Location, TokKind};
use std::collections::HashMap;

#[derive(Clone)]
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
    Eq,
    Ne,
    Slt,
    Ult,
    Sle,
    Ule,
    And,
    Or,
    Xor,
    Shl,
    Lshr,
    Ashr,
    Ctpop,
    Ctlz,
    Cttz,
    Zext,
    Infer,
    ResultInst,
    Implies,
    NoneType,
}

#[derive(Clone)]
pub enum SouperOpType {
    Index,
    Constant,
}

pub struct SouperOperand {
    pub kind: SouperOpType,
    // what should be the type of constants values?
    pub idx_val: Option<usize>,
    pub const_val: Option<i32>, //FIXME: fix this width of const value to i64 or something else?
    pub width: u32,
}

pub struct Inst<'a> {
    pub kind: InstKind,
    pub lhs: &'a str,
    pub width: u32,
    pub var_number: Option<u32>,
    pub ops: Option<Vec<SouperOperand>>,
}

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,

    /// Current lookahead token.
    lookahead: Option<TokKind<'a>>,

    /// Location of lookahead.
    loc: Location,

    lex_error: Option<lexer::Error>,

    /// LHS Valname
    lhs_valname: &'a str,

    /// width
    width: u32,

    // inst count
    var_count: u32,

    // hash map of LHS valnames to Index values
    lhsValNames_to_Idx: HashMap<&'a str, usize>,
}

impl<'a> Parser<'a> {
    // Initialize the parser.
    pub fn new(s: &str) -> Parser {
        Parser {
            lex: Lexer::new(s),
            lookahead: None,
            loc: Location { line_num: 0 },
            lex_error: None,
            lhs_valname: "",
            width: 0,
            var_count: 0,
            lhsValNames_to_Idx: HashMap::new(),
        }
    }

    fn create_var(&mut self, instkind: InstKind, instname: &'a str, instwidth: u32) -> Inst<'a> {
        // return the inst struct with details
        // FIXME: add more details later if required
        self.var_count += 1;
        Inst {
            kind: instkind,
            lhs: instname,
            width: instwidth,
            var_number: Some(self.var_count),
            ops: None,
        }
    }

    fn create_inst(
        &mut self,
        instkind: InstKind,
        instname: &'a str,
        instwidth: u32,
        ops: Vec<SouperOperand>,
    ) -> Inst<'a> {
        // return the inst struct with details
        // FIXME: add more details later if required
        // Add Ops details too here: Major TODO
        Inst {
            kind: instkind,
            lhs: instname,
            width: instwidth,
            var_number: None,
            ops: Some(ops),
        }
    }

    // return inst Kind for the given inst names
    fn get_inst_kind(&mut self, name: &str) -> InstKind {
        match name {
            "var" => InstKind::Var,
            "add" => InstKind::Add,
            "mul" => InstKind::Mul,
            "sub" => InstKind::Sub,
            "eq" => InstKind::Eq,
            "ne" => InstKind::Ne,
            "slt" => InstKind::Slt,
            "ult" => InstKind::Ult,
            "sle" => InstKind::Sle,
            "ule" => InstKind::Ule,
            "and" => InstKind::And,
            "or" => InstKind::Or,
            "xor" => InstKind::Xor,
            "shl" => InstKind::Shl,
            "lshr" => InstKind::Lshr,
            "ashr" => InstKind::Ashr,
            "ctpop" => InstKind::Ctpop,
            "ctlz" => InstKind::Ctlz,
            "cttz" => InstKind::Cttz,
            "infer" => InstKind::Infer,
            "result" => InstKind::ResultInst,
            "->" => InstKind::Implies,
            _ => InstKind::NoneType,
        }
    }

    // return souper inst kind name for the given inst kind
    fn get_kind_name(&mut self, kind: InstKind) -> String {
        match kind {
            InstKind::Var => "var".to_string(),
            InstKind::Add => "add".to_string(),
            InstKind::Mul => "mul".to_string(),
            InstKind::Sub => "sub".to_string(),
            InstKind::Eq => "eq".to_string(),
            InstKind::Ne => "ne".to_string(),
            InstKind::Slt => "slt".to_string(),
            InstKind::Ult => "ult".to_string(),
            InstKind::Sle => "sle".to_string(),
            InstKind::Ule => "ule".to_string(),
            InstKind::And => "and".to_string(),
            InstKind::Or => "or".to_string(),
            InstKind::Xor => "xor".to_string(),
            InstKind::Shl => "shl".to_string(),
            InstKind::Lshr => "lshr".to_string(),
            InstKind::Ashr => "ashr".to_string(),
            InstKind::Ctpop => "ctpop".to_string(),
            InstKind::Ctlz => "ctlz".to_string(),
            InstKind::Cttz => "cttz".to_string(),
            InstKind::ResultInst => "result".to_string(),
            InstKind::Implies => "->".to_string(),
            InstKind::Infer => "infer".to_string(),
            _ => "Inst Kind name is not yet handled in function: get_kind_name()".to_string(),
        }
    }

    // print token name
    fn get_token_name(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName(lhs, width)) => println!("ValName "),
            Some(TokKind::Ident(text)) => println!("Ident "),
            Some(TokKind::Comma) => println!("Comma "),
            Some(TokKind::Equal) => println!("Eq "),
            Some(TokKind::Int(width, constVal)) => println!("Int "),
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
            }
            _ => match self.lex.get_next_token() {
                Some(Ok(LocatedToken { kind, location })) => {
                    self.lookahead = Some(kind);
                    self.loc = location;
                }
                Some(Err(LocatedError {
                    error,
                    errmsg,
                    location,
                })) => {
                    self.lex_error = Some(error);
                    self.loc = location;
                }
                _ => {
                    println!("Error: in consume_token(), invalid token type");
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

    fn parse_ops(&mut self) -> Vec<SouperOperand> {
        let mut ops: Vec<SouperOperand> = Vec::new();
        loop {
            let op = self.parse_op();
            ops.push(op);

            // parse_op() already consumed next token, so look
            // for comma token now.
            match self.lookahead {
                Some(TokKind::Comma) => {
                    self.consume_token();
                }
                _ => break,
            }
        }
        ops
    }

    fn parse_op(&mut self) -> SouperOperand {
        match self.lookahead {
            Some(TokKind::ValName(lhs, width)) => {
                let mut value = None;
                for (key, val) in &self.lhsValNames_to_Idx {
                    if key == &lhs {
                        value = Some(*val);
                    }
                }
                self.consume_token();

                SouperOperand {
                    kind: SouperOpType::Index,
                    idx_val: value,
                    const_val: None,
                    width: width,
                }
            }
            Some(TokKind::Int(width, constVal)) => {
                // get the value of const
                // build const inst
                // Inst I = IC.getConst()
                self.consume_token();

                SouperOperand {
                    kind: SouperOpType::Constant,
                    idx_val: None,
                    const_val: Some(constVal),
                    width: width,
                }
            }
            Some(TokKind::UntypedInt) => {
                // get the value of const
                // build untyped const inst
                // Inst I = IC.getUntypedConst()
                self.consume_token();

                SouperOperand {
                    kind: SouperOpType::Constant,
                    idx_val: None,
                    const_val: Some(0), //FIXME: should it be None? verify again
                    width: 0,
                }
            }
            _ => {
                panic!("unexpected token type of Op");
            }
        }
    }

    fn parse_inst_types(&mut self) -> Inst<'a> {
        if let Some(TokKind::Ident(text)) = self.lookahead {
            match self.get_inst_kind(text) {
                InstKind::Var => {
                    // TODO: error checking
                    // instwidth == 0 => error "var inst expects atleast width=1"

                    self.consume_token();
                    // TODO: Deal with dataflow facts later here!

                    let instname = self.lhs_valname.clone();
                    // TODO: collect more attributes of var
                    let instwidth = self.width.clone();

                    self.create_var(InstKind::Var, instname, instwidth)
                }
                _ => {
                    let instKind = self.get_inst_kind(text);

                    // Start parsing Ops
                    self.consume_token();
                    let ops = self.parse_ops();

                    //println!("Build {} instruction", text);
                    // TODO: return the build instruction
                    // IC.getInst(instwidth, instkind, ops)
                    //Some(self.create_inst(instKind, self.lhs_valname.clone()))
                    let instname = self.lhs_valname.clone();
                    let instwidth = self.width.clone();

                    // TODO: Add width to these insts
                    self.create_inst(instKind, instname, instwidth, ops)
                }
            }
        } else {
            panic!("Error: fn: parse_inst_types()");
        }
    }

    fn parse_valname_inst(&mut self) -> Inst<'a> {
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
                    Some(TokKind::Ident(text)) => self.parse_inst_types(),
                    _ => {
                        panic!("Error: Expected a valid Identifier after ValName -> Eq token");
                    }
                }
            }
            _ => {
                panic!("Error: Expected Eq token followed by Valname token");
            }
        }
    }

    // parse instructions that start with an identifier
    // example: infer, cand, pc, blockpc inst in Souper IR
    fn parse_ident_inst(&mut self) -> Inst<'a> {
        if let Some(TokKind::Ident(text)) = self.lookahead {
            match self.get_inst_kind(text) {
                InstKind::Infer => {
                    self.consume_token();
                    match self.lookahead {
                        Some(TokKind::ValName(lhs, width)) => {
                            let ops = self.parse_ops();
                            //error checking on ops length
                            assert!(
                                ops.len() == 1,
                                "expected one operand for infer instruction, but found {}",
                                ops.len()
                            );
                            //println!("Parser Build Infer instruction");
                            //self.create_inst(InstKind::Infer, lhs, width, ops)
                            //FIXED
                            self.create_inst(InstKind::Infer, "infer", width, ops)
                        }
                        _ => {
                            panic!("unexpected infer instruction operand");
                        }
                    }
                }
                InstKind::ResultInst => {
                    self.consume_token();
                    match self.lookahead {
                        Some(TokKind::ValName(lhs, width)) => {
                            let ops = self.parse_ops();
                            //error checking on ops length
                            assert!(
                                ops.len() == 1,
                                "expected one operand for infer instruction, but found {}",
                                ops.len()
                            );
                            //println!("Parsing build Result Inst\n");
                            //self.create_inst(InstKind::ResultInst, lhs, width, ops)
                            //FIXED
                            self.create_inst(InstKind::ResultInst, "result", width, ops)
                        }
                        // Result inst can have a typed int as an operand as well.
                        // We will make it a rule that Souper's result inst *DOES NOT*
                        // have any untyped constant operand.
                        Some(TokKind::Int(width, val)) => {
                            let ops = self.parse_ops();
                            //error checking on ops length
                            assert!(
                                ops.len() == 1,
                                "expected one operand for infer \
                                    instruction, but found {}",
                                ops.len()
                            );
                            self.create_inst(InstKind::ResultInst, "result", width, ops)
                        }
                        _ => {
                            panic!("unexpected result instruction operand");
                        }
                    }
                }
                _ => {
                    panic!("unexpected identifier instruction kind");
                }
            }
        } else {
            panic!("unexpected identifier instruction");
        }
    }

    // parse dummy inst for implies symbol
    fn parse_implies_dummy_inst(&mut self) -> Inst<'a> {
        self.consume_token();
        Inst {
            kind: InstKind::Implies,
            lhs: "",
            width: 0,
            var_number: Some(0),
            ops: None,
        }
    }

    // parse each instruction
    fn parse_inst(&mut self) -> Inst<'a> {
        // Instructions start either with valname or Ident
        // Example:
        // %1:i32 = ....
        // cand ... , infer ... , result ...
        // pc ... , blockpc ... ,

        match self.lookahead {
            Some(TokKind::ValName(lhs, width)) => {
                self.lhs_valname = lhs;
                self.width = width;
                self.parse_valname_inst()
            }
            Some(TokKind::Ident(text)) => self.parse_ident_inst(),
            Some(TokKind::Implies) => self.parse_implies_dummy_inst(),
            _ => {
                // FIXME: Jubi: Build an error
                panic!(
                    "Error: Either instruction \
                    should start with valname or identifier"
                );
            }
        }
    }
}

pub fn parse(text: &str) -> Vec<Inst> {
    let mut p = Parser::new(text);

    p.consume_token();

    // FIXMEL we want a ret value from parse_line() to
    // be used later for code gen purpose

    let mut insts: Vec<Inst> = Vec::new();
    loop {
        match p.lookahead {
            Some(TokKind::Eof) => break,
            _ => {
                let inst = p.parse_inst();
                match inst.kind {
                    InstKind::Implies => {
                        continue;
                    }
                    _ => {
                        let LHS = inst.lhs;
                        insts.push(inst);
                        // create hashmap and keep
                        // inserting valnames + index pair
                        p.lhsValNames_to_Idx.insert(LHS, insts.len() - 1);
                        // Debug
                        // println!("Inserting into hashMap in parser====\n");
                        // println!("Inst = {}\n",
                        //     p.get_kind_name(inst.kind.clone()));
                        // println!("LHS = {} : Idx = {}\n",
                        //     LHS, insts.len()-1);
                    }
                }
            }
        }
    }

    // Debug
    // println!("\n******* Debugging the hashtable for \
    //     LHS name to Index ***\n");
    // for (key, val) in p.lhsValNames_to_Idx {
    //     println!("LHS = {}, Idx = {}\n", key, val);
    // }
    // println!("\n*******************\n");

    insts
}
