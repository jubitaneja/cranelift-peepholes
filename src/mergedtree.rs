// Merged prefix tree

use std::collections::HashMap;
use patternmatcher::{self, Arena, Node, NodeType, NodeID, Node_Index};


#[derive(Clone)]
pub struct MergedArena {
    pub merged_tree: Vec<Node>,
    pub hmap: HashMap<String, usize>,
}

impl MergedArena {
    pub fn build_root_node(&mut self) -> Node {
        Node {
            node_type: NodeType::match_root,
            node_value: "root".to_string(),
            id: 0,
            next: Some(Vec::new()),
        }
    }

    pub fn get_first_node_of_opt_pattern(&mut self, opt: Vec<Node>) -> Node {
        unimplemented!();
    }
}

pub fn generate_merged_prefix_tree(single_tree: Vec<Node>, mut merged_arena: MergedArena) -> MergedArena {
    if merged_arena.merged_tree.len() == 0 {
        let root_node = merged_arena.build_root_node();
    } else {
        let first_node = merged_arena.get_first_node_of_opt_pattern(single_tree);
    }
    //if merged_arena.merged_tree.len() == 0
    //  create root node
    //      add: match_root in node type,
    //      val = "root"
    //      id = 0
    //      next = Vec::new()
    //else {
    //  get top node of single_tree
    //  get value of top node = current_value
    //  get id of top node = current_id
    //
    //  if root.next.len() == 0 {
    //      create hashmap and add entry "current_val, current_id"
    //      push current_id to root.next
    //
    //      merged_arena.push(root_node)
    //      merged_arena.push(rest of the nodes of single tree)
    //
    //      ret merged_arena
    //  } else {
    //      TBD
    //  }
    //}
    unimplemented!();
}
