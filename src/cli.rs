use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rs-cleanup")]
#[command(about = "A tool to clean up directories", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Simulate directory after cleanup")]
    Simulate(commands::simulate::SimulateArgs),
}

pub fn parse() {
    let args = Cli::parse();
    match args.command {
        Commands::Simulate(args) => commands::simulate::execute(args),
    }
}