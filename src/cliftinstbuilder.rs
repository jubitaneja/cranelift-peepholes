// Codegen: Cranelift instruction builder
// This phase of codegen will simply build cranelift instructions
// from souper instructions

use parser::{self, Parser, Inst, InstKind, SouperOperand};

#[derive(Clone)]
pub struct CtonInst {
    pub valuedef: CtonValueDef,
    pub kind: CtonInstKind,
    pub opcode: CtonOpcode,
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
        CtonInstKind::Var => "Var".to_string(),
        _ => "".to_string(),
    }
}

pub fn get_clift_opcode_name<'a>(opcode: CtonOpcode) -> String {
    match opcode {
        CtonOpcode::Iadd => "Iadd".to_string(),
        CtonOpcode::Imul => "Imul".to_string(),
        CtonOpcode::Isub => "Isub".to_string(),
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
        Inst{kind, lhs, ops} => {
            match kind {
                InstKind::Add => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Iadd,
                        // FIXME: Deal with ops mapping in a better way later
                        // because, we have to get rid of souperoperand type completely
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Mul => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Imul,
                        // FIXME: Deal with ops mapping in a better way later
                        // because, we have to get rid of souperoperand type completely
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Sub => {
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Isub,
                        // FIXME: Deal with ops mapping in a better way later
                        // because, we have to get rid of souperoperand type completely
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Var => {
                    CtonInst {
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        cops: build_clift_ops(ops),
                    }
                },
                InstKind::Infer => {
                    CtonInst {
                        valuedef: CtonValueDef::NoneType,
                        kind: CtonInstKind::NoneType,
                        opcode: CtonOpcode::Infer,
                        cops: build_clift_ops(ops),
                    }
                },
                _ => {
                    CtonInst {
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        cops: build_clift_ops(ops),
                    }
                },
            }
        },
        _ => {
            CtonInst{
                valuedef: CtonValueDef::Param,
                kind: CtonInstKind::Var,
                opcode: CtonOpcode::Var,
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
