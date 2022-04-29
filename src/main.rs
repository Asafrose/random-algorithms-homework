use clap::StructOpt;
use command_line_arguments::CommandLineArguments;
mod command_line_arguments;
mod common;
mod document;
mod extensions;
mod q1;
mod q2;
mod q3;

fn main() {
    CommandLineArguments::parse().invoke().unwrap();
}
