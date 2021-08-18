use std::error::Error;
use clap::Clap;
use swedule::command_line_args::CommandLineArgs;
use swedule::run::run;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::parse();
    run(&args)
}
