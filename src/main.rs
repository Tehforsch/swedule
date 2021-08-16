use std::{error::Error, fs, io, path::Path};

use clap::Clap;
use run_data::RunData;

use crate::{
    cell::Cell, domain_decomposition::do_domain_decomposition, sweep::Sweep, vector_3d::Vector3D,
};
use command_line_args::CommandLineArgs;
use direction::{get_equally_distributed_directions_on_sphere, Direction};
use grid::Grid;

pub mod cell;
pub mod command_line_args;
pub mod config;
pub mod dependency;
pub mod direction;
pub mod domain_decomposition;
pub mod edge;
pub mod face;
pub mod graph;
pub mod grid;
pub mod node;
pub mod processor;
pub mod run_data;
pub mod sweep;
pub mod task;
pub mod vector_3d;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::parse();
    let mut grid = read_grid_file(&args.grid_file)?;
    let num_directions = 84;
    let directions = get_equally_distributed_directions_on_sphere(num_directions);
    let processors = [1, 2, 4, 8, 16, 32, 64, 128];
    let run_data_list: Vec<RunData> = processors
        .iter()
        .map(|num_processors| run_sweep_on_processors(&mut grid, &directions, *num_processors))
        .collect();
    let reference = &run_data_list[0];
    for run_data in run_data_list.iter() {
        println!(
            "{:>4}: Time: {:.3} (speedup: {:>6.2}), comm: {:.3}, idle: {:.3}",
            run_data.num_processors,
            run_data.time,
            run_data.get_speedup(&reference),
            run_data.time_spent_communicating / run_data.time,
            run_data.time_spent_waiting / run_data.time,
        );
    }

    Ok(())
}

fn run_sweep_on_processors(
    mut grid: &mut Grid,
    directions: &[Direction],
    num_processors: usize,
) -> RunData {
    println!("Running on {}", num_processors);
    do_domain_decomposition(&mut grid, num_processors);
    let mut sweep = Sweep::new(&grid, &directions, num_processors);
    sweep.run()
}

fn read_grid_file(grid_file: &Path) -> io::Result<Grid> {
    let contents = fs::read_to_string(grid_file)?;
    let mut cells = vec![];
    let mut edges = vec![];
    for line in contents.lines() {
        let mut split = line.split_ascii_whitespace();
        let label = split.next().unwrap().parse::<usize>().unwrap();
        let processor_num = split.next().unwrap().parse::<usize>().unwrap();
        let x = split.next().unwrap().parse::<f64>().unwrap();
        let y = split.next().unwrap().parse::<f64>().unwrap();
        let z = split.next().unwrap().parse::<f64>().unwrap();
        let neighbours = split.map(|num| num.parse::<usize>().unwrap());
        let center = Vector3D::new(x, y, z);
        cells.push(Cell {
            index: label,
            center,
            processor_num,
        });
        for neighbour in neighbours {
            edges.push((label, neighbour));
        }
    }
    Ok(Grid::from_cell_pairs(cells, &edges))
}
