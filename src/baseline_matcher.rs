// Matcher

use cliftinstbuilder::{self};
use processrhs::CliftInstWithArgs;
use lhspatternmatcher::{self, Node, NodeType};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Opt {
    current_entity: String,
    func_str: String,
    scope_stack: Vec<ScopeStack>,
    const_stack: Vec<String>,
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
        }
    }

    pub fn generate_header(&mut self, count: u32) {
        self.func_str.push_str("fn superopt_");
        self.func_str.push_str(&count.to_string());
        self.func_str.push_str("(pos: &mut FuncCursor, inst: Inst)");
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
        // if level is new - not found in stack -
        // push it directly.
        // if level already exists in stack,
        // pop the stack until that level and then
        // push the new level.
        // Debug
        // println!("\n************ Current stack before entering scope is: \n");
        // for x in 0 .. self.scope_stack.len() {
        //     println!("stack levels pushed so far = {}", self.scope_stack[x].level);
        // }
        //println!("Current level number = {}", current_level);
        let index = self.does_level_exist_in_stack(current_level);
        //println!("Found index from stack == {}", index);
        if index != 0 {
            // index exists
            // pop first
            //println!("Level {} exists, pop and exit scope first", current_level);
            self.pop_and_exit_scope_from(index);
        }
        // push the level
        //println!("Push the level {}", current_level);
        self.scope_stack.push(ScopeStack {
            scope_type: scope.clone(),
            level: current_level,
        });
        // append the string
        match scope {
            ScopeType::ScopeMatch => {
                //println!("match scope");
                self.append(String::from(" {\n"));
            }
            ScopeType::ScopeFunc => {
                //println!("function scope");
                self.append(String::from(" {\n"));
            }
            ScopeType::ScopeCase => {
                //println!("case scope");
                self.append(String::from(" => {\n"));
            }
        }
    }

    pub fn exit_scope(&mut self, scope: ScopeType, _level: usize) {
        //println!("Exit scope for level number : {}", _level);
        match scope {
            ScopeType::ScopeMatch => {
                self.append(String::from("\n}"));
            }
            ScopeType::ScopeFunc => {
                self.append(String::from("\n}"));
            }
            ScopeType::ScopeCase => {
                self.append(String::from("\n},"));
                // For baseline matcher, there will be always
                // one node at one level. So, we will end
                // up with one if case and else case.
                self.append(String::from("\n_ => {},"));
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_leaf_node(&mut self, node: Node) -> bool {
        node.next.is_none()
    }

    pub fn init_dummy_node(&mut self) -> Node {
        Node {
            node_type: NodeType::MatchNone,
            node_value: "dummy".to_string(),
            width: 0,
            id: <usize>::max_value(),
            var_id: None,
            arg_flag: false,
            level: 0,
            next: None,
            idx_num: None,
            arg_name: "".to_string(),
        }
    }

    pub fn update_node_with_level(&mut self, mut node: Node, level: usize) -> Node {
        node.level = level;
        node
    }

    pub fn update_node_level_in_lhs(&mut self, updated_node: Node, nodes: &mut Vec<Node>) {
        for n in 0..nodes.len() {
            if nodes[n].id == updated_node.id {
                nodes[n].level = updated_node.level;
                break;
            }
        }
    }

    pub fn find_node_with_id(&mut self, id: usize, nodes: &mut Vec<Node>) -> Node {
        let mut found_node = self.init_dummy_node();
        for i in 0..nodes.len() {
            if id == nodes[i].id {
                found_node = nodes[i].clone();
                break;
            } else {
                continue;
            }
        }
        found_node
    }

    pub fn set_level_of_all_child_nodes(
        &mut self,
        nodes: &mut Vec<Node>,
        n: usize,
        current: usize,
    ) {
        if let Some(next_nodes) = nodes[n].next.clone() {
            if next_nodes.len() > 1 {
                panic!("Error: there should be only one node in LHS single tree\n");
            }
            for n in 0..next_nodes.len() {
                // It will certainly be one next node,
                // as this is for single LHS vec of nodes
                let id = next_nodes[n].index;
                let next_node = self.find_node_with_id(id, nodes);
                let updated_node = self.update_node_with_level(next_node.clone(), current + 1);
                self.update_node_level_in_lhs(updated_node.clone(), nodes);
            }
        }
    }

    pub fn build_root_node(&mut self) -> Node {
        Node {
            node_type: NodeType::MatchRoot,
            node_value: "root".to_string(),
            width: 0,
            id: 0,
            var_id: None,
            arg_flag: false,
            level: 0,
            next: Some(Vec::new()),
            idx_num: None,
            arg_name: "".to_string(),
        }
    }

    pub fn get_relation_in_args(
        &mut self,
        table: HashMap<String, usize>,
        arg1: String,
        arg2: String) -> String {
        let mut result = "".to_owned();
        let idx1 = table[&arg1];
        let idx2 = table[&arg2];
        if idx1 == idx2 {
            result.push_str(&arg1);
            result.push_str(&" == ".to_string());
            result.push_str(&arg2);
        } else {
            result.push_str(&arg1);
            result.push_str(&" != ".to_string());
            result.push_str(&arg2);
        }
        println!("*** Result str = {}\n", result);
        result
    }

    pub fn generate_path_condition(&mut self, pctable: HashMap<String, usize>) -> String {
        // first, collect all keys in vector
        let mut pc_args = Vec::new();
        for key in pctable.keys() {
            pc_args.push(key.clone());
        }
        
        // make all combinations
        let mut pcs = Vec::new();
        for i in 0..pc_args.len() {
            for j in i+1..pc_args.len() {
                println!("pcarg i = {}, j = {}\n", pc_args[i], pc_args[j]);
                let pc = self.get_relation_in_args(
                    pctable.clone(),
                    pc_args[i].clone(),
                    pc_args[j].clone());
                pcs.push(pc);
            }
        }

        let mut result = "".to_owned();
        if pcs.len() > 0 {
            result.push_str(&pcs[0]);
        }

        for i in 1..pcs.len() {
            result.push_str(&" && ".to_string());
            result.push_str(&pcs[i]);
        }
        result
    }

    pub fn take_action(&mut self, rhs: Vec<CliftInstWithArgs>, pctbl: HashMap<String, usize>, _level: usize) {
        let mut pc_str = "".to_owned();
        if pctbl.len() > 1 {
            pc_str += &"if ".to_owned();
            pc_str += &self.generate_path_condition(pctbl.clone());
            self.func_str.push_str(&pc_str);
            // FIXED: You can't enter into scope without pushing them
            // on the stack with level number of the node on scope stack
            //self.func_str.push_str(&" {\n".to_string());
            self.enter_scope(ScopeType::ScopeFunc, _level);
        }
        let mut replace_inst_str = "".to_owned();
        if rhs.len() == 1 {
            let each_inst = rhs[0].clone();
            replace_inst_str += &"pos.func.dfg.replace(".to_owned();
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

            // FIX: Add the rhs.cops vector string - should be one element only.
            for i in 0..each_inst.cops.len() {
                if i > 0 {
                    replace_inst_str += &", ".to_owned();
                }
                replace_inst_str += &each_inst.cops[i].to_owned();
            }
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

                // FIX: Insert the rhs.cops string args list
                for i in 0..each_inst.cops.len() {
                    if i > 0 {
                        replace_inst_str += &", ".to_owned();
                    }
                    replace_inst_str += &each_inst.cops[i].to_owned();
                }

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

                // FIX: Insert args list of rhs inst
                for i in 0..each_inst.cops.len() {
                    if i > 0 {
                        replace_inst_str += &", ".to_owned();
                    }
                    replace_inst_str += &each_inst.cops[i].to_owned();
                }

                replace_inst_str += &");\n".to_owned();
                self.func_str.push_str(&replace_inst_str);
            }
        }
        // FIXED: This was added just as a hack earlier
        // to exit the scope for if (args[x] == args[y]) condition
        // Now, exit_scope() function can take care of it because
        // enter_scope() is called to deal with entering into if condition
        //self.func_str.push_str(&"\n}\n".to_string());
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

pub fn is_node_actionable(node_id: usize, table: HashMap<usize, Vec<CliftInstWithArgs>>) -> bool {
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

pub fn get_arg_name_dup(idx: usize, tbl: HashMap<usize, String>) -> String {
    let mut arg_name = "PC_ARG_".to_owned();
    match tbl.get(&idx) {
        Some(name) => arg_name.push_str(&name),
        None => {
            println!("***```````` arg name not found, for index = {}\n", idx);
        },
    }
    arg_name
}

pub fn generate_baseline_matcher(
    mut nodes: Vec<Node>,
    rhs: HashMap<usize, Vec<CliftInstWithArgs>>,
    count: u32,
    idx_to_argname: HashMap<usize, String>,
    pc_table: HashMap<String, usize>
) -> String {
    for (id, rinsts) in &rhs {
        println!("id = {} : \n", id);
        for i in 0..rinsts.len() {
            println!("\t\tinst = {}\n",
                cliftinstbuilder::get_clift_opcode_name(rinsts[i].opcode.clone()));
        }
    }
    let mut opt_func = Opt::new();
    let mut arg_str = String::from("");
    let mut arg_counter: u32 = 0;
    let mut const_counter: u32 = 0;

    // Create and insert root node at the beginning of
    // vector of LHS single tree nodes
    nodes.insert(0, opt_func.build_root_node());

    for node in 0..nodes.len() {
        let action_flag = is_node_actionable(nodes[node].id, rhs.clone());
        // dump: begin
        println!("Node ==== ======================");
        println!("\t\t Actionable? = {}", action_flag);
        println!(
            "\t\t Node Type = {}",
            lhspatternmatcher::get_node_type(nodes[node].clone().node_type)
        );
        println!("\t\t Node Id = {}", nodes[node].id);
        println!("\t\t Node Level = {}", nodes[node].level);
        println!("\t\t Node Value = {}", nodes[node].node_value);
        match nodes[node].idx_num.clone() {
            Some(i) => {
                println!("\t\t Node op idx number = {}", i);
                println!("\t\t\t\t Node pre-cond name = {}\n", get_arg_name_dup(i, idx_to_argname.clone()));
            },
            None => println!("\t\t Node op idx number = NONE"),
        }
        println!("\t\t\t Node arg_name = {}", nodes[node].arg_name);
        match nodes[node].clone().var_id {
            Some (var_num) => {
                println!("\t\t\t Node Var number = {}", var_num);
            },
            None => {
                println!("\t\t\t Node Var number = None\n");
            },
        }
        match nodes[node].next.clone() {
            Some(ids) => {
                for i in 0..ids.len() {
                    println!("\t\t Node->next = {}", ids[i].index);
                }
            }
            None => println!("No next\n"),
        }
        // dump: end
        match nodes[node].node_type {
            NodeType::MatchRoot => {
                opt_func.generate_header(count);
                let current_level = nodes[node].level;
                opt_func.enter_scope(ScopeType::ScopeFunc, current_level);
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                if action_flag {
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::MatchInstData => {
                let current_level = nodes[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);

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
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::InstType => {
                let current_level = nodes[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                // Check if there is any child node already
                // matched at same level
                // If yes, pop and exit scope first, and
                // then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                match nodes[node].node_value.as_ref() {
                    "Var" => {}
                    "Binary" => {
                        // FIXME: "args" part, make a connection
                        // between actual args and string
                        opt_func.append(String::from("InstructionData::Binary { opcode, args }"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        // FIXED: Generate: "let args_<counter> = args;"

                        // NEW FIX: Jubi: We now pick arg_<number> from
                        // nodes' arg_name

                        // opt_func.append(String::from("let args_"));
                        // arg_counter = opt_func.get_argument_counter(arg_counter);

                        opt_func.append(String::from("let "));
                        opt_func.append(nodes[node].arg_name.clone());

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
                        //opt_func.append(String::from("let args_"));
                        //arg_counter = opt_func.get_argument_counter(arg_counter);
                        opt_func.append(String::from("let "));
                        opt_func.append(nodes[node].arg_name.clone());

                        opt_func.append(String::from(" = args;\n"));
                    }
                    "Unary" => {
                        // FIXME: "arg" part, make a connection
                        // b/w actual args and string
                        opt_func.append(String::from("InstructionData::Unary { opcode, arg }"));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        // FIXED: Generate: "let args_<counter> = arg;"
                        // FIXME: TODO: Feb 9: Do we need a fix here?
                        // shouldn't it be: "let <arg name of node> = arg;"
                        // opt_func.append(String::from("let args_"));
                        // arg_counter = opt_func.get_argument_counter(arg_counter);
                        // NEW FIX: "let <arg name of node> = arg;
                        opt_func.append(String::from("let "));
                        opt_func.append(nodes[node].arg_name.clone());
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
                        // FIXME: "args" part, make a connection
                        // between actual args and string
                        opt_func.append(String::from(
                            "InstructionData::BinaryImm64 { opcode, arg, imm }",
                        ));
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                        opt_func.set_entity(String::from("opcode"));
                        //opt_func.append(String::from("let args_"));
                        //arg_counter = opt_func.get_argument_counter(arg_counter);
                        // NEWFIX:just as we have in Binary "let <arg name of node> = arg"

                        opt_func.append(String::from("let "));
                        opt_func.append(nodes[node].arg_name.clone());
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
                        // opt_func.append(String::from("let args_"));
                        // arg_counter = opt_func.get_argument_counter(arg_counter);
                        // NEW FIX: just as IntCompare node

                        opt_func.append(String::from("let "));
                        opt_func.append(nodes[node].arg_name.clone());
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
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::MatchValDef => {
                let current_level = nodes[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                // Check if there is any child node already
                // matched at same level
                // If yes, pop and exit scope first, and
                // then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                match nodes[node].node_value.as_ref() {
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
                        // FIXME - do we want error handling here
                        // for NoneType and ""
                        println!("\t\t entering unknown valdef case\n");
                    }
                }
                if action_flag {
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::MatchOpcode => {
                let current_level = nodes[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                let opt_clone = opt_func.clone();
                let ent = opt_clone.current_entity;
                // FIXME: Any purpose of ent here?
                if !ent.is_empty() {
                    opt_func.append(String::from("match opcode"));
                    opt_func.enter_scope(ScopeType::ScopeMatch, current_level);
                }
                if action_flag {
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::Opcode => {
                let current_level = nodes[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                // Check if there is any child node already
                // matched at same level
                // If yes, pop and exit scope first, and
                // then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                // match the actual opcode types
                match nodes[node].node_value.as_ref() {
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
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::MatchCond => {
                let current_level = nodes[node].level;
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                let opt_clone = opt_func.clone();
                let ent = opt_clone.current_entity;
                // FIXME: Any purpose of ent here?
                if !ent.is_empty() {
                    opt_func.append(String::from("match cond"));
                    opt_func.enter_scope(ScopeType::ScopeMatch, current_level);
                }
                if action_flag {
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::Cond => {
                let current_level = nodes[node].level;
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                // Check if there is any child node already
                // matched at same level
                // If yes, pop and exit scope first, and
                // then enter into new matching case
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                // match the actual opcode types
                match nodes[node].node_value.as_ref() {
                    "eq" | "ne" | "ult" | "ule" | "slt" | "sle" => {
                        let cond = get_cond_name(nodes[node].clone().node_value);
                        opt_func.append(cond);
                        opt_func.enter_scope(ScopeType::ScopeCase, current_level);
                    }
                    _ => {
                        panic!("Error: this condition type is not yet handled");
                    }
                }
                if action_flag {
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::MatchArgs => {
                let current_level = nodes[node].level;
                //set the level of root->next nodes to 0+1
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                // Create an optional argument matching string here
                // we will decide later whether we need this match
                // on args or not depending on if the argument type
                // is Result or Param. Param
                // type does not need this match part at all.
                let arg_node_val = nodes[node].node_value.clone();
                let mut optional_argstr = String::from("");
                if arg_node_val.contains("arg") {
                    optional_argstr.push_str(&(String::from("match pos.func.dfg.value_def")));
                    optional_argstr.push_str(&(String::from("(")));
                    // make string like: args_2 or args_2[0]
                    // depending on binaryImm or binary
                    //let arg_node_val = nodes[node].node_value.clone();

                    // NEW FIX: Jubi: We will pick arg_name of this node,
                    // and append it with [0] or [1]
                    //optional_argstr.push_str(&(String::from("args_")));
                    //optional_argstr.push_str(&(String::from(arg_counter.to_string())));

                    optional_argstr.push_str(&(nodes[node].arg_name.clone()));

                    if let Some(i) = arg_node_val.find('[') {
                        optional_argstr.push_str(&(nodes[node].node_value.clone())[i..]);
                    }
                    optional_argstr.push_str(&(String::from(")")));
                    println!("in Match Args node ----> argument parameter = {}", optional_argstr.clone());
                }
                // FIXME: Do we want to take action here and should we
                // append to arg_str, or opt_func?
                if arg_str != optional_argstr {
                    arg_str.push_str(&optional_argstr);
                }
            }
            NodeType::MatchPlainConst => {
                let current_level = nodes[node].level;
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                if action_flag {
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            NodeType::MatchConst => {
                let current_level = nodes[node].level;
                // FIXED: Earlier, I assumed that constant node will
                // mark an end of the instruction i.e. it will always be
                // second operand of immediate instruction, and hence there
                // were no setting of levels for next nodes.
                // But, const node can be the first operand as well, so
                // no matter what, we should always look up for next nodes
                // and set their levels.
                opt_func.set_level_of_all_child_nodes(&mut nodes, node, current_level);
                let index = opt_func.does_level_exist_in_stack(current_level);
                if index != 0 {
                    opt_func.pop_and_exit_scope_from(index);
                }
                let const_value = &nodes[node].node_value;
                // FIXME: fix width of the constant in rhs part
                // Check Cranelift's instructions specifications
                const_counter = opt_func.get_const_counter(const_counter);
                // FIXME: pop the rhs immediate arguments from the ConstStack
                let rhs_arg = opt_func.pop_from_const_stack();
                opt_func.append(String::from("if "));
                opt_func.append(String::from(rhs_arg.to_string()));
                opt_func.append(String::from(" == "));
                opt_func.append(const_value.to_string());
                opt_func.enter_scope(ScopeType::ScopeFunc, current_level);
                if action_flag {
                    let found_rhs = &rhs[&nodes[node].id];
                    opt_func.take_action(found_rhs.to_vec(), pc_table.clone(), current_level);
                }
            }
            _ => {
                panic!("\n\nmatch type not handled yet!\n");
            }
        }
    }

    // exit func scope
    // debug scope stack info
    println!("********* Scope Stack ***********");
    for x in 0 .. opt_func.scope_stack.len() {
        let elem = opt_func.scope_stack[x].clone();
        println!("Level of scope elem = {}", elem.level);
        match elem.scope_type {
            ScopeType::ScopeFunc => {
                println!("scope func");
            },
            ScopeType::ScopeMatch => {
                println!("scope match");
            },
            ScopeType::ScopeCase => {
                println!("scope case");
            },
        }
    }
    println!("********* Scope Stack End ***********");
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
