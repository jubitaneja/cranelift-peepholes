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
    pub lhs_index: usize,
}

pub struct RHSInfo {
    pub rhs_insts: Vec<CliftInstWithArgs>,
    pub full_table: HashMap<usize, String>,
}

impl RHSInfo {
    pub fn new() -> RHSInfo {
        RHSInfo {
            rhs_insts: Vec::new(),
            full_table: HashMap::new(),
        }
    }

    pub fn get_arg_name(&mut self, idx: usize, tbl: HashMap<usize, String>) -> String {
        let mut arg_name = "".to_owned();
        match tbl.get(&idx) {
            Some(name) => arg_name.push_str(&name),
            None => {
                println!("************** arg name not found, for index = {}\n", idx);
                arg_name.push_str(&String::from("rhs_inst_"));
                arg_name.push_str(&idx.to_string());
            },
        }
        arg_name
    }
}

pub fn update_rhs_with_argnames(
    insts: Vec<CtonInst>,
    mut idx_to_argname: HashMap<usize, String>
) -> RHSInfo {
    let mut rhs_info = RHSInfo::new();
    
    println!("= = = = = = = In fn: update_rhs_with_argnames()");
    for i in 0..insts.len() {
        println!("for RHS inst #{}", i);
        let inst = insts[i].clone();
        let mut new_inst = CliftInstWithArgs {
            valuedef: inst.valuedef,
            kind: inst.kind,
            opcode: inst.opcode,
            cond: inst.cond,
            width: inst.width,
            var_num: inst.var_num,
            cops: Vec::new(),
            lhs_index: inst.lhs_index,
        };
        let mut ops_list: Vec<String> = Vec::new();

        // Create an arg name for Left part of each RHS instruction and
        // store it in the "idx_to_argname" hashmap
        // Example: 
        // %0 = var
        // ..
        // infer %3
        // %4 [lhs_index = 4] = mul %0 [0], %1 [1]
        // now, we are creating arg name for lhs_index = 4 and
        // storing an entry {4, rhsinst_4} in hashmap

        // Here, get_arg_name() function will always go to the None case
        // and create the new name of rhs instruction and return
        println!("********************** get rhs inst name for index = {}", inst.lhs_index);
        let rhs_inst_name = rhs_info.get_arg_name(inst.lhs_index, idx_to_argname.clone());
        idx_to_argname.insert(inst.lhs_index, rhs_inst_name);

        match inst.cops {
            Some(ops) => {
                for op in ops {
                    match op.idx_val {
                        Some(idx) => {
                            // fetch arg name from hashmap for 'idx'
                            let arg_name = rhs_info.get_arg_name(idx, idx_to_argname.clone());
                            println!("op index = {}, arg name from table = {}", idx, arg_name.clone());
                            // push arg name to ops_list
                            ops_list.push(arg_name);
                        },
                        None => {
                            match op.const_val {
                                Some(c) => {
                                    // push constant val 'c'.to_string()
                                    // to ops_list
                                    println!("op const val = {}", c);
                                    ops_list.push(c.to_string())
                                },
                                None => {println!("op has no index and it's not constant");},
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
    for (x, y) in &idx_to_argname {
        println!("\t^^^^^^^^^^^^^^^idx = {}, arg = {}", x, y);
    }

    RHSInfo {
        rhs_insts: rhs_info.rhs_insts,
        full_table: idx_to_argname,
    }
}
