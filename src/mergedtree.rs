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
        if opt.len() != 0 {
            opt[0].clone()
        } else {
            panic!("Error: optimization pattern has no nodes")
        }
    }

    pub fn get_value_of_node(&mut self, node: Node) -> String {
        node.node_value
    }

    pub fn get_id_of_node(&mut self, node: Node) -> usize {
        node.id
    }

    pub fn update_hash_map(&mut self, value: String, id: usize) {
        self.hmap.insert(value, id);
    }

    pub fn update_next_nodes_list(&mut self, mut node: Node, next_id: usize) -> Node {
        let mut next_ids: Vec<NodeID> = Vec::new();
        next_ids.push(NodeID{ index: next_id });
        node.next = Some(next_ids);
        node
    }

    pub fn add_node_to_arena(&mut self, node: Node) {
        self.merged_tree.push(node);
    }

    pub fn find_node_with_id_in_arena(&mut self, node_id: usize) -> Node {
        // FIXME: not a good implementation to traverse and exit if id is found
        let mut found_node = Node {
                                   node_type: NodeType::match_none,
                                   node_value: "dummy".to_string(),
                                   id: 10000,
                                   next: None,
                                  };
        for n in 0 .. self.merged_tree.len() {
            if self.merged_tree[n].id == node_id {
                found_node = self.merged_tree[n].clone();
                break;
            } else {
                panic!("Error: node with id: {} doesn't exist in merged_arena", node_id);
            }
        }
        found_node
    }

    pub fn node_has_any_connection(&mut self, node: Node) -> bool {
        let mut connections = false;
        if let Some(nodes_list) = node.next {
            if nodes_list.len() == 0 {
                connections = false;
            } else {
                connections = true;
            }
        }
        connections
    }
}

pub fn generate_merged_prefix_tree(single_tree: Vec<Node>, mut merged_arena: MergedArena) -> MergedArena {
    if merged_arena.merged_tree.len() == 0 {
        let root_node = merged_arena.build_root_node();
        merged_arena.add_node_to_arena(root_node);
    }

    let first_node = merged_arena.get_first_node_of_opt_pattern(single_tree.clone());
    let current_val = merged_arena.get_value_of_node(first_node.clone());
    let current_id = merged_arena.get_id_of_node(first_node.clone());

    let found_root = merged_arena.find_node_with_id_in_arena(0);

    if !merged_arena.node_has_any_connection(found_root) {
        println!("No connection of root node ------------");
    } else {
        println!("found connection of root node ------------");
    }

    merged_arena
}
