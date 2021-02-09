use lhspatternmatcher::{Node, NodeType};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct LHSInfo {
    pub nodes: Vec<Node>,
    pub htable: HashMap<usize, String>,
}

pub struct OpStacks {
    arg_stack: Vec<Node>,
    parent_stack: Vec<Node>,
}

impl OpStacks {
    pub fn new() -> OpStacks {
        OpStacks {
            arg_stack: Vec::new(),
            parent_stack: Vec::new(),
        }
    }

    pub fn push_to_arg_stack(&mut self, node: Node) {
        self.arg_stack.push(node);
    }

    pub fn push_to_parent_stack(&mut self, node: Node) {
        self.parent_stack.push(node);
    }

    pub fn ready_to_pop_from_arg_stack(&mut self, args: usize) -> bool {
        // FIXED: this popping logic should be on the basis
        // of top element in parent stack. If its binary,
        // then we need at least 2 elems in arg stack to pop.
        // if it is unary on top of parent stack, then we
        // need atleast one elem in arg stack, and so on for other
        // types
        if self.arg_stack.len() >= args {
            true
        } else {
            false
        }
    }

    pub fn pop_from_arg_stack(&mut self) -> Option<Node> {
        self.arg_stack.pop()
    }

    pub fn update_arg_name_for_node(
        &mut self,
        mut node: Node,
        name: String) -> Node {
        node.arg_name = name;
        node
    }

    pub fn update_in_lhs(&mut self, update: Node, nodes: &mut Vec<Node>) {
        for n in 0..nodes.len() {
            if nodes[n].id == update.id {
                nodes[n].arg_name = update.arg_name;
                break;
            }
        }
    }

    // FIXME: Bug in initializing `id = 0`
    // WORKAROUND: For now, insert in table hashmap happens
    // only when the id doesn't exist and it's a new entry
    // But, ideally we would want to fix the issue of not
    // having a default value of id = 0 in None case
    // of `match idx {..}`
    pub fn insert_in_hashmap(
        &mut self,
        table: &mut HashMap<usize, String>,
        idx: Option<usize>,
        name: String,
        argnum: String) {
        let mut id: usize = 0;
        let mut argname = name;
        println!("fn: insert_in_hashmap(): name= {}, argnum/val = {}", argname, argnum);
        match idx {
            Some(i) => id = i,
            _ => {
                println!("for immediate args, idx is None and hance assigned id = {}", id);
            },
        }
        if let Some(i) = argnum.find('[') {
            argname.push_str(&(argnum)[i..]);
        }
        println!("Check if this ID is already assigned any argname?");
        if table.contains_key(&id) {
            println!("Yes, id = {} exists in hashmap with value = {}", id, table[&id]);
        } else {
            println!("No, id = {} doesn't exist in table, so add it surely", id);
            println!("Going to insert idx = {}, name = {}\n", id, argname);
            table.insert(id, argname);
        }
    }
}

pub fn update_arg_nodes_in_lhs(mut nodes: Vec<Node>) -> LHSInfo {
    let mut process = OpStacks::new();
    let mut idx_to_arg_name: HashMap<usize, String> = HashMap::new();
    for node in (0..nodes.len()).rev() {
        match nodes[node].node_type {
            NodeType::MatchArgs => {
                println!("Push node: {} to arg stack\n", nodes[node].id);
                process.push_to_arg_stack(nodes[node].clone());
            },
            NodeType::InstType => {
                match nodes[node].node_value.as_ref() {
                    // FIXME: TODO: Add another case for UnaryImm, Unary
                    // and in that only pop one node, i.e. replicate logic
                    // for n1 only.
                    "Binary" | "BinaryImm" |
                    "IntCompare" | "IntCompareImm" => {
                        // FIXME: fix implementation of popping logic check test
                        // details mentioned above
                        // FIXED: Feb 1, 21 - Passing the parameter "2" i.e.
                        // total number of arguments for node
                        if process.ready_to_pop_from_arg_stack(2) {
                            let parent_arg_name = &nodes[node].arg_name.clone();
                            // pop/update the first node and then second, n1 and n2
                            let n1 = process.pop_from_arg_stack();
                            match n1 {
                                Some(n) => {
                                    println!("Popped node: {} from arg_stack\n", n.id);
                                    let updated_n1 = process. 
                                        update_arg_name_for_node(
                                            n,
                                            parent_arg_name.clone());
                                    process.update_in_lhs(updated_n1.clone(), &mut nodes);
                                    process.insert_in_hashmap(
                                        &mut idx_to_arg_name,
                                        updated_n1.idx_num,
                                        updated_n1.arg_name,
                                        updated_n1.node_value
                                    );
                                },
                                None => {},
                            }
                            let n2 = process.pop_from_arg_stack();
                            match n2 {
                                Some(n) => {
                                    println!("Popped node: {} from arg_stack\n", n.id);
                                    let updated_n2 = process.
                                        update_arg_name_for_node(
                                            n,
                                            parent_arg_name.clone());
                                    process.update_in_lhs(updated_n2.clone(), &mut nodes);
                                    process.insert_in_hashmap(
                                        &mut idx_to_arg_name,
                                        updated_n2.idx_num,
                                        updated_n2.arg_name,
                                        updated_n2.node_value
                                    );
                                },
                                None => {},
                            }
                        } else {
                            println!("Push node: {} to parent stacj\n", nodes[node].id);
                            process.push_to_parent_stack(nodes[node].clone());
                        }
                    },
                    "Unary" | "UnaryImm" => {
                        if process.ready_to_pop_from_arg_stack(1) {
                            let parent_arg_name = &nodes[node].arg_name.clone();
                            // pop/update the first node and then second, n1 and n2
                            let n1 = process.pop_from_arg_stack();
                            match n1 {
                                Some(n) => {
                                    println!("Popped node: {} from arg_stack\n", n.id);
                                    let updated_n1 = process. 
                                        update_arg_name_for_node(
                                            n,
                                            parent_arg_name.clone());
                                    process.update_in_lhs(updated_n1.clone(), &mut nodes);
                                    process.insert_in_hashmap(
                                        &mut idx_to_arg_name,
                                        updated_n1.idx_num,
                                        updated_n1.arg_name,
                                        updated_n1.node_value
                                    );
                                },
                                None => {},
                            }
                        } else {
                            println!("Push node: {} to parent stacj\n", nodes[node].id);
                            process.push_to_parent_stack(nodes[node].clone());
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    println!("----**** Debugging updated LHS with arg names ****----");
    for n in 0..nodes.len() {
        println!("Node id = {}", nodes[n].id);
        match nodes[n].clone().var_id {
            Some (var_num) => {
                println!("Var number = {}", var_num);
            },
            None => {
                println!("Var number = None\n");
            },
        }
        println!("Node value = {}", nodes[n].node_value);
        match nodes[n].idx_num.clone() {
            Some(i) => println!("Node op idx num = {}", i),
            None => println!("Node op idx num = NONE"),
        }
        println!("Node arg_name == {}", nodes[n].arg_name);
        match nodes[n].clone().next {
            Some(x) => {
                for i in 0 .. x.len() {
                    println!("next = {}", x[i].index);
                }
            },
            None => {
                println!("next = None");
            }
        }
        println!("-------***************************--------");
    }

    LHSInfo {
        nodes: nodes,
        htable: idx_to_arg_name,
    }
}
