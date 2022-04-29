use crate::{document::DocumentCommand, q2::Q2Command, q3::Q3Command};
use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;

use super::q1::Q1Command;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CommandLineArguments {
    #[clap(short, long)]
    debug: bool,
    #[clap(subcommand)]
    command: Commands,
}

impl CommandLineArguments {
    pub fn invoke(&self) -> Result<()> {
        let env = Env::new().default_filter_or(if self.debug { "debug" } else { "info" });
        env_logger::init_from_env(env);

        self.command.invoke()
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    ///Run Question1 program
    Q1(Q1Command),
    ///Run Question2 program
    Q2(Q2Command),
    ///Run Question3 program
    Q3(Q3Command),
    ///Run Documentation process
    Document(DocumentCommand),
}

impl Commands {
    pub fn invoke(&self) -> Result<()> {
        match self {
            Commands::Q1(command) => command.invoke(),
            Commands::Q2(command) => command.invoke(),
            Commands::Q3(command) => command.invoke(),
            Commands::Document(command) => command.invoke(),
        }
    }
}
