use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
pub struct SimulateArgs {
    tree: String
}

pub fn execute(args: SimulateArgs) {
    Command::new("tree")
        .arg(&args.tree)
        .status()
        .expect("Failed to execute tree command");
}
