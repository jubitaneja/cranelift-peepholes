// Matcher

use mergedtree::{self, MergedArena};
use patternmatcher::{self, Node, NodeType, NodeID};

#[derive(Clone)]
pub struct Opt {
    current_entity: String,
    func_str: String,
    scope_stack: Vec<ScopeType>,
}

#[derive(Clone)]
pub enum ScopeType {
    scope_match,
    scope_case,
    scope_func,
}

impl Opt {
    pub fn new() -> Opt {
        Opt {
            current_entity: String::from("inst"),
            func_str: String::from(""),
            scope_stack: Vec::new(),
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

    pub fn enter_scope(&mut self, scope: ScopeType) {
        //FIXME: push the types in scope stack
        match scope {
            ScopeType::scope_match => {
                self.append(String::from(" {\n"));
            },
            ScopeType::scope_func => {
                self.append(String::from(" {\n"));
            },
            ScopeType::scope_case => {
                self.append(String::from(" => {\n"));
            },
            _ => {
                panic!("Error: No such scope type exists");
            },
        }
    }

    pub fn exit_scope(&mut self, scope: ScopeType) {
        //FIXME: pop the types in scope stack
        match scope {
            ScopeType::scope_match => {
                self.append(String::from("\n}\n"));
            },
            ScopeType::scope_func => {
                self.append(String::from("\n}\n"));
            },
            ScopeType::scope_case => {
                self.append(String::from("\n}\n"));
            },
            _ => {
                panic!("Error: No such scope type exists");
            },
        }
    }
}

pub fn generate_matcher(mut arena: MergedArena) -> String {
    let mut opt_func = Opt::new();

    opt_func.generate_header();
    opt_func.enter_scope(ScopeType::scope_func);

    for node in 1 .. arena.merged_tree.len() {
        println!("Node ==== ============================================================");
        println!("\t\t Node Id = {}", arena.merged_tree[node].id);
        match arena.merged_tree[node].node_type {
            NodeType::match_instdata => {
                println!("\t\t Instdata node type");
                if !arena.merged_tree[node].arg_flag {
                    opt_func.append(String::from("match pos.func.dfg"));
                    opt_func.append(String::from("["));
                    let mut opt_clone = opt_func.clone();
                    let mut ent = opt_clone.current_entity;
                    opt_func.append(ent);
                    opt_func.append(String::from("]"));

                    opt_func.enter_scope(ScopeType::scope_match);

                    match arena.merged_tree[node].node_value.as_ref() {
                        // TODO: Add more types of instdata here
                        // FIXME: Later make sure if Var case is handled well.
                        // Example:
                        // %0 = var
                        // infer %0
                        "Var" => {},
                        "Binary" => {
                            opt_func.append(String::from("InstructionData::Binary { opcode, args }"));
                            opt_func.enter_scope(ScopeType::scope_case);
                        },
                        _ => {
                            panic!("Error: This instruction data type is not yet handled");
                        },
                    }
                } else {
                    opt_func.append(String::from("ValDef::"));
                    match arena.merged_tree[node].node_value.as_ref() {
                        "Var" => {
                            println!("\t\t\t entering valdef::param here");
                            opt_func.append(String::from("Param(_, _)"));
                            opt_func.enter_scope(ScopeType::scope_case);
                            opt_func.set_entity(String::from(""));
                        },
                        _ => {
                            println!("\t\t\t entering valdef::result here");
                            opt_func.append(String::from("Result(arg_ty, _)"));
                            opt_func.enter_scope(ScopeType::scope_case);
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
                            opt_func.append(String::from("]"));
                            opt_func.enter_scope(ScopeType::scope_match);
                            //
                            //match node_value
                            // TODO: InstructionData::node_value stuff
                            // enter scope case
                            match arena.merged_tree[node].node_value.as_ref() {
                                // TODO: Add more types of instdata here
                                // FIXME: Later make sure if Var case is handled well.
                                // Example:
                                // %0 = var
                                // infer %0
                                "Var" => {},
                                "Binary" => {
                                    opt_func.append(String::from("InstructionData::Binary { opcode, args }"));
                                    opt_func.enter_scope(ScopeType::scope_case);
                                },
                                _ => {
                                    panic!("Error: This instruction data type is not yet handled");
                                },
                            }
                        },
                    }
                }
            },
            NodeType::match_opcode => {
                // match the actual opcode types
                // FIXME: Later add more opcodes here
                match arena.merged_tree[node].node_value.as_ref() {
                    "Var" => {},
                    "Iadd" => {
                        opt_func.append(String::from("match_opcode"));
                        opt_func.enter_scope(ScopeType::scope_match);
                        opt_func.append(String::from("Opcode::Iadd"));
                        opt_func.enter_scope(ScopeType::scope_case);
                    },
                    _ => {
                        panic!("Error: this opcode type is not yet handled");
                    },
                }
            },
            NodeType::match_args => {
                // create a default match string
                opt_func.append(String::from("match pos.func.dfg.val_def"));
                opt_func.append(String::from("("));
                opt_func.append(arena.merged_tree[node].node_value.clone());
                opt_func.append(String::from(")"));

                opt_func.enter_scope(ScopeType::scope_match);
 
                // set the arg_flag to true for next nodes of match_args
                if let Some(next_nodes) = arena.merged_tree[node].next.clone() {
                    for n in 0 .. next_nodes.len() {
                        let id = next_nodes[n].index;
                        let mut next_node = arena.find_node_with_id_in_arena(id);
                        let updated_node = arena.update_node_with_arg_flag(next_node.clone(), true);
                        arena.update_node_arg_flag_in_arena(updated_node.clone());
                    }
                }
            },
            _ => {
                println!("\n\nmatch type not handled yet!\n");
            },
        }
    }

    // the transformation actions will be inserted here
    opt_func.append(String::from("unimplemented!()"));

    // exit func scope

    opt_func.func_str
}
