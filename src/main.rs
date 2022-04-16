use clap::StructOpt;
use command_line_arguments::CommandLineArguments;
use env_logger::Env;

mod hash_function;
mod command_line_arguments;
mod q1;

fn main() {
    let env = Env::new().default_filter_or("info");
    env_logger::init_from_env(env);
    CommandLineArguments::parse().invoke().unwrap();
}
