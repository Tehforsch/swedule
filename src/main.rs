use std::{error::Error, fs, io, path::Path};

use clap::Clap;

use command_line_args::CommandLineArgs;
use direction::Direction;
use grid::Grid;
use crate::{cell::Cell, vector_3d::Vector3D};

pub mod cell;
pub mod direction;
pub mod edge;
pub mod face;
pub mod graph;
pub mod grid;
pub mod node;
pub mod task;
pub mod vector_3d;
pub mod command_line_args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::parse();
    let grid = read_grid_file(&args.grid_file)?;
    let graph = grid.get_dependency_graph(&Direction {
        vector: Vector3D::new(1.0, 0.0, 0.0),
        index: 0,
    });
    Ok(())
}

fn read_grid_file(grid_file: &Path) -> io::Result<Grid> {
    let contents = fs::read_to_string(grid_file)?;
    let mut cells = vec![];
    let mut edges = vec![];
    for line in contents.lines() {
        let mut split = line.split_ascii_whitespace();
        let label = split.next().unwrap().parse::<usize>().unwrap();
        let x = split.next().unwrap().parse::<f64>().unwrap();
        let y = split.next().unwrap().parse::<f64>().unwrap();
        let z = split.next().unwrap().parse::<f64>().unwrap();
        let neighbours = split.map(|num| num.parse::<usize>().unwrap());
        cells.push(Cell {
            label: label,
            center: Vector3D::new(x, y, z),
        });
        for neighbour in neighbours {
            edges.push((label, neighbour));
        }
    }
    Ok(Grid::from_cell_pairs(cells, &edges))
}

