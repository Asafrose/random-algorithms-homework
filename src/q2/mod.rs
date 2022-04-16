mod hash_function;
mod q2_amplificated_command;
mod q2_naive_command;

use anyhow::Result;
use clap::{Args, Subcommand};
use log::debug;
use nameof::name_of;

use crate::extensions::vec_extensions::VecExtensions;

use self::{
    hash_function::HashFunction, q2_amplificated_command::Q2AmplificatedCommand,
    q2_naive_command::Q2NaiveCommand,
};

#[derive(Debug, Args)]
pub struct Q2Command {
    #[clap(subcommand)]
    command: Commands,
}

impl Q2Command {
    pub fn invoke(&self) -> Result<()> {
        let array = Vec::with_random_items_in_range(1000, || 0..=2);

        match &self.command {
            Commands::Naive(command) => command.invoke(&array),
            Commands::Amplificated(command) => command.invoke(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Naive(Q2NaiveCommand),
    Amplificated(Q2AmplificatedCommand),
}

fn invoke_internal(array: &Vec<i32>) -> i32 {
    debug!("{} started", name_of!(invoke_internal));

    let hash_function = HashFunction::new(0..array.len(), [-1, 1]);

    let result = (0..array.len())
        .map(|num| hash_function.get_value(&num) * array[num])
        .sum::<i32>()
        .pow(2);

    debug!(
        "{} finished [{}={}]",
        name_of!(invoke_internal),
        name_of!(result),
        result
    );

    result
}

fn get_l2_norm(array: &Vec<i32>) -> i32 {
    array.iter().map(|num| num.pow(2)).sum()
}
