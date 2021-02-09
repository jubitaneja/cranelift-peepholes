use lhspatternmatcher::{Node, NodeType};
use std::collections::HashMap;

pub fn insert_to_pc_hashmap(
    table: &mut HashMap<String, usize>,
    index: usize,
    argname: String) {
    table.insert(argname, index);
}

pub fn insert_to_nextnode_hashmap(
    table: &mut HashMap<usize, String>,
    next_id: usize,
    pcarg: String) {
    table.insert(next_id, pcarg);
}

pub fn update_pchashtable(
    table: &mut HashMap<String, usize>,
    argname: String) {
    table.remove(&argname);
}

pub fn build_pc_argname(node: Node) -> String {
    println!("\nbuild pc argname\n");
    let mut argname = "".to_string();
    argname.push_str(&node.arg_name);
    // Bugfix: node value can be "imm" for
    // iconst nodes
    println!("part1 of pcargname = {}\n", argname);
    println!("\npart2 fetched from node value = {}\n", &node.node_value.clone());
    // fixme: fix this bug here, on matching node value to "arg"
    // "arg" is the node value for the binaryimm opcode cases.
    // if node.node_value != "imm" {
    //     argname.push_str(&node.node_value[4..]);
    // }
    if node.node_value == "imm" {}
    else if node.node_value == "arg" {}
    else {
        argname.push_str(&node.node_value[4..]);
    }
    println!("pc argname returned = {}\n", argname);
    argname
}

pub fn get_pcarg_from_node_id(
    table: HashMap<usize, String>,
    id: usize) -> String {
    table[&id].clone()
}

pub fn get_path_condition_args_for_lhs(nodes: Vec<Node>) -> HashMap<String, usize> {
    println!("** pctable: start func\n");
    let mut pcargs_to_idx: HashMap<String, usize> = HashMap::new();
    let mut nextnode_table: HashMap<usize, String> = HashMap::new();

    for n in 0..nodes.len() {
        println!("***** Node ID = {}", nodes[n].id);
        match nodes[n].node_type {
            NodeType::MatchArgs => {
                // check if next node is Result or Param
                // if Param, do:
                // get the Some(idx_num) and arg_name of thatnode
                // and build pcarg_name
                // insert in hashmap -> pcargname, Some(idx_num)
                
                let pcarg = build_pc_argname(nodes[n].clone());
                match nodes[n].idx_num.clone() {
                    Some(idx) => {
                        insert_to_pc_hashmap(
                            &mut pcargs_to_idx,
                            idx,
                            pcarg.clone());

                        match nodes[n].next.clone() {
                            Some(next_nodes) => {
                                let next_node = next_nodes[0].clone();
                                insert_to_nextnode_hashmap(
                                    &mut nextnode_table,
                                    next_node.index,
                                    pcarg);
                            },
                            None => {},
                        }
                    },
                    None => {},
                }
            },
            NodeType::MatchValDef => {
                match nodes[n].clone().node_value.as_ref() {
                    "Result" => {
                        let id = nodes[n].clone().id;
                        let pcarg_name = get_pcarg_from_node_id(nextnode_table.clone(), id);
                        println!("Node type is ValueDef: Result => pcarg_name = {}", pcarg_name.clone());
                        update_pchashtable(&mut pcargs_to_idx, pcarg_name);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        for (x, y) in pcargs_to_idx.clone() {
            println!("arg = {}, idx = {}", x, y);
        }
    }
    pcargs_to_idx
}
