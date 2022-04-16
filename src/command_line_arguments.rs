use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::q2::Q2Command;

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
    ///Run Question1 program
    Q1(Q1Command),
    ///Run Question2 program
    Q2(Q2Command)
}

impl Commands {
    pub fn invoke(&self) -> Result<()> {
        match self {
            Commands::Q1(command) => command.invoke(),
            Commands::Q2(command) => command.invoke(),
        }
    }
}
