use super::grid::{extract_empty_positions, Grid};
use ndarray::prelude::*;

use rand::{seq::SliceRandom, thread_rng};

fn make_random_move(grid: &Grid) -> Grid {
    let empty_position = extract_empty_positions(grid);
    let mut rng = thread_rng();
    let random_positions = empty_position
        .values()
        .collect::<Vec<_>>()
        .choose(&mut rng)
        .unwrap()
        .to_owned();

    let mut new_grid = grid.clone();
    new_grid[*random_positions] = -1;
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_random_move_should_add_minus_one() {
        #[rustfmt::skip]
        let grid = array![
            [0, 0, 1], 
            [0, -1, 0], 
            [1, 0, 0]
        ];
        let new_grid = make_random_move(&grid);
        let zero_delta =
            grid.iter().filter(|x| **x == 0).count() - new_grid.iter().filter(|x| **x == 0).count();
        assert_eq!(zero_delta, 1);
        assert_eq!(new_grid.iter().filter(|x| **x == -1).count(), 2);
    }
}
