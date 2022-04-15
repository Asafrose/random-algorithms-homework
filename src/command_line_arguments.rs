use anyhow::Result;
use clap::{Parser, Subcommand};
use super::q1::Q1Command;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CommandLineArguments {
    #[clap(subcommand)]
    command: Commands,
}

impl CommandLineArguments {
    pub fn invoke(&self) -> Result<()> {
        self.command.invoke()
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Q1(Q1Command)
}

impl Commands {
    pub fn invoke(&self) -> Result<()> {
        match self {
            Commands::Q1(command) => command.invoke()
        }
    }
}
