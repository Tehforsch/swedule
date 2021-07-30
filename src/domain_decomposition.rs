use generational_arena::Index;
use hilbert::{point_list, Point};

use crate::grid::Grid;
pub fn do_domain_decomposition(grid: &mut Grid, num_processors: usize) {
    let points: Vec<Vec<f64>> = grid
        .iter()
        .map(|cell| vec![*cell.center.x, *cell.center.y, *cell.center.z])
        .collect();
    let indices: Vec<Index> = grid.iter_nodes().map(|node| node.index).collect();
    let factor = 100.0;
    let (mut point_list, _bits) = point_list::make_points_f64(&points, 0, None, None, factor);
    let bits = factor.log2().ceil() as usize;
    Point::hilbert_sort(&mut point_list, bits);
    let weight_per_cell = 1.0; // Cell weight is constant for a sweep
    let total_weight: f64 = points.len() as f64 * weight_per_cell;
    let weight_per_processor = total_weight / (num_processors as f64);
    let mut current_processor = 0;
    let mut current_weight = 0.0;
    for point in point_list {
        let cell_index = indices[point.get_id()];
        let cell = grid.get_cell_by_index_mut(cell_index);
        cell.processor_num = current_processor;
        current_weight += weight_per_cell;
        println!(
            "{} {} {} {}",
            cell.center.x, cell.center.y, cell.center.z, current_processor
        );
        if current_weight > weight_per_processor {
            current_weight = 0.0;
            current_processor += 1;
        }
    }
}
