use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rs-cleanup")]
#[command(about = "A tool to clean up directories", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
}

pub fn parse() -> Cli {
    Cli::parse()
}