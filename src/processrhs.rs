use cliftinstbuilder::{CtonInst, CtonValueDef,
    CtonInstKind, CtonOpcode,
    CtonCmpCond};
use std::collections::HashMap;

#[derive(Clone)]
pub struct CliftInstWithArgs {
    pub valuedef: CtonValueDef,
    pub kind: CtonInstKind,
    pub opcode: CtonOpcode,
    pub cond: Option<CtonCmpCond>,
    pub width: u32,
    pub var_num: Option<u32>,
    pub cops: Vec<String>,
}

pub struct RHSInfo {
    pub rhs_insts: Vec<CliftInstWithArgs>,
    // this count is for all arguments that
    // are not defined in LHS, but are defined
    // and used in RHS only.
    pub newarg_count: u32,
}

impl RHSInfo {
    pub fn new() -> RHSInfo {
        RHSInfo {
            rhs_insts: Vec::new(),
            newarg_count: 0,
        }
    }

    pub fn get_arg_name(&mut self, idx: usize, tbl: HashMap<usize, String>) -> String {
        let mut arg_name = "".to_owned();
        match tbl.get(&idx) {
            Some(name) => arg_name.push_str(&name),
            None => {
                println!("************** arg name not found, for index = {}\n", idx);
                arg_name.push_str(&String::from("new_arg_"));
                arg_name.push_str(&self.newarg_count.to_string());
                self.newarg_count += 1;
            },
        }
        arg_name
    }
}

pub fn update_rhs_with_argnames(
    insts: Vec<CtonInst>,
    idx_to_argname: HashMap<usize, String>
) -> Vec<CliftInstWithArgs> {
    let mut rhs_info = RHSInfo::new();
    
    for i in 0..insts.len() {
        let inst = insts[i].clone();
        let mut new_inst = CliftInstWithArgs {
            valuedef: inst.valuedef,
            kind: inst.kind,
            opcode: inst.opcode,
            cond: inst.cond,
            width: inst.width,
            var_num: inst.var_num,
            cops: Vec::new(),
        };
        let mut ops_list: Vec<String> = Vec::new();
        match inst.cops {
            Some(ops) => {
                for op in ops {
                    match op.idx_val {
                        Some(idx) => {
                            // fetch arg name from hashmap for 'idx'
                            let arg_name = rhs_info.get_arg_name(idx, idx_to_argname.clone());
                            // push arg name to ops_list
                            ops_list.push(arg_name);
                        },
                        None => {
                            match op.const_val {
                                Some(c) => {
                                    // push constant val 'c'.to_string()
                                    // to ops_list
                                    ops_list.push(c.to_string())
                                },
                                None => {},
                            }
                        },
                    }
                }
            },
            None => {},
        }
        new_inst.cops = ops_list;
        rhs_info.rhs_insts.push(new_inst);
    }

    // Debug  
    // for i in 0..rhslist.len() {
    //     println!("opcode = {}\n",cliftinstbuilder::get_clift_opcode_name(rhslist[i].opcode.clone()));
    //     for j in 0..rhslist[i].cops.len() {
    //         println!("op = {}, ", rhslist[i].cops[j]);
    //     }
    // }

    rhs_info.rhs_insts
}
