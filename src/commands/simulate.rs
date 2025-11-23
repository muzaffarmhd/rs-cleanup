use clap::Parser;
use std::process::Command;
use crate::internal::parse::parse_rules;

#[derive(Parser, Debug)]
pub struct SimulateArgs {
    tree: String
}

pub fn execute(args: SimulateArgs) {
    // Command::new("tree")
    //     .arg(&args.tree)
    //     .status()
    //     .expect("Failed to execute tree command");

    parse_rules(&args.tree);
}
