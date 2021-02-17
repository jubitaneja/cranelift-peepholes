// Parser for souper tokens

use lexer::{self, Lexer, LocatedError, LocatedToken, Location, TokKind};
use std::collections::HashMap;

#[derive(Clone)]
#[allow(dead_code)]
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

#[derive(Clone)]
pub struct SouperOperand {
    pub kind: SouperOpType,
    // what should be the type of constants values?
    pub idx_val: Option<usize>,
    pub const_val: Option<i128>, //FIXME: fix this width of const value to i64 or something else?
    pub width: u32,
}

#[derive(Clone)]
pub struct Inst {
    pub kind: InstKind,
    pub lhs: String,
    pub lhs_idx: usize,
    pub width: u32,
    pub var_number: Option<u32>,
    pub ops: Option<Vec<SouperOperand>>,
}

#[derive(Clone)]
pub struct OpsInfo {
    pub both_index: bool,
    pub const_index: usize,
}

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,

    /// Current lookahead token.
    lookahead: Option<TokKind>,

    /// Location of lookahead.
    loc: Location,

    lex_error: Option<lexer::Error>,

    /// LHS Valname
    lhs_valname: String,

    /// width
    width: u32,

    // inst count
    var_count: u32,

    // hash map of LHS valnames to Index values
    lhs_val_names_to_idx: HashMap<String, usize>,

    // track count of constants, whenever we create it
    // as an individual instruction
    const_count: u32,

    // total instructions
    total_insts: usize,
}

impl<'a> Parser<'a> {
    // Initialize the parser.
    pub fn new(s: &str) -> Parser {
        Parser {
            lex: Lexer::new(s),
            lookahead: None,
            loc: Location { line_num: 0 },
            lex_error: None,
            lhs_valname: String::from(""),
            width: 0,
            var_count: 0,
            lhs_val_names_to_idx: HashMap::new(),
            const_count: 0,
            total_insts: 0,
        }
    }

    fn create_var(
        &mut self,
        instkind: InstKind,
        instname: String,
        instwidth: u32
    ) -> Inst {
        // return the inst struct with details
        // FIXME: add more details later if required
        self.var_count += 1;
        Inst {
            kind: instkind,
            lhs: instname,
            lhs_idx: 0,
            width: instwidth,
            var_number: Some(self.var_count),
            ops: None,
        }
    }

    fn create_inst(
        &mut self,
        instkind: InstKind,
        instname: String,
        instwidth: u32,
        ops: Vec<SouperOperand>,
    ) -> Inst {
        // return the inst struct with details
        // FIXME: add more details later if required
        // Add Ops details too here: Major TODO
        Inst {
            kind: instkind,
            lhs: instname,
            lhs_idx: 0,
            width: instwidth,
            var_number: None,
            ops: Some(ops),
        }
    }

    // return inst Kind for the given inst names
    fn get_inst_kind(&mut self, name: String) -> InstKind {
        match name.as_ref() {
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
    #[allow(dead_code)]
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
            InstKind::Const => "const".to_string(),
            _ => "Inst Kind name is not yet handled in function: get_kind_name()".to_string(),
        }
    }

    // print token name
    #[allow(dead_code)]
    fn get_token_name(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName(..)) => println!("ValName "),
            Some(TokKind::Ident(..)) => println!("Ident "),
            Some(TokKind::Comma) => println!("Comma "),
            Some(TokKind::Equal) => println!("Eq "),
            Some(TokKind::Int(..)) => println!("Int "),
            Some(TokKind::Eof) => println!("EOF "),
            Some(TokKind::Error) => println!("Error "),
            Some(TokKind::UntypedInt) => println!("Untypedint "),
            _ => println!("Token type not handled "),
        }
    }

    // returns the current token
    fn consume_token(&mut self) -> Option<TokKind> {
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
                    error, location, ..
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

    #[allow(dead_code)]
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
        match self.lookahead.clone() {
            Some(TokKind::ValName(lhs, width)) => {
                let mut value = None;
                for (key, val) in &self.lhs_val_names_to_idx {
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
            Some(TokKind::Int(width, const_val)) => {
                // get the value of const
                // build const inst
                // Inst I = IC.getConst()
                self.consume_token();

                SouperOperand {
                    kind: SouperOpType::Constant,
                    idx_val: None,
                    const_val: Some(const_val),
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

    fn create_const_lhs(&mut self) -> String {
        self.const_count += 1;
        let mut lhs = String::from("%const");
        lhs.push_str(&self.const_count.to_string());
        lhs
    }

    fn create_const_inst(
        &mut self,
        op: SouperOperand,
        width: u32
    ) -> Inst {
        let mut const_ops = vec![];
        const_ops.push(op);
        Inst {
            kind: InstKind::Const,
            lhs: self.create_const_lhs(),
            lhs_idx: 0,
            width: width,
            var_number: None,
            ops: Some(const_ops),
        }
    }

    fn create_const_inst_sequence(
        &mut self,
        kind: InstKind,
        lhs: String,
        width: u32,
        ops: Vec<SouperOperand>
    ) -> Vec<Inst> {
        let mut insts = vec![];
        // create const inst for first operand
        let const_inst0 = self.create_const_inst(ops[0].clone(), width);
        let const_idx0 = self.total_insts;
        self.lhs_val_names_to_idx.insert(const_inst0.lhs.clone(), const_idx0);
        self.total_insts += 1;
        insts.push(const_inst0);

        // create const inst for second operand
        let const_inst1 = self.create_const_inst(ops[1].clone(), width);
        let const_idx1 = self.total_insts;
        self.lhs_val_names_to_idx.insert(const_inst1.lhs.clone(), const_idx1);
        self.total_insts += 1;
        insts.push(const_inst1);

        // create final inst using above two as ops
        let mut inst_ops = vec![];
        inst_ops.push(SouperOperand {
            kind: SouperOpType::Index,
            idx_val: Some(const_idx0),
            const_val: None,
            width: width,
        });
        inst_ops.push(SouperOperand {
            kind: SouperOpType::Index,
            idx_val: Some(const_idx1),
            const_val: None,
            width: width,
        });
        insts.push(Inst {
            kind: kind,
            lhs: lhs,
            lhs_idx: 0,
            width: width,
            var_number: None,
            ops: Some(inst_ops),
        });

        insts
    }

    fn both_ops_const(&mut self, ops: Vec<SouperOperand>) -> bool {
        let mut op_type = true;
        for op in ops {
            match op.kind {
                SouperOpType::Index => {
                    op_type = false;
                    break
                },
                _ => {},
            }
            //if let op.kind = SouperOpType::Index {
            //    op_type = false;
            //    break
            //}
        }
        op_type
    }

    fn both_ops_index(&mut self, ops: Vec<SouperOperand>) -> OpsInfo {
        let mut op_type = true;
        let mut idx = 0;
        for op in 0..ops.len() {
            match ops[op].kind {
                SouperOpType::Constant => {
                    op_type = false;
                    idx = op;
                    break
                },
                _ => {},
            }
        }
        OpsInfo {
            both_index: op_type,
            const_index: idx,
        }
    }

    fn parse_inst_types(&mut self) -> Vec<Inst> {
        if let Some(TokKind::Ident(text)) = self.lookahead.clone() {
            match self.get_inst_kind(text.clone()) {
                InstKind::Var => {
                    // TODO: error checking
                    // instwidth == 0 => error "var inst expects atleast width=1"

                    self.consume_token();
                    let instname = self.lhs_valname.clone();
                    let instwidth = self.width.clone();
                    let mut var_inst = vec![];
                    var_inst.push(self.create_var(InstKind::Var, instname, instwidth));
                    var_inst
                }
                _ => {
                    let inst_kind = self.get_inst_kind(text.clone());
                    self.consume_token();
                    let ops = self.parse_ops();
                    let instname = self.lhs_valname.clone();
                    let instwidth = self.width.clone();

                    if self.both_ops_const(ops.clone()) {
                        self.create_const_inst_sequence(
                            inst_kind,
                            instname,
                            instwidth,
                            ops)
                    } else {
                        // FIXME: check if any op is constant and get its
                        // op-index i.e. is it op0 or op1
                        let ops_info = self.both_ops_index(ops.clone());
                        let mut insts = vec![];
                        if ops_info.both_index {
                            insts.push(
                                self.create_inst(
                                    inst_kind,
                                    instname,
                                    instwidth,
                                    ops
                                )
                            );
                        } else {
                            let mut ordered_ops = vec![];
                            // In this we are sorting operands to match semantics
                            // of cranelift IR i.e. inst_imm X, c
                            // Subtraction c -x is represented as irsub_imm X, c
                            // However, X -c is transformed to add_imm X, -c
                            if ops_info.const_index == 0 {
                                match inst_kind.clone() {
                                    InstKind::Add | InstKind::Mul |
                                    InstKind::And | InstKind::Or |
                                    InstKind:: Xor | InstKind::Eq |
                                    InstKind::Ne | InstKind::Slt |
                                    InstKind::Ult | InstKind::Sle |
                                    InstKind::Ule | InstKind::Sub => {
                                        ordered_ops.push(ops[1].clone());
                                        ordered_ops.push(ops[0].clone());
                                        insts.push(
                                            self.create_inst(
                                                inst_kind,
                                                instname,
                                                instwidth,
                                                ordered_ops
                                            )
                                        );
                                    },
                                    _ => {
                                        insts.push(
                                            self.create_inst(
                                                inst_kind,
                                                instname,
                                                instwidth,
                                                ops
                                            )
                                        );
                                    },
                                }
                            } else if ops_info.const_index == 1 {
                                match inst_kind.clone() {
                                    InstKind::Sub => {
                                        ordered_ops.push(ops[0].clone());
                                        let const_op = ops[1].clone();
                                        let new_const;
                                        match const_op.const_val.clone() {
                                            Some(c) => {
                                                new_const = -1 * c;
                                                ordered_ops.push(
                                                    SouperOperand {
                                                        kind: SouperOpType::Constant,
                                                        idx_val: None,
                                                        const_val: Some(new_const),
                                                        width: instwidth,
                                                    }
                                                );
                                                insts.push(
                                                    self.create_inst(
                                                        InstKind::Add,
                                                        instname,
                                                        instwidth,
                                                        ordered_ops
                                                    )
                                                );
                                            },
                                            _ => {
                                                insts.push(
                                                    self.create_inst(
                                                        inst_kind,
                                                        instname,
                                                        instwidth,
                                                        ops
                                                    )
                                                );
                                            },
                                        }
                                    },
                                    _ => {
                                        insts.push(
                                            self.create_inst(
                                                inst_kind,
                                                instname,
                                                instwidth,
                                                ops
                                            )
                                        );
                                    },
                                }
                            }
                        }
                        // OpInfo {
                        //    result: bool => are_both_ops_index() - if false, go ahead and check further
                        //    op-index: usize -- save index of constant operand
                        // } = self.both_ops_index(ops.clone());
                        // if OpInfo.result is false {
                        //     if op-index is 0 {
                        //         match inst_kind {
                        //             Add || Mul || And || Or || Xor => {
                        //                 ordered_ops.push(ops[1]) in list first
                        //                 ordered_ops.push(ops[0]) later becuase we want 'imm' operand in the end
                        //             },
                        //             Sub => {
                        //                 ordered_ops.push(ops[1]) in list first
                        //                 ordered_ops.push(ops[0]) later becuase we want 'imm' operand in the end
                        //             },
                        //             _ => {},
                        //         }
                        //     } else if op-index is 1 && inst_kind is Sub {
                        //         ordered_ops.push(ops[0]) in list
                        //         ordered_ops.push(negative value of constant op i..e ops[1])
                        //         modify inst kind to "ADD" now sub x, c ==> add x, -c
                        //     }

                        // }
                        insts
                    }
                }
            }
        } else {
            panic!("Error: fn: parse_inst_types()");
        }
    }

    fn parse_valname_inst(&mut self) -> Vec<Inst> {
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
                    Some(TokKind::Ident(..)) => self.parse_inst_types(),
                    _ => panic!("Error: Expected a valid Identifier after ValName -> Eq token"),
                }
            }
            _ => panic!("Error: Expected Eq token followed by Valname token"),
        }
    }

    // parse instructions that start with an identifier
    // example: infer, cand, pc, blockpc inst in Souper IR
    fn parse_ident_inst(&mut self) -> Vec<Inst> {
        if let Some(TokKind::Ident(text)) = self.lookahead.clone() {
            let mut insts = vec![];
            match self.get_inst_kind(text.clone()) {
                InstKind::Infer => {
                    self.consume_token();
                    match self.lookahead.clone() {
                        Some(TokKind::ValName(_lhs, width)) => {
                            let ops = self.parse_ops();
                            assert!(
                                ops.len() == 1,
                                "expected one operand for infer instruction, but found {}",
                                ops.len()
                            );
                            insts.push(
                                self.create_inst(
                                    InstKind::Infer,
                                    String::from("infer"),
                                    width,
                                    ops
                                )
                            );
                            insts
                        }
                        _ => {
                            panic!("unexpected infer instruction operand");
                        }
                    }
                }
                InstKind::ResultInst => {
                    self.consume_token();
                    match self.lookahead.clone() {
                        Some(TokKind::ValName(_lhs, width)) => {
                            let ops = self.parse_ops();
                            //error checking on ops length
                            assert!(
                                ops.len() == 1,
                                "expected one operand for infer instruction, but found {}",
                                ops.len()
                            );
                            insts.push(
                                self.create_inst(
                                    InstKind::ResultInst,
                                    String::from("result"),
                                    width,
                                    ops
                                )
                            );
                            insts
                        }
                        // Result inst can have a typed int as an operand as well.
                        // We will make it a rule that Souper's result inst *DOES NOT*
                        // have any untyped constant operand.
                        Some(TokKind::Int(width, _val)) => {
                            let ops = self.parse_ops();
                            //error checking on ops length
                            assert!(
                                ops.len() == 1,
                                "expected one operand for infer \
                                    instruction, but found {}",
                                ops.len()
                            );
                            insts.push(
                                self.create_inst(
                                    InstKind::ResultInst,
                                    String::from("result"),
                                    width,
                                    ops
                                )
                            );
                            insts
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
    fn parse_implies_dummy_inst(&mut self) -> Inst {
        self.consume_token();
        Inst {
            kind: InstKind::Implies,
            lhs: String::from(""),
            lhs_idx: 0,
            width: 0,
            var_number: Some(0),
            ops: None,
        }
    }

    // parse each instruction
    fn parse_inst(&mut self) -> Vec<Inst> {
        // Instructions start either with valname or Ident
        // Example:
        // %1:i32 = ....
        // cand ... , infer ... , result ...
        // pc ... , blockpc ... ,

        match self.lookahead.clone() {
            Some(TokKind::ValName(lhs, width)) => {
                self.lhs_valname = lhs;
                self.width = width;
                self.parse_valname_inst()
            }
            Some(TokKind::Ident(_)) => self.parse_ident_inst(),
            Some(TokKind::Implies) => {
                let mut insts = vec![];
                insts.push(self.parse_implies_dummy_inst());
                insts
            },
            _ => {
                // FIXME: Jubi: Build an error
                panic!(
                    "Error: Either instruction \
                    should start with valname or identifier"
                );
            }
        }
    }

    // set lhs_idx of each inst
    fn get_updated_inst_with_lhs_idx_num(
        &mut self,
        instruction: Inst,
        idx: usize) -> Inst {
            Inst {
                kind: instruction.kind,
                lhs: instruction.lhs,
                lhs_idx: idx,
                width: instruction.width,
                var_number: instruction.var_number,
                ops: instruction.ops,
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
                let parsed_insts = p.parse_inst();
                for inst in parsed_insts {
                match inst.kind {
                    InstKind::Implies => {
                        continue;
                    }
                    _ => {
                        let lhs = inst.lhs.clone();
                        insts.push(inst);
                        // create hashmap and keep
                        // inserting valnames + index pair
                        // FIXME: TODO: If key exists in hashmap, then don't insert 
                        if p.lhs_val_names_to_idx.get(&lhs).is_none() {
                            p.lhs_val_names_to_idx.insert(lhs, p.total_insts);
                            // Debug
                            // println!("Inserting into hashMap in parser====\n");
                            // println!("LHS = {} : Idx = {}\n",
                            //     lhs, p.total_insts);

                            p.total_insts += 1;
                        }
                    }
                }
                }
            }
        }
    }
    
    let mut updated_insts: Vec<Inst> = Vec::new();
    // set the lhs_idx number for each instruction
    // get this info from the hashtable that stores LHS_name : Idx_num
    for i in insts.clone() {
        updated_insts.push(
            p.get_updated_inst_with_lhs_idx_num(
                i.clone(),
                p.lhs_val_names_to_idx[&i.lhs.clone()]));
    }

    // Debug
    //////println!("Parsed Souper Instructions:\n");
    //////for i in updated_insts.clone() {
    //////    println!("Inst = {}\n", p.get_kind_name(i.kind));
    //////    println!("\t LHS = {}\n", i.lhs);
    //////    println!("\t\tLHS index num = {}", i.lhs_idx);
    //////    match i.ops {
    //////        Some(ops_lst) => {
    //////            for op in ops_lst {
    //////                //
    //////                match op.idx_val {
    //////                    Some(id) => {
    //////                        println!("\t op: idx_val = {}\n", id);
    //////                    },
    //////                    None => {},
    //////                }
    //////                match op.const_val {
    //////                    Some(c) => {
    //////                        println!("\t op: const_val = {}\n", c);
    //////                    },
    //////                    None => {},
    //////                }
    //////            }
    //////        },
    //////        None => {},
    //////    }
    //////}
    //////println!("\n******* Parser: Debugging the hashtable for \
    //////    LHS name to Index ***\n");
    //////for (key, val) in p.lhs_val_names_to_idx {
    //////    println!("LHS = {}, Idx = {}\n", key, val);
    //////}
    //////println!("\n*******************\n");

    updated_insts
}
