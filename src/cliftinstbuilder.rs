// Codegen: Cranelift instruction builder
// This phase of codegen will simply build cranelift instructions
// from souper instructions

use parser::{self, Parser, Inst, InstKind, SouperOperand};

#[derive(Clone)]
pub struct CtonInst {
    pub valuedef: CtonValueDef,
    pub kind: CtonInstKind,
    pub opcode: CtonOpcode,
    //FIXME: We have to get rid of Souper's structs here!
    //pub cops: Option<Vec<SouperOperand>>,
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
    Infer,
}

/// Helper functions

/// Returns the cretonne instruction names for the given cretonne opcode
pub fn get_cton_inst_name(opcode: CtonOpcode) {
    match opcode {
        CtonOpcode::Iadd => println!("CtonOpcode = Iadd"),
        CtonOpcode::IaddImm => println!("CtonOpcode = IaddImm"),
        CtonOpcode::Var => println!("CtonOpcode = Var"),
        _ => {
            println!("CtonOpcode not yet handled");
        },
    }
}

pub fn getCtonOpCodeName(opcode: CtonOpcode) {
    match opcode {
        CtonOpcode::Iadd => println!("Cton::Opcode =Iadd"),
        CtonOpcode::Var => println!("Cton::Opcode = Var"),
        _ => println!("Cton: other type yet to be handled"),
    }
}

pub fn getCtonValDefName(vdef: CtonValueDef) {
    match vdef {
        CtonValueDef::Result => println!("Cton::ValDef = Result"),
        CtonValueDef::Param => println!("Cton::ValDef =  Param"),
        _ => println!("Cton: No such value def types"),
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
                        //cops: ops,
                    }
                },
                InstKind::Var => {
                    CtonInst {
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        //cops: ops,
                    }
                },
                InstKind::Infer => {
                    CtonInst {
                        valuedef: CtonValueDef::NoneType,
                        kind: CtonInstKind::NoneType,
                        opcode: CtonOpcode::Infer,
                        //cops = ops,
                    }
                },
                _ => {
                    CtonInst {
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        //cops: ops,
                    }
                },
            }
        },
        _ => {
            CtonInst{
                valuedef: CtonValueDef::Param,
                kind: CtonInstKind::Var,
                opcode: CtonOpcode::Var,
                //cops: None,
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
