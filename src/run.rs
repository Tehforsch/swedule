use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::cell::Cell;
use crate::cell::CellId;
use crate::direction::get_directions;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::param_file::ParamFile;
use crate::run_data::RunData;
use crate::sweep::Sweep;
use crate::vector_3d::Vector3D;

pub fn simulate_grid<
        U: AsRef<Path>,
        V: AsRef<Path>
        >(
    param_file_path: U,
    grid_files: &[V],
) -> Result<Vec<RunData>> {
    let param_file = ParamFile::read(&param_file_path.as_ref())?;
    let directions = get_directions(param_file.num_directions);
    let grids: Result<Vec<_>> = grid_files
        .iter()
        .map(|file| convert_to_grid(file.as_ref()))
        .collect();
    let run_data_list: Vec<_> = grids?
        .into_iter()
        .map(|mut grid| run_sweep_on_processors(&param_file, &mut grid, &directions))
        .collect();
    Ok(run_data_list)
}

fn convert_to_grid(file: &Path) -> Result<Grid> {
    if let Some(extension) = file.extension() {
        let ext_str = extension.to_str().unwrap();
        assert_eq!(ext_str, "dat");
        return read_grid_file(file).context("While reading file as grid file");
    }
    Err(anyhow!("Unknown file ending"))
}

fn run_sweep_on_processors(
    param_file: &ParamFile,
    grid: &mut Grid,
    directions: &[Direction],
) -> RunData {
    let num_processors = grid.iter().map(|cell| cell.processor_num).max().unwrap() + 1;
    let mut sweep = Sweep::new(param_file, grid, directions, num_processors);
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
