// Merged prefix tree

use std::collections::HashMap;
use lhspatternmatcher::{self, Arena, Node, NodeType, NodeID, Node_Index};


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
            width: 0,
            id: 0,
            var_id: None,
            arg_flag: false,
            level: 0,
            next: Some(Vec::new()),
        }
    }

    pub fn get_top_node_of_opt_pattern(&mut self, opt: Vec<Node>) -> Node {
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
        if let Some(mut node_next_list) = node.next {
            node_next_list.push(NodeID{ index: next_id });
            node.next = Some(node_next_list);
            node
        } else {
            panic!("Error: Node's next list is None, should be atleast an empty vec<NodeID>");
        }
    }

    pub fn update_node_with_arg_flag(&mut self, mut node: Node, val: bool) -> Node {
        node.arg_flag = val;
        node
    }

    pub fn update_node_with_level(&mut self, mut node: Node, level: usize) -> Node {
        node.level = level;
        node
    }

    // add a new node to arena
    pub fn add_node_to_arena(&mut self, node: Node) {
        self.merged_tree.push(node);
    }

    // when the node exists in arena already, update it
    pub fn update_node_in_arena(&mut self, updated_node: Node) {
        for n in 0 .. self.merged_tree.len() {
            if self.merged_tree[n].id == updated_node.id {
                self.merged_tree[n].next = updated_node.next;
                break;
            }
        }
    }

    pub fn update_node_arg_flag_in_arena(&mut self, updated_node: Node) {
        for n in 0 .. self.merged_tree.len() {
            if self.merged_tree[n].id == updated_node.id {
                self.merged_tree[n].arg_flag = updated_node.arg_flag;
                break;
            }
        }
    }

    pub fn update_node_level_in_arena(&mut self, updated_node: Node) {
        for n in 0 .. self.merged_tree.len() {
            if self.merged_tree[n].id == updated_node.id {
                self.merged_tree[n].level = updated_node.level;
                break;
            }
        }
    }

    pub fn init_dummy_node(&mut self) -> Node {
        Node {
              node_type: NodeType::match_none,
              node_value: "dummy".to_string(),
              width: 0,
              id: <usize>::max_value(),
              var_id: None,
              arg_flag: false,
              level: 0,
              next: None,
             }
    }

    pub fn find_node_with_id_in_arena(&mut self, node_id: usize) -> Node {
        // FIXME: not a good implementation to traverse and exit if id is found
        let mut found_node = self.init_dummy_node();
        for n in 0 .. self.merged_tree.len() {
            if self.merged_tree[n].id == node_id {
                found_node = self.merged_tree[n].clone();
                break;
            } else {
                continue;
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

    pub fn is_node_value_already_in_hash_map(&mut self, val: String) -> bool {
        if self.hmap.contains_key(&val) {
            true
        } else {
            false
        }
    }

    pub fn get_next_node_of_single_tree(&mut self, single_tree: Vec<Node>, current: Node) -> Node {
        let mut ret_node = self.init_dummy_node();
        if self.node_has_any_connection(current.clone()) {
            if let Some(nodes_list) = current.next {
                // assuming these are always linear trees with one next entry
                let next_id = nodes_list[0].index;
                for n in 0 .. single_tree.len() {
                    if next_id == single_tree[n].id {
                        ret_node = single_tree[n].clone();
                        break;
                    }
                }
            }
        }
        ret_node
    }

    // merged tre's nodes may have more than one next nodes
    pub fn get_next_node_of_merged_tree(&mut self, current: Node) -> Vec<Node> {
        let mut ret_nodes: Vec<Node> = Vec::new();
        if self.node_has_any_connection(current.clone()) {
            if let Some(nodes_list) = current.next {
                for n in 0 .. nodes_list.len() {
                    let next_id = nodes_list[n].index;
                    ret_nodes.push(self.find_node_with_id_in_arena(next_id));
                }
            }
        } else {
            ret_nodes.push(self.init_dummy_node());
        }
        ret_nodes
    }

    pub fn is_node_dummy(&mut self, node: Node) -> bool {
        if node.node_value == "dummy".to_string() {
            true
        } else {
            false
        }
    }

    pub fn add_node_in_arena(&mut self, node: Node) {
        self.merged_tree.push(node);
    }

    pub fn are_node_values_same(&mut self, node1: Node, node2: Node) -> bool {
        if node1.node_value == node2.node_value {
            // specific case for Var type nodes
            // compare the width of vars first
            // then, compare the var_id (b/c all vars are given var_number while parsing)
            if node1.node_value == "Var".to_string() {
                if node1.width == node2.width {
                    if node1.var_id == node2.var_id {
                        //println!("************Yayy!! Matched the variables here ************\n");
                        true
                    } else {
                        //println!("************Noo!! Didn't Match the variables here ************\n");
                        false
                    }
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }
}

pub fn generate_merged_prefix_tree(
    single_tree: Vec<Node>,
    mut merged_arena: MergedArena
) -> MergedArena {

    if merged_arena.merged_tree.len() == 0 {
        let root_node = merged_arena.build_root_node();
        merged_arena.add_node_to_arena(root_node);
    }

    let top_node =
        merged_arena.get_top_node_of_opt_pattern(
            single_tree.clone());
    let top_val =
        merged_arena.get_value_of_node(top_node.clone());
    let top_id =
        merged_arena.get_id_of_node(top_node.clone());

    let found_root = merged_arena.find_node_with_id_in_arena(0);
    if merged_arena.is_node_dummy(found_root.clone()) {
        panic!("Error: the node is expected to be found in merged arena");
    }

    if !merged_arena.node_has_any_connection(found_root.clone()) {
        // case 1
        //println!("No connection of root node ------------");
        merged_arena.update_hash_map(top_val, top_id);
        let updated_root = merged_arena.update_next_nodes_list(found_root.clone(), top_id);
        merged_arena.update_node_in_arena(updated_root);
        for n in 0 .. single_tree.len() {
            merged_arena.add_node_to_arena(single_tree[n].clone());
        }
    } else {
        //println!("found connection of root node ------------");
        //println!("current val for opt2 = {}", top_val);
        if merged_arena.is_node_value_already_in_hash_map(top_val.clone()) {
            // case 2
            //println!("Hash key found");

            // get the id of existing hashkey
            let found_id = merged_arena.hmap[&top_val];

            // find the node with id = found_id in arena
            // set the mered tree tracking point
            let mut mtrack = merged_arena.find_node_with_id_in_arena(found_id);
            if merged_arena.is_node_dummy(found_root.clone()) {
                panic!("Error: the node is expected to be found in merged arena, since its also added in hashmap");
            }

            // set a previous tracking point in merged tree,
            // that allows to add new connections
            let mut prev = mtrack.clone();

            // keep a tracker for nodes in single_tree (current optimization pattern)
            let mut strack = top_node;

            // Loop and compare the merged-tree nodes values with single-tree nodes vals
            // one-by-one and, make a decision of when to append the nodes from given
            // single-tree optimization pattern
            let mut merged_next_nodes: Vec<Node> = Vec::new();
            loop {
                //////////////////////// TASK 1
                strack = merged_arena.get_next_node_of_single_tree(single_tree.clone(), strack.clone());
                merged_next_nodes = merged_arena.get_next_node_of_merged_tree(mtrack.clone());

                // check for dummy nodes in single/merged tree

                // if merged tree got a dummy next node, its going to be the only one
                // in this merged_next_nodes: vec<Node> list

                //////////////////////// TASK 2
                if merged_arena.is_node_dummy(merged_next_nodes[0].clone()) &&
                   !merged_arena.is_node_dummy(strack.clone()) {
                       //
                       //
                       //  update prev->next with strack.id
                       //  update prev node in arena
                       //  loop {
                       //   if strack is dummy {
                       //       break;
                       //   }
                       //   add_to_arena(strack);
                       //   strack = get_next_in_single_tree();
                       //  }
                       //
                       //
                       prev = merged_arena.update_next_nodes_list(prev, strack.id);
                       merged_arena.update_node_in_arena(prev.clone());
                       loop {
                           if merged_arena.is_node_dummy(strack.clone()) {
                               break;
                           } else {
                               merged_arena.add_node_in_arena(strack.clone());
                               strack = merged_arena.get_next_node_of_single_tree(single_tree.clone(), strack.clone());
                               continue;
                           }
                       }
                }

                //////////////////////// TASK 3
                // if single tree got a dummy next node, this means the current single tree
                // pattern already exists in merged tree, no need to append any new nodes
                // and, just exit
                if merged_arena.is_node_dummy(strack.clone()) {
                    break;
                }

                //////////////////////// TASK 4
                // compare the node values
                //
                // let mut flag = false;
                // for mt in mt_list {
                //  if merged_arena.are_node_values_same(mt.node_value, strack.node_value) {
                //      prev = mt;
                //      flag = true;
                //      break;
                //  } else {
                //      continue;
                //  }
                // }
                //
                //
                let mut same_nodes = false;
                for mnode in 0 .. merged_next_nodes.len() {
                    if merged_arena.are_node_values_same(merged_next_nodes[mnode].clone(), strack.clone()) {
                        prev = merged_next_nodes[mnode].clone();
                        mtrack = merged_next_nodes[mnode].clone();
                        same_nodes = true;
                        break;
                    } else {
                        continue;
                    }
                }

                //////////////////////// TASK 5
                //if flag {
                //    continue;
                //} else {
                //    repeat task 2
                //}
                //
                //
                if same_nodes {
                    continue;
                } else {
                    prev = merged_arena.update_next_nodes_list(prev, strack.id);
                    merged_arena.update_node_in_arena(prev.clone());
                    loop {
                        if merged_arena.is_node_dummy(strack.clone()) {
                            break;
                        } else {
                            merged_arena.add_node_in_arena(strack.clone());
                            strack = merged_arena.get_next_node_of_single_tree(single_tree.clone(), strack.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            // case 3 - same as case 1
            merged_arena.update_hash_map(top_val, top_id);
            let updated_root = merged_arena.update_next_nodes_list(found_root.clone(), top_id);
            merged_arena.update_node_in_arena(updated_root);
            for n in 0 .. single_tree.len() {
                merged_arena.add_node_to_arena(single_tree[n].clone());
            }
        }
    }

    merged_arena
}
