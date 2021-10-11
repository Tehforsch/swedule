use std::error::Error;

use clap::Clap;
use voronoi_swim::command_line_args::CommandLineArgs;
use voronoi_swim::run::simulate_grid;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::parse();
    let run_data_list = simulate_grid(&args.param_file, &args.grid_files);
    let reference = &run_data_list[0];
    for run_data in run_data_list.iter() {
        println!(
            "{:>4} {:.3} (speedup: {:>6.2}, efficiency {:>6.2}), comm: {:.3}, idle: {:.3}",
            run_data.num_processors,
            run_data.time,
            run_data.get_speedup(reference),
            run_data.get_efficiency(reference),
            run_data.time_spent_communicating / run_data.time,
            run_data.time_spent_waiting / run_data.time,
        );
    }
}
