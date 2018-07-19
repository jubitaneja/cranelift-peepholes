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
    previous: Option<NodeID>,
    next: Option<NodeID>,
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
        Arena { nodes: Vec::new() }
    }

    pub fn build_instdata_node(&mut self, clift_inst: &CtonInst) -> Node {
        let instdata_val = clift_inst.kind.clone();
        let mut prev = None;
        let mut next = None;
        if self.nodes.len() == 0 {
            prev = None;
            next = None;
        } else {
            prev = Some(NodeID{index: self.nodes.len() - 1});
            next = None;
        }
        let id = self.nodes.len();
        Node {
            node_type: NodeType::match_instdata,
            node_value: cliftinstbuilder::get_clift_instdata_name(instdata_val),
            id: id,
            previous: prev,
            next: next,
        }
    }

    pub fn build_opcode_node(&mut self, clift_inst: &CtonInst) -> Node {
        let opcode_val = clift_inst.opcode.clone();
        Node {
            node_type: NodeType::match_opcode,
            node_value: cliftinstbuilder::get_clift_opcode_name(opcode_val),
            id: self.nodes.len(),
            previous: Some(NodeID {index: self.nodes.len() - 1}),
            next: None,
        }
    }

    pub fn set_next_of_prev_node(&mut self, current: Node) {
        let current_prev = current.previous;
        match current_prev {
            Some(NodeID{index}) => {
                self.nodes[index].next = Some(NodeID{index: current.id});
            },
            None => {
                panic!("Current node must have a valid previous index");
            },
        }
    }

    pub fn build_separate_arg_node(&mut self, arg: usize) -> Node {
        Node {
            node_type: NodeType::match_args,
            node_value: get_arg_name(arg),
            id: self.nodes.len(),
            previous: Some(NodeID {index: self.nodes.len() - 1}),
            next: None,
        }
    }

    pub fn build_args_node(&mut self, clift_inst: &CtonInst) -> Vec<Node> {
        let mut args_nodes: Vec<Node> = Vec::new();
        let total_args = get_total_number_of_args(clift_inst);
        for op in 0 .. total_args {
            let plain_arg_node = self.build_separate_arg_node(op);
            self.set_next_of_prev_node(plain_arg_node.clone());
            // repeat the prefix tree build here again!
        }
        unimplemented!();
    }

    pub fn build_sequence_of_nodes(&mut self, clift_inst: &CtonInst) -> Vec<Node> {
        let node_instdata = self.build_instdata_node(clift_inst);
        self.nodes.push(node_instdata);

        let node_opcode = self.build_opcode_node(clift_inst);
        self.set_next_of_prev_node(node_opcode.clone());
        self.nodes.push(node_opcode);

        let node_args = self.build_args_node(clift_inst);
        unimplemented!();
    }
}

/// Build prefix tree with root instruction
pub fn build_prefix_tree(root_inst: &CtonInst) {
    /// Create Arena and initialize it
    let mut arena = Arena::new();

    /// Create a node and push it, if arena is empty
    //if arena.nodes.is_empty() {
        arena.build_sequence_of_nodes(root_inst);
    //}
}

pub fn generate_single_tree_patterns(clift_insts: Vec<CtonInst>) {
    let last_clift_inst = get_last_clift_inst(clift_insts.clone());
    let last_clift_ops = get_last_clift_op(last_clift_inst);
    let index_from_last_clift_op = get_index_from_last_clift_op(last_clift_ops);
    let inst_at_last_op_idx = &clift_insts[index_from_last_clift_op];

    // Start building prefix tree from here
    build_prefix_tree(inst_at_last_op_idx);
}
