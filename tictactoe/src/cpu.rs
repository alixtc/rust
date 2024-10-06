use crate::GridChecker;

use super::grid::{extract_empty_positions, Grid};

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

#[derive(Copy, Clone)]
enum Marker {
    X = 1,
    O = -1,
}

fn extract_winning_positions(grid: &Grid, marker: &Marker) -> Vec<(usize, usize)> {
    let mut winning_position = Vec::<(usize, usize)>::new();

    for ((x, y), _) in grid.indexed_iter().filter(|(_, val)| **val == 0) {
        let mut attempt_grid = grid.clone();
        attempt_grid[[x, y]] = *marker as i8;
        if attempt_grid.is_winning_grid().is_some() {
            winning_position.push((x, y));
        }
    }

    winning_position
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Difficulty {
    Low = 0,
    Medium = 1,
    High = 2,
}

fn make_cpu_move(grid: &Grid, difficulty: Difficulty) -> Grid {
    if difficulty == Difficulty::High {
        let winning_moves = extract_winning_positions(grid, &Marker::O);
        if !winning_moves.is_empty() {
            let mut new_grid = grid.clone();
            let (x, y) = winning_moves[0];
            new_grid[[x, y]] = -1;
            return new_grid;
        }
    }

    if difficulty >= Difficulty::Medium {
        let adversary_winning_moves = extract_winning_positions(grid, &Marker::X);
        if !adversary_winning_moves.is_empty() {
            let mut new_grid = grid.clone();
            let (x, y) = adversary_winning_moves[0];
            new_grid[[x, y]] = -1;
            return new_grid;
        }
    }

    make_random_move(grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::prelude::*;

    #[test]
    fn make_random_move_should_add_minus_one() {
        #[rustfmt::skip]
        let grid = array![
            [0, 0, 1],
            [0, -1, 0],
            [1, 0, 0],
        ];
        let new_grid = make_random_move(&grid);
        let zero_delta =
            grid.iter().filter(|x| **x == 0).count() - new_grid.iter().filter(|x| **x == 0).count();
        assert_eq!(zero_delta, 1);
        assert_eq!(new_grid.iter().filter(|x| **x == -1).count(), 2);
    }

    #[test]
    fn extract_winning_positions_returns_empty_array_with_no_winning_position() {
        #[rustfmt::skip]
        let grid = array![
            [1, -1, 1],
            [-1, 0, 1],
            [1, 1, -1],
        ];
        let positions = extract_winning_positions(&grid, &Marker::O);
        assert_eq!(positions, Vec::new())
    }

    #[test]
    fn extract_winning_positions_returns_array_of_tuples() {
        #[rustfmt::skip]
        let grid = array![
            [1, 0, 1],
            [-1, 0, -1],
            [1, 1, -1],
        ];
        let positions = extract_winning_positions(&grid, &Marker::O);
        assert_eq!(positions, vec![(1, 1)])
    }

    #[test]
    fn extract_winning_positions_work_on_diagonal_and_anti_diagonal() {
        #[rustfmt::skip]
        let grid = array![
            [-1, 1, -1],
            [1, -1, 1],
            [0, 0, 0],
        ];
        let positions = extract_winning_positions(&grid, &Marker::O);
        assert_eq!(positions, vec![(2, 0), (2, 2),]);
    }

    #[test]
    fn extract_winning_positions_allows_to_switch_player() {
        #[rustfmt::skip]
        let grid = array![
            [1, -1, 1],
            [0, 0, -1],
            [1, 1, -1],
        ];
        let positions = extract_winning_positions(&grid, &Marker::X);
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn make_cpu_move_should_fill_an_empty_slot_on_low_difficulty() {
        #[rustfmt::skip]
        let grid = array![
            [1, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ];
        let grid_after_action = make_cpu_move(&grid, Difficulty::Low);
        assert_eq!(grid_after_action.iter().filter(|x| **x == -1).count(), 1);
    }

    #[test]
    fn make_cpu_move_should_fill_block_auto_win_when_medium_difficulty() {
        #[rustfmt::skip]
        let grid = array![
            [1, 0, 0],
            [1, -1, 0],
            [0, 0, 0],
        ];
        let expected = array![[1, 0, 0], [1, -1, 0], [-1, 0, 0],];
        let grid_after_action = make_cpu_move(&grid, Difficulty::Medium);
        assert_eq!(grid_after_action, expected);
    }

    #[test]
    fn make_cpu_move_should_fill_block_auto_win_when_high_difficulty() {
        #[rustfmt::skip]
        let grid = array![
            [1, 0, 0],
            [1, -1, 0],
            [0, 0, 0],
        ];
        let expected = array![[1, 0, 0], [1, -1, 0], [-1, 0, 0],];
        let grid_after_action = make_cpu_move(&grid, Difficulty::High);
        assert_eq!(grid_after_action, expected);
    }

    #[test]
    fn make_cpu_move_should_make_winning_move_on_high_difficulty() {
        #[rustfmt::skip]
        let grid = array![
            [1, -1, 0],
            [1, -1, 0],
            [0, 0, 0],
        ];
        let expected = array![[1, -1, 0], [1, -1, 0], [0, -1, 0],];
        let grid_after_action = make_cpu_move(&grid, Difficulty::High);
        assert_eq!(grid_after_action, expected);
    }
}
