// Main function to invoke lexer, parser, codegen modules

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod lexer;
mod parser;
mod cliftinstbuilder;
mod lhspatternmatcher;
mod rhspatternmatcher;
mod tablerhs;
mod mergedtree;
mod matcher;

use mergedtree::MergedArena;

fn main () {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: not enough arguments passed to souper parser");
    }

    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let souper_delimiter = "#########";

    let splitter = contents.split(souper_delimiter);
    let mut merged_arena = MergedArena{
                                       merged_tree: Vec::new(),
                                       hmap: HashMap::new(),
                                      };
    let mut rhs_table = HashMap::new();
    let mut global_nodes_count: usize = 0;
    for s in splitter {
        // lexing
        lexer::start_lexer(&s);

        // Parsing
        let souper_insts = parser::parse(&s);
    
        // Cranelift Instruction Building
        let clift_insts = cliftinstbuilder::transform_souper_to_clift_insts(souper_insts);
    
        // Pattern Matching - Single prefix tree
        let lhs_single_tree = lhspatternmatcher::generate_single_tree_patterns(clift_insts.clone(), global_nodes_count+1);

        // build prefix tree for RHS
        // TODO: FIXME: No need of global count in RHS, fix this module
        let rhs_single_tree = rhspatternmatcher::generate_single_tree_patterns(clift_insts.clone(), global_nodes_count+1);
        global_nodes_count += lhs_single_tree.len();

        // create a hashMap <leaf_node_of_each_LHS, RHS_tree>
        let hash_id = lhs_single_tree[lhs_single_tree.len()-1].id;
        println!("hash id for LHS is: {}\n", hash_id);
        rhs_table = tablerhs::map_lhs_to_rhs(hash_id, rhs_single_tree, rhs_table.clone());

        //for (x, y) in rhs_table.clone() {
        //    println!("******* For LHS ID = {}, RHS is == \n", x);
        //}

        // Merged prefix tree
        merged_arena = mergedtree::generate_merged_prefix_tree(lhs_single_tree, merged_arena.clone());
        
        // Pretty print the merged arena
        println!("----- nodes in merged_tree are -----");
        for n in 0 .. merged_arena.merged_tree.len() {
            println!("Node id = {}", merged_arena.merged_tree[n].id);
            if let Some(sub_nodes) = merged_arena.merged_tree[n].next.clone() {
                for sub_node in 0 .. sub_nodes.len() {
                    println!("\t\tSub Node id: {}", sub_nodes[sub_node].index);
                }
            } else {
                continue;
            }
        }

        match merged_arena.merged_tree[0].next.clone() {
            Some(nodes_list) => {
                for x in 0 .. nodes_list.len() {
                    println!("root's next = {}", nodes_list[x].index);
                }
            },
            None => {
            },
        }
        println!("==== hashmap entries =====");
        for (val, idx) in &merged_arena.hmap {
            println!("{}: {}", val, idx);
        }

        println!("======================================================");

  }
  let matcher_func = matcher::generate_matcher(merged_arena.clone());
  println!("{}", matcher_func);
}
