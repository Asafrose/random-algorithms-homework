use clap::StructOpt;
use command_line_arguments::CommandLineArguments;

mod hash_function;
mod command_line_arguments;
mod q1;

fn main() {
    CommandLineArguments::parse().invoke().unwrap();
}
