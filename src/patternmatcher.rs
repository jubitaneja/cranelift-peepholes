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
        }
    }

    pub fn build_instdata_node(&mut self, clift_inst: &CtonInst) -> Node {
        let instdata_val = clift_inst.kind.clone();
        let id = self.nodes.len();
        Node {
            node_type: NodeType::match_instdata,
            node_value: cliftinstbuilder::get_clift_instdata_name(instdata_val),
            id: id,
            next: None,
        }
    }

    pub fn build_opcode_node(&mut self, clift_inst: &CtonInst) -> Node {
        let opcode_val = clift_inst.opcode.clone();
        Node {
            node_type: NodeType::match_opcode,
            node_value: cliftinstbuilder::get_clift_opcode_name(opcode_val),
            id: self.nodes.len(),
            next: None,
        }
    }

    pub fn set_next_of_prev_node(&mut self, current: Node, mut previous: Node) {
        let mut next_ids: Vec<NodeID> = Vec::new();
        next_ids.push(NodeID{ index: current.id, });
        previous.next = Some(next_ids);
    }

    pub fn build_separate_arg_node(&mut self, arg: usize) -> Node {
        Node {
            node_type: NodeType::match_args,
            node_value: get_arg_name(arg),
            id: self.nodes.len(),
            next: None,
        }
    }

    pub fn build_args_node(&mut self, clift_inst: &CtonInst) -> Vec<Node> {
        let mut args_nodes: Vec<Node> = Vec::new();
        let total_args = get_total_number_of_args(clift_inst);
        for op in 0 .. total_args {
            let plain_arg_node = self.build_separate_arg_node(op);
            // repeat the prefix tree build here again!
        }
        unimplemented!();
    }

    pub fn build_sequence_of_nodes(&mut self, clift_inst: &CtonInst) -> Vec<Node> {
        let node_instdata = self.build_instdata_node(clift_inst);
        self.nodes.push(node_instdata.clone());

        let node_opcode = self.build_opcode_node(clift_inst);
        self.set_next_of_prev_node(node_opcode.clone(), node_instdata.clone());
        self.nodes.push(node_opcode);

        let node_args = self.build_args_node(clift_inst);
        unimplemented!();
    }
}

/// Build prefix tree with root instruction
///pub fn build_prefix_tree(root_inst: &CtonInst) {
///    /// Create Arena and initialize it
///    let mut arena = Arena::new();
///    arena.build_sequence_of_nodes(root_inst);
///}

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
