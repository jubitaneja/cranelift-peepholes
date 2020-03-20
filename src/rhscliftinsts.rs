// RHS cranelift instructions only

use cliftinstbuilder::{CtonInst, CtonOpcode};

pub fn get_result_clift_insts_only(all_insts: Vec<CtonInst>) -> Vec<CtonInst> {
    // just split out only rhs part from all instructions
    let mut infer_found = false;
    let mut rhs_insts: Vec<CtonInst> = Vec::new();
    for inst in all_insts {
        match inst.opcode {
            CtonOpcode::Infer => {
                infer_found = true;
            }
            _ => {
                if infer_found {
                    // start collecting result part now
                    rhs_insts.push(inst);
                } else {
                    continue;
                }
            }
        }
    }
    rhs_insts
}
