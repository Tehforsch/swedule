use clap::Clap;
use std::error::Error;
use voronoi_swim::command_line_args::CommandLineArgs;
use voronoi_swim::run::run;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::parse();
    run(&args)
}
