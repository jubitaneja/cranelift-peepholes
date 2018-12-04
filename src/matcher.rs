// Matcher

use mergedtree::{self, MergedArena};
use patternmatcher::{self, Node, NodeType, NodeID};

#[derive(Clone)]
pub struct Opt {
    current_entity: String,
    func_str: String,
    //scope_stack: Vec<ScopeType>,
    scope_stack: Vec<ScopeStack>,
}

#[derive(Clone)]
pub struct ScopeStack {
    scope_type: ScopeType,
    level: usize,
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

    pub fn does_level_exist_in_stack(&mut self, find_level: usize) -> usize {
        let mut index = 0;
        for i in 0 .. self.scope_stack.len() {
            if find_level == self.scope_stack[i].level {
                index = i;
                break;
            } else {
                continue;
            }
        }
        index
    }

    pub fn pop_and_exit_scope_from(&mut self, from: usize) {
        for i in from .. self.scope_stack.len() {
            let stack_elem = self.scope_stack.pop();
            match stack_elem {
                Some(elem) => {
                    self.exit_scope(elem.scope_type, elem.level);
                },
                None => {},
            }
        }
    }

    pub fn enter_scope(&mut self, scope: ScopeType, current_level: usize) {
        // check for the level in current stack
        // if level is new - not found in stack - push it directly.
        // if level already exists in stack, pop the stack until that level and then
        // push the new level.
        //println!("Current stack before entering scope is: -------");
        //for x in 0 .. self.scope_stack.len() {
           //println!("stack levels pushed so far = {}", self.scope_stack[x].level);
        //}
        //println!("find the level number = {} in stack", current_level);
        let index = self.does_level_exist_in_stack(current_level);
        //println!("Found index from stack == {}", index);
        if index != 0 {
            // index exists
            // pop first
            self.pop_and_exit_scope_from(index);
        }
        // push the level
        self.scope_stack.push(ScopeStack {scope_type: scope.clone(), level: current_level });
        // append the string
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

    pub fn exit_scope(&mut self, scope: ScopeType, level: usize) {
        match scope {
            ScopeType::scope_match => {
                self.append(String::from("\n}"));
            },
            ScopeType::scope_func => {
                self.append(String::from("\n}"));
            },
            ScopeType::scope_case => {
                self.append(String::from("\n},"));
            },
            _ => {
                panic!("Error: No such scope type exists");
            },
        }
    }

    pub fn is_leaf_node(&mut self, node: Node) -> bool {
        //println!("check leaf node =========\n\n");
        match node.next {
            Some(x) => false,
            None => true,
        }
    }

    pub fn set_level_of_all_child_nodes(&mut self, arena: &mut MergedArena, n: usize, current: usize) {
        if let Some(next_nodes) = arena.merged_tree[n].next.clone() {
            for n in 0 .. next_nodes.len() {
                let id = next_nodes[n].index;
                let mut next_node = arena.find_node_with_id_in_arena(id);
                let updated_node = arena.update_node_with_level(next_node.clone(), current + 1);
                arena.update_node_level_in_arena(updated_node.clone());
            }
        }
    }
}

pub fn generate_matcher(mut arena: MergedArena) -> String {
    let mut opt_func = Opt::new();

    for node in 0 .. arena.merged_tree.len() {
        // dump: begin
        println!("Node ==== ============================================================");
        println!("\t\t Node Id = {}", arena.merged_tree[node].id);
        println!("\t\t Node Level = {}", arena.merged_tree[node].level);
        println!("\t\t Node Level = {}", arena.merged_tree[node].node_value);
        match arena.merged_tree[node].next.clone() {
            Some(ids) => {
                for i in 0 .. ids.len() {
                    println!("\t\t Node->next = {}", ids[i].index);
                }
            },
            None => {
                println!("No next\n")
            },
        }
        // dump: end
        match arena.merged_tree[node].node_type {
            NodeType::match_root => {
                opt_func.generate_header();
                let current_level = arena.merged_tree[node].level;
                opt_func.enter_scope(ScopeType::scope_func, current_level);
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
            },
            NodeType::match_instdata => {
                //println!("\t\t Instdata node type");
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
               
                let mut opt_clone = opt_func.clone();
                let mut ent = opt_clone.current_entity;
                if !ent.is_empty() {
                    opt_func.append(String::from("match pos.func.dfg"));
                    opt_func.append(String::from("["));
                    opt_func.append(ent);
                    opt_func.append(String::from("]"));
                    opt_func.enter_scope(ScopeType::scope_match, current_level);
                }
            },
            NodeType::inst_type => {
                //println!("\t\tSpecific instruction type node\n");
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                // Check if there is any child node already matched at same level
                // If yes, pop and exit scope first, and then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                match arena.merged_tree[node].node_value.as_ref() {
                    "Var" => {},
                    "Binary" => {
                        opt_func.append(String::from("InstructionData::Binary { opcode, args }"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                        opt_func.set_entity(String::from("opcode"));
                    },
                    "Unary" => {
                        opt_func.append(String::from("InstructionData::Unary { opcode, arg }"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                        opt_func.set_entity(String::from("opcode"));
                    },
                    _ => {
                        panic!("Error: This instruction data type is not yet handled");
                    },
                }
            },
            NodeType::match_valdef => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                // Check if there is any child node already matched at same level
                // If yes, pop and exit scope first, and then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                opt_func.append(String::from("\nValDef::"));
                match arena.merged_tree[node].node_value.as_ref() {
                    "Param" => {
                        //println!("\t\t\t entering valdef::param here");
                        opt_func.append(String::from("Param(_, _)"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                        opt_func.set_entity(String::from(""));
                    },
                    "Result" => {
                        //println!("\t\t\t entering valdef::result here");
                        opt_func.append(String::from("Result(arg_ty, _)"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                        opt_func.set_entity(String::from("arg_ty"));
                    },
                    _ => {
                        // FIXME - do we want error handling here for NoneType and ""
                        println!("\t\t entering unknown valdef case\n");
                    },
                }
            },
            NodeType::match_opcode => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                let mut opt_clone = opt_func.clone();
                let mut ent = opt_clone.current_entity;
                if !ent.is_empty() {
                    opt_func.append(String::from("match opcode"));
                    opt_func.enter_scope(ScopeType::scope_match, current_level);
                }
            },
            NodeType::opcode => {
                //println!("\t\tIn specific opcode case in matcher\n");
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                // Check if there is any child node already matched at same level
                // If yes, pop and exit scope first, and then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                // match the actual opcode types
                match arena.merged_tree[node].node_value.as_ref() {
                    "Var" => {},
                    "Iadd" => {
                        opt_func.append(String::from("Opcode::Iadd"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "Imul" => {
                        opt_func.append(String::from("Opcode::Imul"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "Isub" => {
                        opt_func.append(String::from("Opcode::Isub"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "icmpeq" => {
                        opt_func.append(String::from("Opcode::Eq"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "icmpne" => {
                        opt_func.append(String::from("Opcode::Ne"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "icmpslt" => {
                        opt_func.append(String::from("Opcode::Slt"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "icmpult" => {
                        opt_func.append(String::from("Opcode::Ult"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "icmpsle" => {
                        opt_func.append(String::from("Opcode::Sle"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "icmpule" => {
                        opt_func.append(String::from("Opcode::Ule"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "band" => {
                        opt_func.append(String::from("Opcode::Band"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "bor" => {
                        opt_func.append(String::from("Opcode::Bor"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "bxor" => {
                        opt_func.append(String::from("Opcode::Bxor"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "ishl" => {
                        opt_func.append(String::from("Opcode::Ishl"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "sshr" => {
                        opt_func.append(String::from("Opcode::Sshr"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "ushr" => {
                        opt_func.append(String::from("Opcode::Ushr"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "popcnt" => {
                        opt_func.append(String::from("Opcode::Popcnt"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "clz" => {
                        opt_func.append(String::from("Opcode::Clz"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    "ctz" => {
                        opt_func.append(String::from("Opcode::Ctz"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    _ => {
                        panic!("Error: this opcode type is not yet handled");
                    },
                }
            },
            NodeType::match_args => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                // create a default match string
                opt_func.append(String::from("match pos.func.dfg.val_def"));
                opt_func.append(String::from("("));
                opt_func.append(arena.merged_tree[node].node_value.clone());
                opt_func.append(String::from(")"));

                opt_func.enter_scope(ScopeType::scope_match, current_level);
            },
            _ => {
                panic!("\n\nmatch type not handled yet!\n");
            },
        }
    }

    // exit func scope
    for s in 0 .. opt_func.scope_stack.len() {
        match opt_func.scope_stack.pop() {
            Some(elem) => {
                opt_func.exit_scope(elem.scope_type, elem.level);
            },
            None => {},
        }
        //let elem_ty = opt_func.scope_stack.pop();
        //match elem_ty {
        //    Some(ty) => {
        //        opt_func.exit_scope(ty);
        //    },
        //    None => {},
        //}
    }

    opt_func.func_str
}
