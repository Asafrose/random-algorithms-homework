use std::{fs::File, io::Write};

use clap::{StructOpt, IntoApp};
use command_line_arguments::CommandLineArguments;
use env_logger::Env;
mod command_line_arguments;
mod common;
mod extensions;
mod q1;
mod q2;
mod q3;

fn main() {
    let env = Env::new().default_filter_or("info");
    env_logger::init_from_env(env);
    CommandLineArguments::parse().invoke().unwrap();
}
