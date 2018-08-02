// Main function to invoke lexer, parser, codegen modules

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod lexer;
mod parser;
mod cliftinstbuilder;
mod patternmatcher;
mod mergedtree;

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

    let mut splitter = contents.split(souper_delimiter);
    //let mut full_tree;
    for s in splitter {
        // lexing
        lexer::start_lexer(&s);

        // Parsing
        let souper_insts = parser::parse(&s);
    
        // Cranelift Instruction Building
        let clift_insts = cliftinstbuilder::transform_souper_to_clift_insts(souper_insts);
    
        // Pattern Matching - Single prefix tree
        let pattern_tree = patternmatcher::generate_single_tree_patterns(clift_insts);

        // Merged prefix tree
        //full_tree = mergedtree::generate_merged_prefix_tree(pattern_tree, full_tree);
        println!("======================================================");
  }
}
