use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short = 'd', long = "description")]
        description: String,
    },
    #[command(alias = "ls")]
    List {
        #[arg(short = 'a', long = "all")]
        all: bool,
    },
}
