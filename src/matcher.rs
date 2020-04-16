// Matcher

use cliftinstbuilder::{self, CtonInst};
use lhspatternmatcher::{self, Node, NodeType};
use mergedtree::MergedArena;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Opt {
    current_entity: String,
    func_str: String,
    scope_stack: Vec<ScopeStack>,
    const_stack: Vec<String>,
    const_table: HashMap<usize, String>,
}

#[derive(Clone)]
pub struct ScopeStack {
    scope_type: ScopeType,
    level: usize,
}

#[derive(Clone)]
pub enum ScopeType {
    ScopeMatch,
    ScopeCase,
    ScopeFunc,
}

impl Opt {
    pub fn new() -> Opt {
        Opt {
            current_entity: String::from("inst"),
            func_str: String::from(""),
            scope_stack: Vec::new(),
            const_stack: Vec::new(),
            const_table: HashMap::new(),
        }
    }

    pub fn generate_header(&mut self) {
        self.func_str
            .push_str("fn matcher(pos: &mut FuncCursor, inst: Inst)");
    }

    pub fn append(&mut self, input: String) {
        self.func_str.push_str(&input);
    }

    pub fn set_entity(&mut self, entity: String) {
        self.current_entity = entity;
    }

    pub fn does_level_exist_in_stack(&mut self, find_level: usize) -> usize {
        let mut index = 0;
        for i in 0..self.scope_stack.len() {
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
        for _ in from..self.scope_stack.len() {
            if let Some(elem) = self.scope_stack.pop() {
                self.exit_scope(elem.scope_type, elem.level);
            }
        }
    }

    pub fn get_const_arg_from_hash_table(&mut self, level: usize) -> String {
        match self.const_table.get(&level) {
            Some(rhs) => rhs.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn add_const_arg_to_hash_table(&mut self, arg: String, level: usize) {
        self.const_table.insert(level, arg);
    }

    pub fn push_to_const_stack(&mut self, rhs: String) {
        self.const_stack.push(rhs);
    }

    pub fn pop_from_const_stack(&mut self) -> String {
        match self.const_stack.pop() {
            Some(rhs) => rhs.to_string(),
            None => "".to_string(),
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
        self.scope_stack.push(ScopeStack {
            scope_type: scope.clone(),
            level: current_level,
        });
        // append the string
        match scope {
            ScopeType::ScopeMatch => {
                self.append(String::from(" {\n"));
            }
            ScopeType::ScopeFunc => {
                self.append(String::from(" {\n"));
            }
            ScopeType::ScopeCase => {
                self.append(String::from(" => {\n"));
            }
        }
    }

    pub fn exit_scope(&mut self, scope: ScopeType, _level: usize) {
        match scope {
            ScopeType::ScopeMatch => {
                self.append(String::from("\n}"));
            }
            ScopeType::ScopeFunc => {
                self.append(String::from("\n}"));
            }
            ScopeType::ScopeCase => {
                self.append(String::from("\n},"));
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_leaf_node(&mut self, node: Node) -> bool {
        //println!("check leaf node =========\n\n");
        node.next.is_none()
    }

    pub fn set_level_of_all_child_nodes(
        &mut self,
        arena: &mut MergedArena,
        n: usize,
        current: usize,
    ) {
        if let Some(next_nodes) = arena.merged_tree[n].next.clone() {
            for n in 0..next_nodes.len() {
                let id = next_nodes[n].index;
                let next_node = arena.find_node_with_id_in_arena(id);
                let updated_node = arena.update_node_with_level(next_node.clone(), current + 1);
                arena.update_node_level_in_arena(updated_node.clone());
            }
        }
    }

    pub fn take_action(&mut self, rhs: Vec<CtonInst>) {
        // Debug logs for RHS
        // println! ("========   RHS insts  =======\n");
        // for i in 0 .. rhs.len() {
        //     let isa = rhs[i].clone();
        //     println!("inst = {}\n", cliftinstbuilder::get_clift_opcode_name(isa.opcode));
        //     match isa.cops {
        //         Some(ops) => {
        //             for op in ops {
        //                 match op.const_val {
        //                     Some(c) => {
        //                         println!("Result const op ==== {}\n", c);
        //                     },
        //                     None => {},
        //                 }
        //             }
        //         },
        //         None => {},
        //     }
        // }
        // Special Case: what to do for RHS with 1 inst only?
        // For example: result 20:i32 (it simply returns a constant)
        if rhs.len() == 1 {
            let each_inst = rhs[0].clone();
            let mut rhs_const: i32 = 0;

            match each_inst.cops {
                Some(ops) => {
                    for op in ops {
                        match op.const_val {
                            Some(c) => {
                                rhs_const = c;
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }

            let mut replace_inst_str = "pos.func.dfg.replace(".to_owned();
            // FIXME: fix the inst name here
            replace_inst_str += &"inst".to_owned();
            replace_inst_str += &").".to_owned();
            replace_inst_str += &"iconst(".to_owned();
            // FIXME: fix the width part
            // Note: Width is not set in CliftInst operands
            // for constant type, only const_val exists.
            // Only parser has the correct width for constant, see
            // how you pass that information further to CliftInst struct
            replace_inst_str += &"width".to_owned();
            replace_inst_str += &", ".to_owned();
            replace_inst_str += &rhs_const.to_string();
            replace_inst_str += &"); ".to_owned();
            self.func_str.push_str(&replace_inst_str);
        } else {
            for inst in 0..rhs.len() - 2 {
                let each_inst = rhs[inst].clone();
                let mut insert_inst_str = "let inst".to_owned();
                insert_inst_str += &inst.to_string();
                insert_inst_str += &" = pos.ins().".to_owned();
                insert_inst_str += &cliftinstbuilder::get_clift_opcode_name(each_inst.opcode);
                insert_inst_str += &"(".to_owned();
                // FIXME: fix the args names and count of args here
                insert_inst_str += &"args[0], args[1]".to_owned();
                insert_inst_str += &");\n".to_owned();
                self.func_str.push_str(&insert_inst_str);
            }
            for inst in rhs.len() - 2..rhs.len() - 1 {
                let each_inst = rhs[inst].clone();
                let mut replace_inst_str = "pos.func.dfg.replace(".to_owned();
                // FIXME: fix the inst name here
                replace_inst_str += &"inst".to_owned();
                replace_inst_str += &").".to_owned();
                replace_inst_str += &cliftinstbuilder::get_clift_opcode_name(each_inst.opcode);
                replace_inst_str += &"(".to_owned();
                // FIXME: fix the args names and count of args here
                replace_inst_str += &"args[0], args[1]".to_owned();
                replace_inst_str += &");\n".to_owned();
                self.func_str.push_str(&replace_inst_str);
            }
        }
    }

    pub fn get_argument_counter(&mut self, mut count: u32) -> u32 {
        count = count + 1;
        self.append(String::from(count.to_string()));
        count
    }

    pub fn get_const_counter(&mut self, mut count: u32) -> u32 {
        count = count + 1;
        count
    }
}

pub fn is_node_actionable(node_id: usize, table: HashMap<usize, Vec<CtonInst>>) -> bool {
    if table.contains_key(&node_id) {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
pub enum IntCC {
    Equal,
    NotEqual,
    SignedLessThan,
    SignedGreaterThanOrEqual,
    SignedGreaterThan,
    SignedLessThanOrEqual,
    UnsignedLessThan,
    UnsignedGreaterThanOrEqual,
    UnsignedGreaterThan,
    UnsignedLessThanOrEqual,
    Overflow,
    NotOverflow,
}

pub fn get_cond_name(cmp: String) -> String {
    let cond = match cmp.as_ref() {
        "eq" => "IntCC::Equal".to_string(),
        "ne" => "IntCC::NotEqual".to_string(),
        "slt" => "IntCC::SignedLessThan".to_string(),
        "ult" => "IntCC::UnsignedLessThan".to_string(),
        "sle" => "IntCC::SignedLessThanOrEqual".to_string(),
        "ule" => "IntCC::UnsignedLessThanOrEqual".to_string(),
        // Souper does not generated ygt, sgt, sge, uge,
        // overflow, not overflow - conditions
        _ => "".to_string(),
    };
    cond
}

pub fn generate_matcher(mut arena: MergedArena, rhs: HashMap<usize, Vec<CtonInst>>) -> String {
    let mut opt_func = Opt::new();
    let mut arg_str = String::from("");
    let mut arg_counter: u32 = 0;
    let mut const_counter: u32 = 0;

    for node in 0..arena.merged_tree.len() {
        let action_flag = is_node_actionable(arena.merged_tree[node].id, rhs.clone());
        // dump: begin
        println!("Node ==== ===================================");
        println!("\t\t Node Id = {}", arena.merged_tree[node].id);
        println!("\t\t Actionable? = {}", action_flag);
        match arena.merged_tree[node].clone().var_id {
            Some(x) => {
                println!("Var number = {}", x);
            },
            None => {
                println!("Var number = None\n");
            },
        }
        println!(
            "\t\t Node Type = {}",
            lhspatternmatcher::get_node_type(arena.merged_tree[node].clone().node_type)
        );
        println!("\t\t Node Level = {}", arena.merged_tree[node].level);
        println!("\t\t Node Value = {}", arena.merged_tree[node].node_value);
        match arena.merged_tree[node].next.clone() {
            Some(ids) => {
                for i in 0..ids.len() {
                    println!("\t\t Node->next = {}", ids[i].index);
                }
            }
            None => println!("No next\n"),
        }
        // dump: end
        match arena.merged_tree[node].node_type {
            NodeType::MatchRoot => {
                opt_func.generate_header();
                let current_level = arena.merged_tree[node].level;
                opt_func.enter_scope(ScopeType::ScopeFunc, current_level);
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::MatchInstData => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);

                let opt_clone = opt_func.clone();
                let ent = opt_clone.current_entity;
                if !ent.is_empty() {
                    opt_func.append(String::from("match pos.func.dfg"));
                    opt_func.append(String::from("["));
                    // FIXME: Connect this ent string with RHS replacement part
                    opt_func.append(ent);
                    opt_func.append(String::from("]"));
                    opt_func.enter_scope(ScopeType::ScopeMatch, current_level);
                }
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::InstType => {
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
                    "Var" => {}
                    "Binary" => {
                        // FIXME: "args" part, make a connection between actual args and string
                        opt_func.append(String::from("InstructionData::Binary { opcode, args }"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        // FIXED: Generate: "let args_<counter> = args;"
                        opt_func.append(String::from("let args_"));
                        arg_counter = opt_func.get_argument_counter(arg_counter);
                        opt_func.append(String::from(" = args;\n"));
                    }
                    "IntCompare" => {
                        // FIXME: "args" part, make a connection
                        // between actual args and string
                        opt_func.append(String::from(
                            "InstructionData::IntCompare { opcode, cond, args }",
                        ));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        // FIXED: Generate: "let args_<counter> = args;"
                        opt_func.append(String::from("let args_"));
                        arg_counter = opt_func.get_argument_counter(arg_counter);
                        opt_func.append(String::from(" = args;\n"));
                    }
                    "Unary" => {
                        // FIXME: "arg" part, make a connection b/w actual args and string
                        opt_func.append(String::from("InstructionData::Unary { opcode, arg }"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        // FIXED: Generate: "let args_<counter> = arg;"
                        opt_func.append(String::from("let args_"));
                        arg_counter = opt_func.get_argument_counter(arg_counter);
                        opt_func.append(String::from(" = arg;\n"));
                    }
                    "UnaryImm" => {
                        opt_func.append(String::from("InstructionData::UnaryImm { opcode, imm }"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        const_counter = opt_func.get_const_counter(const_counter);
                        let mut rhs_arg = "rhs_".to_string();
                        rhs_arg.push_str(&const_counter.to_string());
                        opt_func.append(String::from("let "));
                        opt_func.append(String::from(rhs_arg.to_string()));
                        opt_func.append(String::from(" : i64 = imm.into();\n"));
                        opt_func.push_to_const_stack(rhs_arg.to_string());
                    }
                    "BinaryImm" => {
                        // FIXME: "args" part, make a connection between actual args and string
                        opt_func.append(String::from(
                            "InstructionData::BinaryImm { opcode, arg, imm }",
                        ));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        // FIXED: Generate: "let args_<counter> = args;"
                        opt_func.append(String::from("let args_"));
                        arg_counter = opt_func.get_argument_counter(arg_counter);
                        opt_func.append(String::from(" = arg;\n"));
                        // Push the rhs_<count> to ConstStack
                        const_counter = opt_func.get_const_counter(const_counter);
                        let mut rhs_arg = "rhs_".to_string();
                        rhs_arg.push_str(&const_counter.to_string());
                        opt_func.append(String::from("let "));
                        opt_func.append(String::from(rhs_arg.to_string()));
                        opt_func.append(String::from(" : i64 = imm.into();\n"));
                        opt_func.push_to_const_stack(rhs_arg.to_string());
                    }
                    "IntCompareImm" => {
                        // FIXME: "args" part, make a connection
                        // between actual args and string
                        opt_func.append(String::from(
                            "InstructionData::IntCompareImm { opcode, cond, arg, imm }",
                        ));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        // FIXED: Generate: "let args_<counter> = args;"
                        opt_func.append(String::from("let args_"));
                        arg_counter = opt_func.get_argument_counter(arg_counter);
                        opt_func.append(String::from(" = arg;\n"));
                        // Push the rhs_<count> to ConstStack
                        const_counter = opt_func.get_const_counter(const_counter);
                        let mut rhs_arg = "rhs_".to_string();
                        rhs_arg.push_str(&const_counter.to_string());
                        opt_func.append(String::from("let "));
                        opt_func.append(String::from(rhs_arg.to_string()));
                        opt_func.append(String::from(" : i64 = imm.into();\n"));
                        opt_func.push_to_const_stack(rhs_arg.to_string());
                    }
                    _ => {
                        panic!("Error: This instruction data type is not yet handled");
                    }
                }
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::MatchValDef => {
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
                    "Param" => {
                        // Reset the argument match string to empty str
                        // so that for further args, it's not appended.
                        arg_str = String::from("");
                        opt_func.set_entity(String::from(""));
                    }
                    "Result" => {
                        // for Result type, we want to generate arg match part,
                        // so, append it.
                        opt_func.append(arg_str.clone());
                        arg_str = String::from("");
                        opt_func.enter_scope(ScopeType::ScopeMatch, current_level - 1);
                        opt_func.append(String::from("\nValueDef::"));
                        opt_func.append(String::from("Result(arg_ty, _)"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("arg_ty"));
                    }
                    _ => {
                        // FIXME - do we want error handling here for NoneType and ""
                        println!("\t\t entering unknown valdef case\n");
                    }
                }
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::MatchOpcode => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                let opt_clone = opt_func.clone();
                let ent = opt_clone.current_entity;
                // FIXME: Any purpose of ent here?
                if !ent.is_empty() {
                    opt_func.append(String::from("match opcode"));
                    opt_func.enter_scope(ScopeType::ScopeMatch, current_level);
                }
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::Opcode => {
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
                    "Var" => {}
                    "iadd" => {
                        opt_func.append(String::from("Opcode::Iadd"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "iadd_imm" => {
                        opt_func.append(String::from("Opcode::IaddImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "imul" => {
                        opt_func.append(String::from("Opcode::Imul"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "imul_imm" => {
                        opt_func.append(String::from("Opcode::ImulImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "isub" => {
                        opt_func.append(String::from("Opcode::Isub"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "irsub_imm" => {
                        opt_func.append(String::from("Opcode::IsubImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "icmp" => {
                        opt_func.append(String::from("Opcode::Icmp"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "icmp_imm" => {
                        opt_func.append(String::from("Opcode::IcmpImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "band" => {
                        opt_func.append(String::from("Opcode::Band"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "band_imm" => {
                        opt_func.append(String::from("Opcode::BandImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "bor" => {
                        opt_func.append(String::from("Opcode::Bor"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "bor_imm" => {
                        opt_func.append(String::from("Opcode::BorImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "bxor" => {
                        opt_func.append(String::from("Opcode::Bxor"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "bxor_imm" => {
                        opt_func.append(String::from("Opcode::BxorImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "ishl" => {
                        opt_func.append(String::from("Opcode::Ishl"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "ishl_imm" => {
                        opt_func.append(String::from("Opcode::IshlImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "sshr" => {
                        opt_func.append(String::from("Opcode::Sshr"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "sshr_imm" => {
                        opt_func.append(String::from("Opcode::SshrImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "ushr" => {
                        opt_func.append(String::from("Opcode::Ushr"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "ushr_imm" => {
                        opt_func.append(String::from("Opcode::UshrImm"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "popcnt" => {
                        opt_func.append(String::from("Opcode::Popcnt"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "clz" => {
                        opt_func.append(String::from("Opcode::Clz"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "ctz" => {
                        opt_func.append(String::from("Opcode::Ctz"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    "iconst" => {
                        opt_func.append(String::from("Opcode::Iconst"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    _ => {
                        panic!("Error: this opcode type is not yet handled");
                    }
                }
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::MatchCond => {
                let current_level = arena.merged_tree[node].level;
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                let opt_clone = opt_func.clone();
                let ent = opt_clone.current_entity;
                // FIXME: Any purpose of ent here?
                if !ent.is_empty() {
                    opt_func.append(String::from("match cond"));
                    opt_func.enter_scope(ScopeType::ScopeMatch, current_level);
                }
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::Cond => {
                let current_level = arena.merged_tree[node].level;
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                // Check if there is any child node already
                // matched at same level
                // If yes, pop and exit scope first, and
                // then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                // match the actual opcode types
                match arena.merged_tree[node].node_value.as_ref() {
                    "eq" | "ne" | "ult" | "ule" | "slt" | "sle" => {
                        let cond = get_cond_name(arena.merged_tree[node].clone().node_value);
                        opt_func.append(cond);
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    _ => {
                        panic!("Error: this condition type is not yet handled");
                    }
                }
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::MatchArgs => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                // Create an optional argument matching string here
                // we will decide later whether we need this match on args or not
                // depending on if the argument type is Result or Param. Param
                // type does not need this match part at all.
                arg_str.push_str(&(String::from("match pos.func.dfg.value_def")));
                arg_str.push_str(&(String::from("(")));
                // make string like: args_2 or args_2[0] depending on binaryImm or binary
                let arg_node_val = arena.merged_tree[node].node_value.clone();
                arg_str.push_str(&(String::from("args_")));
                arg_str.push_str(&(String::from(arg_counter.to_string())));
                if let Some(i) = arg_node_val.find('[') {
                    arg_str.push_str(&(arena.merged_tree[node].node_value.clone())[i..]);
                }
                arg_str.push_str(&(String::from(")")));
                // FIXME: Do we want to take action here and should we
                // append to arg_str, or opt_func?
            }
            NodeType::MatchPlainConst => {
                let current_level = arena.merged_tree[node].level;
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            NodeType::MatchConst => {
                let current_level = arena.merged_tree[node].level;
                opt_func.set_level_of_all_child_nodes(&mut arena, node, current_level);
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                let const_value = &arena.merged_tree[node].node_value;
                // FIXME: fix width of the constant in rhs part
                // Check Cranelift's instructions specifications

                let mut rhs_arg = opt_func.get_const_arg_from_hash_table(current_level);
                match rhs_arg.as_ref() {
                    "" => {
                        rhs_arg = opt_func.pop_from_const_stack();
                        opt_func.add_const_arg_to_hash_table(rhs_arg.clone(), current_level);
                        opt_func.append(String::from("if "));
                    }
                    _ => {
                        opt_func.append(String::from("else if "));
                    }
                }
                opt_func.append(String::from(rhs_arg.to_string()));
                opt_func.append(String::from(" == "));
                opt_func.append(const_value.to_string());
                opt_func.enter_scope(ScopeType::ScopeFunc, current_level);
                if action_flag {
                    let found_rhs = &rhs[&arena.merged_tree[node].id];
                    opt_func.take_action(found_rhs.to_vec());
                }
            }
            _ => {
                panic!("\n\nmatch type not handled yet!\n");
            }
        }
    }

    // exit func scope
    while let Some(elem) = opt_func.scope_stack.pop() {
        opt_func.exit_scope(elem.scope_type, elem.level);
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
