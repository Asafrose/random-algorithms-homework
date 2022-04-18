mod hash_function;
mod l2_algorithm;
mod q2_amplificated_command;
mod q2_naive_command;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{common::algorithm::Algorithm, extensions::vec_extensions::SampleUniformVecExtensions};

use self::{q2_amplificated_command::Q2AmplificatedCommand, q2_naive_command::Q2NaiveAlgorithm};

#[derive(Debug, Args)]
pub struct Q2Command {
    #[clap(subcommand)]
    command: Commands,
}

impl Q2Command {
    pub fn invoke(&self) -> Result<()> {
        let array = Vec::with_random_items_in_range(1000, || 0..=2);

        match &self.command {
            Commands::Naive => Q2NaiveAlgorithm::run(array),
            Commands::Amplificated(command) => command.invoke(array),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Naive,
    Amplificated(Q2AmplificatedCommand),
}
