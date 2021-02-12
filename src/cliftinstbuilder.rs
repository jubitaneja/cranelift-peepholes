// Codegen: Cranelift instruction builder
// This phase of codegen will simply build cranelift instructions
// from souper instructions

use parser::{Inst, InstKind, SouperOperand};

#[derive(Clone)]
pub struct CtonInst {
    pub valuedef: CtonValueDef,
    pub kind: CtonInstKind,
    pub opcode: CtonOpcode,
    pub cond: Option<CtonCmpCond>,
    pub width: u32,
    pub var_num: Option<u32>,
    pub cops: Option<Vec<CtonOperand>>,
    pub lhs_index: usize,
}

#[derive(Clone)]
pub enum CtonValueDef {
    Result,
    Param,
    NoneType, //added to deal with infer inst in souper IR
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum CtonInstKind {
    Unary,
    UnaryImm,
    Binary,
    BinaryImm,
    Var,
    IntCompare,
    IntCompareImm,
    NoneType, //added for infer inst in souper IR
}

#[derive(Clone)]
pub enum CtonOpcode {
    Iconst,
    Iadd,
    IaddImm,
    Var,
    Imul,
    ImulImm,
    Isub,
    IsubImm,
    Band,
    BandImm,
    Bor,
    BorImm,
    Bxor,
    BxorImm,
    Ishl,
    IshlImm,
    Sshr,
    SshrImm,
    Ushr,
    UshrImm,
    Popcnt,
    Clz,
    Ctz,
    Icmp,
    IcmpImm,
    Infer,
    ResultInst,
    NoneType,
}

/// Types of conditions for cranelift icmp inst
/// We are ignoring 'gretar-than' cond, because
/// Souper always generates 'less-than' opcodes.
#[derive(Clone)]
pub enum CtonCmpCond {
    Eq,
    Ne,
    Slt,
    Ult,
    Sle,
    Ule,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum CtonOpType {
    Index,
    Constant,
}

#[derive(Clone)]
pub struct CtonOperand {
    pub idx_val: Option<usize>,
    pub const_val: Option<i128>, //FIXME: maybe set constant operand width to i64?
}

/// Helper functions

/// Returns the cretonne instruction names for the given cretonne opcode
#[allow(dead_code)]
pub fn get_cton_inst_name(opcode: CtonOpcode) {
    match opcode {
        CtonOpcode::Iconst => println!("CtonOpcode = Iconst"),
        CtonOpcode::Iadd => println!("CtonOpcode = Iadd"),
        CtonOpcode::Imul => println!("CtonOpcode = Imul"),
        CtonOpcode::ImulImm => println!("CtonOpcode = ImulImm"),
        CtonOpcode::Isub => println!("CtonOpcode = Isub"),
        CtonOpcode::IsubImm => println!("CtonOpcode = IsubImm"),
        CtonOpcode::Band => println!("CtonOpcode = Band"),
        CtonOpcode::BandImm => println!("CtonOpcode = BandImm"),
        CtonOpcode::Bor => println!("CtonOpcode = Bor"),
        CtonOpcode::BorImm => println!("CtonOpcode = BorImm"),
        CtonOpcode::Bxor => println!("CtonOpcode = Bxor"),
        CtonOpcode::BxorImm => println!("CtonOpcode = BxorImm"),
        CtonOpcode::Ishl => println!("CtonOpcode = Ishl"),
        CtonOpcode::IshlImm => println!("CtonOpcode = IshlImm"),
        CtonOpcode::Sshr => println!("CtonOpcode = Sshr"),
        CtonOpcode::SshrImm => println!("CtonOpcode = SshrImm"),
        CtonOpcode::Ushr => println!("CtonOpcode = Ushr"),
        CtonOpcode::UshrImm => println!("CtonOpcode = UshrImm"),
        CtonOpcode::Popcnt => println!("CtonOpcode = Popcnt"),
        CtonOpcode::Clz => println!("CtonOpcode = Clz"),
        CtonOpcode::Ctz => println!("CtonOpcode = Ctz"),
        CtonOpcode::IaddImm => println!("CtonOpcode = IaddImm"),
        CtonOpcode::Var => println!("CtonOpcode = Var"),
        CtonOpcode::Icmp => println!("CtonOpcode = Icmp"),
        CtonOpcode::IcmpImm => println!("CtonOpcode = IcmpImm"),
        _ => {
            println!("CtonOpcode not yet handled");
        }
    }
}

#[allow(dead_code)]
pub fn get_cton_opcode_name(opcode: CtonOpcode) {
    match opcode {
        CtonOpcode::Iconst => println!("Cton::Opcode = Iconst"),
        CtonOpcode::Iadd => println!("Cton::Opcode = Iadd"),
        CtonOpcode::Imul => println!("Cton::Opcode = Imul"),
        CtonOpcode::ImulImm => println!("Cton::Opcode = ImulImm"),
        CtonOpcode::Isub => println!("Cton::Opcode = Isub"),
        CtonOpcode::IsubImm => println!("Cton::Opcode = IsubImm"),
        CtonOpcode::Band => println!("Cton::Opcode = Band"),
        CtonOpcode::BandImm => println!("Cton::Opcode = BandImm"),
        CtonOpcode::Bor => println!("Cton::Opcode = Bor"),
        CtonOpcode::BorImm => println!("Cton::Opcode = BorImm"),
        CtonOpcode::Bxor => println!("Cton::Opcode = Bxor"),
        CtonOpcode::BxorImm => println!("Cton::Opcode = BxorImm"),
        CtonOpcode::Ishl => println!("Cton::Opcode = Ishl"),
        CtonOpcode::IshlImm => println!("Cton::Opcode = IshlImm"),
        CtonOpcode::Sshr => println!("Cton::Opcode = Sshr"),
        CtonOpcode::SshrImm => println!("Cton::Opcode = SshrImm"),
        CtonOpcode::Ushr => println!("Cton::Opcode = Ushr"),
        CtonOpcode::UshrImm => println!("Cton::Opcode = UshrImm"),
        CtonOpcode::Popcnt => println!("Cton::Opcode = Popcnt"),
        CtonOpcode::Clz => println!("Cton::Opcode = Clz"),
        CtonOpcode::Ctz => println!("Cton::Opcode = Ctz"),
        CtonOpcode::Var => println!("Cton::Opcode = Var"),
        CtonOpcode::Infer => println!("Cton::Opcode = Infer"),
        CtonOpcode::ResultInst => println!("Cton::Opcode = Result"),
        CtonOpcode::Icmp => println!("Cton::Opcode = Icmp"),
        CtonOpcode::IcmpImm => println!("Cton::Opcode = IcmpImm"),
        _ => println!("Cton: other type yet to be handled"),
    }
}

pub fn get_clift_valdef_name(vdef: CtonValueDef) -> String {
    match vdef {
        CtonValueDef::Result => "Result".to_string(),
        CtonValueDef::Param => "Param".to_string(),
        CtonValueDef::NoneType => "None".to_string(),
    }
}

pub fn get_clift_instdata_name(instdata: CtonInstKind) -> String {
    match instdata {
        CtonInstKind::Binary => "Binary".to_string(),
        CtonInstKind::BinaryImm => "BinaryImm".to_string(),
        CtonInstKind::Unary => "Unary".to_string(),
        CtonInstKind::UnaryImm => "UnaryImm".to_string(),
        CtonInstKind::IntCompare => "IntCompare".to_string(),
        CtonInstKind::IntCompareImm => "IntCompareImm".to_string(),
        CtonInstKind::Var => "Var".to_string(),
        _ => "".to_string(),
    }
}

pub fn get_clift_cond_name<'a>(cond: Option<CtonCmpCond>) -> String {
    match cond {
        Some(CtonCmpCond::Eq) => "eq".to_string(),
        Some(CtonCmpCond::Ne) => "ne".to_string(),
        Some(CtonCmpCond::Slt) => "slt".to_string(),
        Some(CtonCmpCond::Ult) => "ult".to_string(),
        Some(CtonCmpCond::Sle) => "sle".to_string(),
        Some(CtonCmpCond::Ule) => "ule".to_string(),
        None => "".to_string(),
    }
}

pub fn get_clift_opcode_name<'a>(opcode: CtonOpcode) -> String {
    match opcode {
        CtonOpcode::Iconst => "iconst".to_string(),
        CtonOpcode::Iadd => "iadd".to_string(),
        CtonOpcode::Imul => "imul".to_string(),
        CtonOpcode::ImulImm => "imul_imm".to_string(),
        CtonOpcode::Isub => "isub".to_string(),
        CtonOpcode::IsubImm => "irsub_imm".to_string(),
        CtonOpcode::Band => "band".to_string(),
        CtonOpcode::BandImm => "band_imm".to_string(),
        CtonOpcode::Bor => "bor".to_string(),
        CtonOpcode::BorImm => "bor_imm".to_string(),
        CtonOpcode::Bxor => "bxor".to_string(),
        CtonOpcode::BxorImm => "bxor_imm".to_string(),
        CtonOpcode::Ishl => "ishl".to_string(),
        CtonOpcode::IshlImm => "ishl_imm".to_string(),
        CtonOpcode::Sshr => "sshr".to_string(),
        CtonOpcode::SshrImm => "sshr_imm".to_string(),
        CtonOpcode::Ushr => "ushr".to_string(),
        CtonOpcode::UshrImm => "ushr_imm".to_string(),
        CtonOpcode::Popcnt => "popcnt".to_string(),
        CtonOpcode::Clz => "clz".to_string(),
        CtonOpcode::Ctz => "ctz".to_string(),
        CtonOpcode::IaddImm => "iadd_imm".to_string(),
        CtonOpcode::Var => "Var".to_string(),
        CtonOpcode::Icmp => "icmp".to_string(),
        CtonOpcode::IcmpImm => "icmp_imm".to_string(),
        CtonOpcode::Infer => "Infer".to_string(),
        CtonOpcode::ResultInst => "Result".to_string(),
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
        }
        None => None,
    }
}

pub fn inst_has_const_operand(clift_ops: Option<Vec<CtonOperand>>) -> bool {
    clift_ops
        .as_ref()
        .into_iter()
        .flat_map(|ops| &**ops)
        .find(|op| op.const_val.is_some())
        .is_some()
}

//pub fn get_operand(ops: Option<Vec<CtonOperand>>, i: u32) -> Option<CtonOperand> {
//    match ops {
//        Some(cops) => {
//            cops[i]
//        },
//        None => None,
//   }
//}
//
//pub fn build_clift_insts(
//    opcode: CtonOpcode,
//    ops: Option<Vec<CtonOperand>>,
//    kind: CtonInstKind,
//    cond: Option<CtonCmpCond>,
//    width: u32,
//    var_num: Option<u32>) -> Vec<CtonInst> {
//        let mut insts = Vec![];
//        // if both operands are const
//        let op0 = get_operand(clift_ops.clone(), 0);
//        let op1 = get_operand(clift_ops.clone(), 1);
//        if both_const_operands(clift_ops.clone()) {
//            // iconst op0
//            let mut ops0: Vec<CtonOperand> = Vec::new();
//            ops0.push(Some(op0));
//            insts.push(CtonInst {
//                valuedef: CtonValueDef::Result,
//                kind: CtonInstKind::UnaryImm,
//                opcode: CtonOpcode::ConstInst,
//                cond: None,
//                width: width,
//                var_num: None,
//                cops: Some(ops0),
//            });
//            // iconst op1
//            let mut ops1: Vec<CtonOperand> = Vec::new();
//            ops1.push(Some(op1));
//            insts.push(CtonInst {
//                valuedef: CtonValueDef::Result,
//                kind: CtonInstKind::UnaryImm,
//                opcode: CtonOpcode::ConstInst,
//                cond: None,
//                width: width,
//                var_num: None,
//                cops: Some(ops1),
//            });
//            // iadd op0, op1
//            // let mut all_ops: Vec<CtonOperand> = Vec::new();
//            // all_ops.push(ops0);
//            // all_ops.push(ops1);
//            insts.push(CtonInst {
//                valuedef: CtonValueDef::Result,
//                kind: CtonInstKind::Binary,
//                opcode: CtonOpcode::Iadd,
//                cond: None,
//                width: width,
//                var_num: None,
//                cops: Some(all_ops),
//            });
//        } else if inst_has_const_operand(clift_ops.clone()) {
//            inst_opcode = CtonOpcode::IaddImm;
//            kind = CtonInstKind::BinaryImm;
//        } else {
//        CtonInst {
//            valuedef: CtonValueDef::Result,
//            kind: kind,
//            opcode: inst_opcode,
//            cond: None,
//            width: width,
//            var_num: var_number,
//            cops: clift_ops,
//        }
//        }
//}

/// Codegen Phase #1
pub fn mapping_souper_to_cton_isa(souper_inst: Inst) -> CtonInst {
    match souper_inst {
        Inst {
            kind,
            lhs_idx,
            width,
            var_number,
            ops,
            ..
        } => {
            match kind {
                // FIXME: Deal with ops mapping in a better way later
                // because, we have to get rid of souperoperand type completely
                InstKind::Const => {
                    let clift_ops = build_clift_ops(ops);
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::UnaryImm,
                        opcode: CtonOpcode::Iconst,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Add => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Iadd;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IaddImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Mul => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Imul;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::ImulImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Sub => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Isub;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IsubImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Eq => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Icmp;
                    let mut kind = CtonInstKind::IntCompare;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IcmpImm;
                        kind = CtonInstKind::IntCompareImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: Some(CtonCmpCond::Eq),
                        width: 1,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Ne => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Icmp;
                    let mut kind = CtonInstKind::IntCompare;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IcmpImm;
                        kind = CtonInstKind::IntCompareImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: Some(CtonCmpCond::Ne),
                        width: 1,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Slt => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Icmp;
                    let mut kind = CtonInstKind::IntCompare;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IcmpImm;
                        kind = CtonInstKind::IntCompareImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: Some(CtonCmpCond::Slt),
                        width: 1,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Ult => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Icmp;
                    let mut kind = CtonInstKind::IntCompare;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IcmpImm;
                        kind = CtonInstKind::IntCompareImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: Some(CtonCmpCond::Ult),
                        width: 1,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Sle => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Icmp;
                    let mut kind = CtonInstKind::IntCompare;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IcmpImm;
                        kind = CtonInstKind::IntCompareImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: Some(CtonCmpCond::Sle),
                        width: 1,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Ule => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Icmp;
                    let mut kind = CtonInstKind::IntCompare;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IcmpImm;
                        kind = CtonInstKind::IntCompareImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: Some(CtonCmpCond::Ule),
                        width: 1,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::And => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Band;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::BandImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Or => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Bor;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::BorImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Xor => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Bxor;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::BxorImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Shl => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Ishl;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::IshlImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Lshr => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Ushr;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::UshrImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Ashr => {
                    let clift_ops = build_clift_ops(ops);
                    let mut inst_opcode = CtonOpcode::Sshr;
                    let mut kind = CtonInstKind::Binary;
                    if inst_has_const_operand(clift_ops.clone()) {
                        inst_opcode = CtonOpcode::SshrImm;
                        kind = CtonInstKind::BinaryImm;
                    }
                    CtonInst {
                        valuedef: CtonValueDef::Result,
                        kind: kind,
                        opcode: inst_opcode,
                        cond: None,
                        width: width,
                        var_num: var_number,
                        cops: clift_ops,
                        lhs_index: lhs_idx,
                    }
                }
                InstKind::Ctpop => CtonInst {
                    valuedef: CtonValueDef::Result,
                    kind: CtonInstKind::Unary,
                    opcode: CtonOpcode::Popcnt,
                    cond: None,
                    width: width,
                    var_num: var_number,
                    cops: build_clift_ops(ops),
                    lhs_index: lhs_idx,
                },
                InstKind::Ctlz => CtonInst {
                    valuedef: CtonValueDef::Result,
                    kind: CtonInstKind::Unary,
                    opcode: CtonOpcode::Clz,
                    cond: None,
                    width: width,
                    var_num: var_number,
                    cops: build_clift_ops(ops),
                    lhs_index: lhs_idx,
                },
                InstKind::Cttz => CtonInst {
                    valuedef: CtonValueDef::Result,
                    kind: CtonInstKind::Unary,
                    opcode: CtonOpcode::Ctz,
                    cond: None,
                    width: width,
                    var_num: var_number,
                    cops: build_clift_ops(ops),
                    lhs_index: lhs_idx,
                },
                InstKind::Var => CtonInst {
                    valuedef: CtonValueDef::Param,
                    kind: CtonInstKind::Var,
                    opcode: CtonOpcode::Var,
                    cond: None,
                    width: width,
                    var_num: var_number,
                    cops: build_clift_ops(ops),
                    lhs_index: lhs_idx,
                },
                InstKind::Infer => CtonInst {
                    valuedef: CtonValueDef::NoneType,
                    kind: CtonInstKind::NoneType,
                    opcode: CtonOpcode::Infer,
                    cond: None,
                    width: width,
                    var_num: var_number,
                    cops: build_clift_ops(ops),
                    lhs_index: lhs_idx,
                },
                InstKind::ResultInst => CtonInst {
                    valuedef: CtonValueDef::NoneType,
                    kind: CtonInstKind::NoneType,
                    opcode: CtonOpcode::ResultInst,
                    cond: None,
                    width: width,
                    var_num: var_number,
                    cops: build_clift_ops(ops),
                    lhs_index: lhs_idx,
                },
                _ => CtonInst {
                    valuedef: CtonValueDef::Param,
                    kind: CtonInstKind::Var,
                    opcode: CtonOpcode::Var,
                    cond: None,
                    width: width,
                    var_num: None,
                    cops: build_clift_ops(ops),
                    lhs_index: lhs_idx,
                },
            }
        }
    }
}

pub fn transform_souper_to_clift_insts(souper_insts: Vec<Inst>) -> Vec<CtonInst> {
    let mut cton_insts: Vec<CtonInst> = Vec::new();
    for souper_inst in souper_insts {
        let cton_inst = mapping_souper_to_cton_isa(souper_inst);
        cton_insts.push(cton_inst);
    }
    cton_insts
}
