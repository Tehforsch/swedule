use std::error::Error;

use clap::Clap;
use voronoi_swim::command_line_args::CommandLineArgs;
use voronoi_swim::param_file::ParamFile;
use voronoi_swim::run::run;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::parse();
    let param_file = ParamFile::read(&args.param_file)?;
    run(&param_file, &args)
}
