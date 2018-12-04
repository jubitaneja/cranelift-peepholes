// Codegen: Cranelift instruction builder
// This phase of codegen will simply build cranelift instructions
// from souper instructions

use parser::{self, Parser, Inst, InstKind, SouperOperand};

#[derive(Clone)]
pub struct CtonInst {
    pub valuedef: CtonValueDef,
    pub kind: CtonInstKind,
    pub opcode: CtonOpcode,
    pub width: u32,
    pub var_num: Option<u32>,
    pub cops: Option<Vec<CtonOperand>>,
}

#[derive(Clone)]
pub enum CtonValueDef {
    Result,
    Param,
    NoneType, //added to deal with infer inst in souper IR
}

#[derive(Clone)]
pub enum CtonInstKind {
    Unary,
    UnaryImm,
    Binary,
    BinaryImm,
    Var,
    NoneType, //added for infer inst in souper IR
}

#[derive(Clone)]
pub enum CtonOpcode {
    Iadd,
    IaddImm,
    Var,
    Imul,
    Isub,
    Eq,
    Ne,
    Slt,
    Ult,
    Sle,
    Ule,
    Band,
    Bor,
    Bxor,
    Ishl,
    Sshr,
    Ushr,
    Popcnt,
    Clz,
    Ctz,
    Infer,
}

#[derive(Clone)]
pub enum CtonOpType {
    Index,
    Constant,
}

#[derive(Clone)]
pub struct CtonOperand {
    pub idx_val: Option<usize>,
    pub const_val: Option<i64>,
}

/// Helper functions

/// Returns the cretonne instruction names for the given cretonne opcode
pub fn get_cton_inst_name(opcode: CtonOpcode) {
    match opcode {
        CtonOpcode::Iadd => println!("CtonOpcode = Iadd"),
        CtonOpcode::Imul => println!("CtonOpcode = Imul"),
        CtonOpcode::Isub => println!("CtonOpcode = Isub"),
        CtonOpcode::Eq => println!("CtonOpcode = Eq"),
        CtonOpcode::Ne => println!("CtonOpcode = Ne"),
        CtonOpcode::Ult => println!("CtonOpcode = Ult"),
        CtonOpcode::Slt => println!("CtonOpcode = Slt"),
        CtonOpcode::Ule => println!("CtonOpcode = Ule"),
        CtonOpcode::Sle => println!("CtonOpcode = Sle"),
        CtonOpcode::Band => println!("CtonOpcode = Band"),
        CtonOpcode::Bor => println!("CtonOpcode = Bor"),
        CtonOpcode::Bxor => println!("CtonOpcode = Bxor"),
        CtonOpcode::Ishl => println!("CtonOpcode = Ishl"),
        CtonOpcode::Sshr => println!("CtonOpcode = Sshr"),
        CtonOpcode::Ushr => println!("CtonOpcode = Ushr"),
        CtonOpcode::Popcnt => println!("CtonOpcode = Popcnt"),
        CtonOpcode::Clz => println!("CtonOpcode = Clz"),
        CtonOpcode::Ctz => println!("CtonOpcode = Ctz"),
        CtonOpcode::IaddImm => println!("CtonOpcode = IaddImm"),
        CtonOpcode::Var => println!("CtonOpcode = Var"),
        _ => {
            println!("CtonOpcode not yet handled");
        },
    }
}

pub fn getCtonOpCodeName(opcode: CtonOpcode) {
    match opcode {
        CtonOpcode::Iadd => println!("Cton::Opcode = Iadd"),
        CtonOpcode::Imul => println!("Cton::Opcode = Imul"),
        CtonOpcode::Isub => println!("Cton::Opcode = Isub"),
        CtonOpcode::Eq => println!("Cton::Opcode = Eq"),
        CtonOpcode::Ne => println!("Cton::Opcode = Ne"),
        CtonOpcode::Ult => println!("Cton::Opcode = Ult"),
        CtonOpcode::Slt => println!("Cton::Opcode = Slt"),
        CtonOpcode::Ule => println!("Cton::Opcode = Ule"),
        CtonOpcode::Sle => println!("Cton::Opcode = Sle"),
        CtonOpcode::Band => println!("Cton::Opcode = Band"),
        CtonOpcode::Bor => println!("Cton::Opcode = Bor"),
        CtonOpcode::Bxor => println!("Cton::Opcode = Bxor"),
        CtonOpcode::Ishl => println!("Cton::Opcode = Ishl"),
        CtonOpcode::Sshr => println!("Cton::Opcode = Sshr"),
        CtonOpcode::Ushr => println!("Cton::Opcode = Ushr"),
        CtonOpcode::Popcnt => println!("Cton::Opcode = Popcnt"),
        CtonOpcode::Clz => println!("Cton::Opcode = Clz"),
        CtonOpcode::Ctz => println!("Cton::Opcode = Ctz"),
        CtonOpcode::Var => println!("Cton::Opcode = Var"),
        CtonOpcode::Infer => println!("Cton::Opcode = Infer"),
        _ => println!("Cton: other type yet to be handled"),
    }
}

pub fn get_clift_valdef_name(vdef: CtonValueDef) -> String {
    match vdef {
        CtonValueDef::Result => "Result".to_string(),
        CtonValueDef::Param => "Param".to_string(),
        CtonValueDef::NoneType => "None".to_string(),
        _ => "".to_string(),
    }
}

pub fn get_clift_instdata_name(instdata: CtonInstKind) -> String {
    match instdata {
        CtonInstKind::Binary => "Binary".to_string(),
        CtonInstKind::Unary => "Unary".to_string(),
        CtonInstKind::Var => "Var".to_string(),
        _ => "".to_string(),
    }
}

pub fn get_clift_opcode_name<'a>(opcode: CtonOpcode) -> String {
    match opcode {
        CtonOpcode::Iadd => "Iadd".to_string(),
        CtonOpcode::Imul => "Imul".to_string(),
        CtonOpcode::Isub => "Isub".to_string(),
        CtonOpcode::Eq => "icmpeq".to_string(),
        CtonOpcode::Ne => "icmpne".to_string(),
        CtonOpcode::Slt => "icmpslt".to_string(),
        CtonOpcode::Ult => "icmpult".to_string(),
        CtonOpcode::Sle => "icmpsle".to_string(),
        CtonOpcode::Ule => "icmpule".to_string(),
        CtonOpcode::Band => "band".to_string(),
        CtonOpcode::Bor => "bor".to_string(),
        CtonOpcode::Bxor => "bxor".to_string(),
        CtonOpcode::Ishl => "ishl".to_string(),
        CtonOpcode::Sshr => "sshr".to_string(),
        CtonOpcode::Ushr => "ushr".to_string(),
        CtonOpcode::Popcnt => "popcnt".to_string(),
        CtonOpcode::Clz => "clz".to_string(),
        CtonOpcode::Ctz => "ctz".to_string(),
        CtonOpcode::IaddImm => "IaddImm".to_string(),
        CtonOpcode::Var => "Var".to_string(),
        CtonOpcode::Infer => "Infer".to_string(),
        _ => "".to_string(),
    }
}

pub fn build_clift_ops(souper_ops: Option<Vec<SouperOperand>>) -> Option<Vec<CtonOperand>> {
    let mut cton_ops: Vec<CtonOperand> = Vec::new();
    match souper_ops {
        Some(souper_ops) => {
            for souper_op in souper_ops {
                cton_ops.push(CtonOperand {
                    idx_val: souper_op.idx_val,
                    const_val: souper_op.const_val,
                });
            }
            Some(cton_ops)
        },
        None => {
            None
        }
    }
}

/// Codegen Phase #1
pub fn mapping_souper_to_cton_isa(souper_inst: Inst) -> CtonInst {
    match souper_inst {
        Inst{kind, lhs, width, var_number, ops} => {
            match kind {
                // FIXME: Deal with ops mapping in a better way later
                // because, we have to get rid of souperoperand type completely
                InstKind::Add => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Iadd,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Mul => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Imul,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Sub => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Isub,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Eq => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Eq,
                        width: 1,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Ne => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Ne,
                        width: 1,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Slt => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Slt,
                        width: 1,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Ult => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Ult,
                        width: 1,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Sle => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Sle,
                        width: 1,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Ule => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Ule,
                        width: 1,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::And => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Band,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Or => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Bor,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Xor => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Bxor,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Shl => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Ishl,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Lshr => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Ushr,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Ashr => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Sshr,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Ctpop => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Unary,
                        opcode: CtonOpcode::Popcnt,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Ctlz => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Unary,
                        opcode: CtonOpcode::Clz,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Cttz => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Unary,
                        opcode: CtonOpcode::Ctz,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Var => {
                    CtonInst {
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Infer => {
                    CtonInst {
                        valuedef: CtonValueDef::NoneType,
                        kind: CtonInstKind::NoneType,
                        opcode: CtonOpcode::Infer,
                        width: width,
                        var_num: var_number,
                        cops: build_clift_ops(ops),
                    }
                },
                _ => {
                    CtonInst {
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        width: width,
                        var_num: None,
                        cops: build_clift_ops(ops),
                    }
                },
            }
        },
        _ => {
            CtonInst {
                valuedef: CtonValueDef::Param,
                kind: CtonInstKind::Var,
                opcode: CtonOpcode::Var,
                width: 0,
                var_num: None,
                cops: None,
            }
        },
    }
}

pub fn transform_souper_to_clift_insts(souper_insts: Vec<Inst>) -> Vec<CtonInst> {
    let mut cton_insts: Vec<CtonInst> = Vec::new();
    for souper_inst in souper_insts {
        // get the mapping souper ISA to cretonne ISA
        // And, insert each cton inst to a new vec<cton_inst>
        // add more details to cton inst structure:
        // name, binary/unary instruction data, 
        let cton_inst = mapping_souper_to_cton_isa(souper_inst);
        cton_insts.push(cton_inst);
    }
    cton_insts
}
