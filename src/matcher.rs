// Matcher

use mergedtree::{self, MergedArena};
use patternmatcher::{self, Node, NodeType, NodeID};

#[derive(Clone)]
pub struct Opt {
    current_entity: String,
    func_str: String,
}

impl Opt {
    pub fn new() -> Opt {
        Opt {
            current_entity: String::from("inst"),
            func_str: String::from(""),
        }
    }

    pub fn generate_header(&mut self) {
        self.func_str.push_str("fn matcher(pos: &mut FuncCursor, inst: Inst)");
    }

    pub fn append(&mut self, input: String) {
        self.func_str.push_str(&input);
    }

    pub fn set_entity(&mut self, entity: String) {
        self.current_entity = entity;
    }
}

pub fn generate_matcher(mut arena: MergedArena) -> String {
    let mut opt_func = Opt::new();

    opt_func.generate_header();
    // enter func scope

    for node in 0 .. arena.merged_tree.len() {
        match arena.merged_tree[node].node_type {
            NodeType::match_instdata => {
                let mut arg_flag = false;
                //FIXME: add this arg_flag in node struct
                if !arg_flag {
                    opt_func.append(String::from("match pos.func.dfg"));
                    opt_func.append(String::from("["));
                    let mut opt_clone = opt_func.clone();
                    let mut ent = opt_clone.current_entity;
                    opt_func.append(ent);
                    //opt_func.append(opt_func.current_entity.clone());
                    opt_func.append(String::from("]"));
                    // enter scope
                } else {
                    opt_func.append(String::from("ValDef::"));
                    match arena.merged_tree[node].node_value.as_ref() {
                        "Var" => {
                            opt_func.append(String::from("Param(_, _)"));
                            // =>
                            // enter scope
                            opt_func.set_entity(String::from(""));
                        },
                        _ => {
                            opt_func.append(String::from("Result(arg_ty, _)"));
                            // =>
                            // enter scope
                            opt_func.set_entity(String::from("arg_ty"));
                        },
                    }
                    match opt_func.current_entity.as_ref() {
                        "" => {},
                        _ => {
                            opt_func.append(String::from("match pos.func.dfg"));
                            opt_func.append(String::from("["));
                            let mut opt_clone = opt_func.clone();
                            let mut ent = opt_clone.current_entity;
                            opt_func.append(ent);
                            //opt_func.append(opt_func.current_entity.clone());
                            opt_func.append(String::from("]"));
                        },
                    }
                }
            },
            NodeType::match_opcode => {
                println!("\n\n opcode case not yet handled");
            },
            NodeType::match_args => {
                opt_func.append(String::from("match pos.func.dfg.val_def"));
                opt_func.append(String::from("("));
                opt_func.append(arena.merged_tree[node].node_value.clone());
                opt_func.append(String::from(")"));
                //append "("
                //append: (args[0])
                //append ")"
                if let Some(next_nodes) = arena.merged_tree[node].next.clone() {
                    for n in 0 .. next_nodes.len() {
                        let id = next_nodes[n].index;
                        let mut next_node = arena.find_node_with_id_in_arena(id);
                        let updated_node = arena.update_node_with_arg_flag(next_node.clone(), true);
                        arena.update_node_in_arena(updated_node);
                    }
                }
            },
            _ => {
                println!("\n\nmatch type not handled yet!\n");
            },
        }
    }

    // exit func scope

    opt_func.func_str
}
