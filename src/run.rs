use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::{error::Error, fs, io, path::Path};

use crate::cell::CellId;
use crate::command_line_args::CommandLineArgs;
use crate::direction::{get_directions, Direction};
use crate::grid::Grid;
use crate::run_data::RunData;
use crate::{
    cell::Cell, domain_decomposition::do_domain_decomposition, sweep::Sweep,
    vector_3d::Vector3D,
};

pub fn run(args: &CommandLineArgs) -> Result<(), Box<dyn Error>> {
    // let directions = get_equally_distributed_directions_on_sphere(NUM_DIRECTIONS);
    let directions = get_directions(1);
    let grids: Result<Vec<_>> = args
        .grid_files
        .iter()
        .map(|file| convert_to_grid(file))
        .collect();
    let run_data_list: Vec<_> = match args.domain_decomposition {
        None => grids?
            .into_iter()
            .map(|mut grid| run_sweep_on_processors(&mut grid, &directions))
            .collect(),
        Some(num) => grids?
            .into_iter()
            .map(|mut grid| {
                run_sweep_and_domain_decomposition_on_processors(&mut grid, &directions, num)
            })
            .collect(),
    };
    let reference = &run_data_list[0];
    for run_data in run_data_list.iter() {
        if !args.quiet {
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
    Ok(())
}

fn convert_to_grid(file: &Path) -> Result<Grid> {
    if let Some(extension) = file.extension() {
        let ext_str = extension.to_str().unwrap();
        assert_eq!(ext_str, "dat");
        return read_grid_file(file).context("While reading file as grid file");
    }
    Err(anyhow!("Unknown file ending"))
}

fn run_sweep_and_domain_decomposition_on_processors(
    mut grid: &mut Grid,
    directions: &[Direction],
    num_processors: usize,
) -> RunData {
    do_domain_decomposition(&mut grid, num_processors);
    let mut sweep = Sweep::new(grid, directions, num_processors);
    sweep.run()
}

fn run_sweep_on_processors(grid: &mut Grid, directions: &[Direction]) -> RunData {
    let num_processors = grid.iter().map(|cell| cell.processor_num).max().unwrap() + 1;
    let mut sweep = Sweep::new(grid, directions, num_processors);
    sweep.run()
}

fn read_grid_file(grid_file: &Path) -> io::Result<Grid> {
    let contents = fs::read_to_string(grid_file)?;
    let mut cells = vec![];
    let mut edges = vec![];
    for line in contents.lines() {
        let (cell, neighbours) = get_cell_and_neighbour_list_from_line(line);
        for neighbour in neighbours {
            edges.push((cell.get_id(), neighbour));
        }
        cells.push(cell);
    }
    let mut label_to_indices: HashMap<CellId, usize> = HashMap::new();
    cells.sort_by_key(|cell| (cell.processor_num, cell.local_index));
    for (index, mut cell) in cells.iter_mut().enumerate() {
        label_to_indices.insert(cell.get_id(), index);
        cell.global_index = index;
    }
    let edges: Vec<(usize, usize)> = edges
        .into_iter()
        .map(|(id, neighbour)| (label_to_indices[&id], label_to_indices[&neighbour]))
        .collect();
    Ok(Grid::from_cell_pairs(cells, &edges))
}

fn get_cell_and_neighbour_list_from_line(line: &str) -> (Cell, Vec<CellId>) {
    let mut split = line.split_ascii_whitespace();
    let index = split.next().unwrap().parse::<usize>().unwrap();
    let processor_num = split.next().unwrap().parse::<usize>().unwrap();
    let x = split.next().unwrap().parse::<f64>().unwrap();
    let y = split.next().unwrap().parse::<f64>().unwrap();
    let z = split.next().unwrap().parse::<f64>().unwrap();
    let neighbours = split.map(|num| num.parse::<CellId>().unwrap()).collect();
    let center = Vector3D::new(x, y, z);
    let cell = Cell {
        global_index: 0,
        local_index: index,
        processor_num,
        center,
    };
    (cell, neighbours)
}
