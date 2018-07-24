// Pattern matcher

use cliftinstbuilder::{self, CtonInst, CtonValueDef, CtonInstKind, CtonOpcode, CtonOperand};

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

pub fn get_total_number_of_args(inst: &CtonInst) -> usize {
    let ops = inst.cops.clone();
    match ops {
        Some(ops_list) => {
            ops_list.len()
        },
        None => 0,
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

    pub fn build_separate_arg_node(&mut self, arg: usize) -> Node {
        Node {
            node_type: NodeType::match_args,
            node_value: get_arg_name(arg),
            id: self.count,
            next: None,
        }
    }

    pub fn build_constant_node(&mut self, constant: i64) -> Node {
        unimplemented!();
    }

    //pub fn build_args_node(&mut self, clift_inst: &CtonInst) -> Vec<Node> {
    pub fn build_args_node(&mut self, clift_inst: &CtonInst) {
        // create an arguments arena
        let mut arg_arena = Arena::new();

        // set the counter of arg arena to parent arena counter
        // to help set id of arg nodes
        arg_arena.count = self.count;

        //let mut args_nodes: Vec<Node> = Vec::new();
        let total_args = get_total_number_of_args(clift_inst);
        for op in 0 .. total_args {
            println!("current inst === ");
            cliftinstbuilder::get_cton_inst_name(clift_inst.opcode.clone());
            println!("====");
            // build just a named argument node (wrapper node)
            println!("** op = {}", op);
            let named_arg_node = arg_arena.build_separate_arg_node(op);
            println!("---- named arg id = {}", named_arg_node.id);
            arg_arena.update_count();

            // repeat the prefix tree build here again!
            let cops = clift_inst.cops.clone();
            match cops {
                Some(ops) => {
                    let arg = &ops[op];
                    match arg.idx_val.clone() {
                        Some(idx) => {
                            let root_inst = &self.clift_insts[idx];
                            println!("build detailed node now\n");
                            let detail_arg_node = arg_arena.build_sequence_of_nodes(root_inst);
                        },
                        None => {
                            match arg.const_val.clone() {
                                Some(constant) => {
                                    let const_arg_node = arg_arena.build_constant_node(constant);
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
            // set the next of previous node (i.e. named_arg_node)
            //self.set_next_of_prev_node(detail_arg_node, named_arg_node);
            // push the named and detailed node to the args node vec
            //args_nodes.push(named_arg_node);
            //args_nodes.push(detail_arg_node);
        }
    }

    //pub fn build_sequence_of_nodes(&mut self, clift_inst: &CtonInst) -> Vec<Node> {
    pub fn build_sequence_of_nodes(&mut self, clift_inst: &CtonInst) {
        let node_instdata = self.build_instdata_node(clift_inst);
        println!("---- instdata id = {}", node_instdata.id);
        self.update_count();

        let node_opcode = self.build_opcode_node(clift_inst);
        println!("---- opcode id = {}", node_opcode.id);
        self.update_count();

        let updated_id_node = self.set_next_of_prev_node(node_opcode.clone(), node_instdata.clone());

        self.nodes.push(updated_id_node.clone());
        self.nodes.push(node_opcode);

        let node_args = self.build_args_node(clift_inst);
    }
}

pub fn generate_single_tree_patterns(clift_insts: Vec<CtonInst>) {
    let last_clift_inst = get_last_clift_inst(clift_insts.clone());
    let last_clift_ops = get_last_clift_op(last_clift_inst);
    let index_from_last_clift_op = get_index_from_last_clift_op(last_clift_ops);
    let inst_at_last_op_idx = &clift_insts[index_from_last_clift_op];

    /// Create Arena and initialize it
    let mut arena = Arena::new();
    arena.clift_insts = clift_insts.clone();
    arena.build_sequence_of_nodes(inst_at_last_op_idx);
}
