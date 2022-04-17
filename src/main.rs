use clap::StructOpt;
use command_line_arguments::CommandLineArguments;
use env_logger::Env;

mod algorithm;
mod repetition_algorithm;
mod command_line_arguments;
mod extensions;
mod q1;
mod q2;

fn main() {
    let env = Env::new().default_filter_or("info");
    env_logger::init_from_env(env);
    CommandLineArguments::parse().invoke().unwrap();
}
