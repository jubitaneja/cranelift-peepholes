// Pattern matcher

use cliftinstbuilder::{self, CtonInst, CtonValueDef, CtonInstKind, CtonOpcode, CtonOperand};

pub struct Arena {
    nodes: Vec<Node>,
    clift_insts: Vec<CtonInst>,
    count: usize,
}

#[derive(Clone)]
pub enum NodeType {
    match_instdata,
    match_opcode,
    match_args,
    match_const,
}

#[derive(Clone)]
pub struct Node {
    node_type: NodeType,
    node_value: String,
    id: usize,
    next: Option<Vec<NodeID>>,
}

#[derive(Clone)]
pub struct NodeID {
    index: usize,
}

#[derive(Clone)]
pub struct Node_Index {
    node: Node,
    index: usize,
}

/// Helper functions
pub fn get_arg_name(index: usize) -> String {
    let mut arg = "arg".to_string();
    arg.push_str(&(index.to_string()));
    arg
}

/// Returns the infer instruction from cranelift instrictions set
pub fn get_last_clift_inst(clift_insts: Vec<CtonInst>) -> CtonInst {
    let last_inst = clift_insts.last();
    match last_inst {
        Some(last_inst) => {
            last_inst.clone()
        },
        None => {
            panic!("No cranelift instruction found");
        }
    }
}

/// Returns the operands of infer instruction
pub fn get_last_clift_op(infer_inst: CtonInst) -> Vec<CtonOperand> {
    let infer_ops = infer_inst.cops;
    match infer_ops {
        Some(ops) => {
            assert_eq!(ops.len(), 1, "Infer instruction must have only one operand");
            ops
        },
        None => panic!("Infer instruction must have one operand"),
    }
}

/// Returns the index of instruction infer instruction points to
pub fn get_index_from_last_clift_op(infer_ops: Vec<CtonOperand>) -> usize {
    let mut idx: usize = 0;
    for op in infer_ops {
        assert_eq!(op.const_val, None, "operand of infer inst must be an index to another inst");
        match op.idx_val {
            Some(index) => {
                idx = index;
            },
            None => panic!("operand of infer inst must have a valid index value to another inst"),
        }
    }
    idx
}

pub fn get_total_number_of_args(inst: &CtonInst) -> usize {
    let ops = inst.cops.clone();
    match ops {
        Some(ops_list) => {
            ops_list.len()
        },
        None => 0,
    }
}

pub fn get_node_type(ty: NodeType) -> String {
    match ty {
        NodeType::match_instdata => "match_instdata".to_string(),
        NodeType::match_opcode => "match_opcode".to_string(),
        NodeType::match_args => "match_args".to_string(),
        NodeType::match_const => "match_const".to_string(),
        _ => panic!("Unexpected node type"),
    }
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            nodes: Vec::new(),
            clift_insts: Vec::new(),
            count: 0,
        }
    }

    pub fn update_count(&mut self) {
        self.count += 1;
    }

    pub fn build_instdata_node(&mut self, clift_inst: &CtonInst) -> Node {
        let instdata_val = clift_inst.kind.clone();
        Node {
            node_type: NodeType::match_instdata,
            node_value: cliftinstbuilder::get_clift_instdata_name(instdata_val),
            id: self.count,
            next: None,
        }
    }

    pub fn build_opcode_node(&mut self, clift_inst: &CtonInst) -> Node {
        let opcode_val = clift_inst.opcode.clone();
        Node {
            node_type: NodeType::match_opcode,
            node_value: cliftinstbuilder::get_clift_opcode_name(opcode_val),
            id: self.count,
            next: None,
        }
    }

    pub fn set_next_of_prev_node(&mut self, current: Node, mut previous: Node) -> Node {
        let mut next_ids: Vec<NodeID> = Vec::new();
        next_ids.push(NodeID{ index: current.id, });
        previous.next = Some(next_ids);
        previous
    }

    pub fn set_next_of_current_node_by_default(&mut self, mut current: Node) -> Node {
        let mut next_ids: Vec<NodeID> = Vec::new();
        next_ids.push(NodeID{ index: self.count.clone() });
        current.next = Some(next_ids);
        current
    }

    pub fn build_separate_arg_node(&mut self, arg: usize) -> Node {
        Node {
            node_type: NodeType::match_args,
            node_value: get_arg_name(arg),
            id: self.count,
            next: None,
        }
    }

    pub fn build_constant_node(&mut self, constant: i64) -> Node {
        Node {
            node_type: NodeType::match_const,
            node_value: constant.to_string(),
            id: self.count,
            next: None,
        }
    }

    pub fn get_node_with_id(&mut self, idx: usize) -> Option<Node_Index> {
        let mut ret_node = None;
        for n in 0 .. self.nodes.clone().len() {
            let node_id = self.nodes[n].clone().id;
            if node_id == idx {
                ret_node = Some(Node_Index {
                    node: self.nodes[n].clone(),
                    index: n,
                });
            } else {
                ret_node = None;
            }
        }
        ret_node
    }

    pub fn build_args_node(&mut self, clift_inst: &CtonInst) {
        let total_args = get_total_number_of_args(clift_inst);
        for op in 0 .. total_args {
            let named_arg_node = self.build_separate_arg_node(op);
            
            //set next of node before named arg node
            let c = self.count.clone();
            let node_x = self.get_node_with_id(c-1);

            match node_x {
                Some(Node_Index { mut node, index }) => {
                    let mut next_ids: Vec<NodeID> = Vec::new();
                    next_ids.push(NodeID{ index: self.count });
                    node.next = Some(next_ids);
                    self.nodes[index] = node;
                },
                None => {
                    panic!("No node with the id = {} found in arena", self.count-1);
                }
            }
            
            self.update_count();

            // set next of named arg node because it's 
            // sure to have some nodes after this
            let updated_named_arg_node = self.set_next_of_current_node_by_default(named_arg_node.clone());

            self.nodes.push(updated_named_arg_node);

            // repeat the prefix tree build here again!
            let cops = clift_inst.cops.clone();
            match cops {
                Some(ops) => {
                    let arg = &ops[op];
                    match arg.idx_val.clone() {
                        Some(idx) => {
                            let root_inst = &self.clift_insts[idx].clone();
                            let detail_arg_node = self.build_sequence_of_nodes(root_inst);
                        },
                        None => {
                            match arg.const_val.clone() {
                                Some(constant) => {
                                    let const_arg_node = self.build_constant_node(constant);
                                    self.update_count();
                                    self.nodes.push(const_arg_node);
                                },
                                None => {
                                    panic!("operand of a clift inst must have either an index value or a constant value")
                                }
                            }
                        },
                    }
                },
                None => {
                    panic!("The clift instruction is expected to have {} operands", total_args);
                }
            }
        }
    }

    pub fn build_sequence_of_nodes(&mut self, clift_inst: &CtonInst) -> Vec<Node> {
        let node_instdata = self.build_instdata_node(clift_inst);
        self.update_count();

        let node_opcode = self.build_opcode_node(clift_inst);
        self.update_count();

        let updated_id_node = self.set_next_of_prev_node(node_opcode.clone(), node_instdata.clone());

        self.nodes.push(updated_id_node.clone());
        self.nodes.push(node_opcode);

        self.build_args_node(clift_inst);

        self.nodes.clone()
    }
}

pub fn generate_single_tree_patterns(clift_insts: Vec<CtonInst>) -> Vec<Node> {
    let last_clift_inst = get_last_clift_inst(clift_insts.clone());
    let last_clift_ops = get_last_clift_op(last_clift_inst);
    let index_from_last_clift_op = get_index_from_last_clift_op(last_clift_ops);
    let inst_at_last_op_idx = &clift_insts[index_from_last_clift_op];

    /// Create Arena and initialize it
    let mut arena = Arena::new();
    arena.clift_insts = clift_insts.clone();
    let all_nodes = arena.build_sequence_of_nodes(inst_at_last_op_idx);

    // just for debugging puprose
    println!("--------------------------------");
    for n in 0 .. all_nodes.len() {
        println!("Node id = {}", all_nodes[n].id);
        println!("Node type = {}", get_node_type(all_nodes[n].clone().node_type));
        println!("Node value = {}", all_nodes[n].node_value);
        match all_nodes[n].clone().next {
            Some(x) => {
                for i in 0 .. x.len() {
                    println!("next = {}", x[i].index);
                }
            },
            None => {
                println!("next = None");
            }
        }
        println!("--------------------------------");
    }
    all_nodes
}
