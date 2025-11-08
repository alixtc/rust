use crate::GridChecker;

use super::grid::{Grid, Marker};

use rand::{seq::SliceRandom, thread_rng};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Difficulty {
    Low = 1,
    Medium = 2,
    High = 3,
}

fn make_random_move(grid: &Grid) -> Grid {
    let empty_position = grid.extract_empty_positions();
    let mut rng = thread_rng();
    let random_grid_coordinates = empty_position
        .values()
        .collect::<Vec<_>>()
        .choose(&mut rng)
        .unwrap()
        .to_owned();

    let mut new_grid = grid.clone();
    new_grid.insert(*random_grid_coordinates, -1);
    new_grid
}

pub fn make_cpu_move(grid: &Grid, difficulty: Difficulty) -> Grid {
    if difficulty == Difficulty::High {
        let winning_moves = grid.extract_winning_positions(&Marker::O);
        if !winning_moves.is_empty() {
            let mut new_grid = grid.clone();
            let (x, y) = winning_moves[0];
            new_grid.insert((x, y), -1);
            return new_grid;
        }
    }

    if difficulty >= Difficulty::Medium {
        let adversary_winning_moves = grid.extract_winning_positions(&Marker::X);
        if !adversary_winning_moves.is_empty() {
            let mut new_grid = grid.clone();
            let (x, y) = adversary_winning_moves[0];
            new_grid.insert((x, y), -1);
            return new_grid;
        }
    }

    make_random_move(grid)
}

#[cfg(test)]
mod tests {
    use super::super::grid::from_array;
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn make_random_move_should_add_minus_one() {
        #[rustfmt::skip]
        let grid = from_array([
            [0, 0, 1],
            [0, -1, 0],
            [1, 0, 0],
        ]);
        let new_grid = make_random_move(&grid);

        let zero_delta =
            grid.extract_empty_positions().len() - new_grid.extract_empty_positions().len();
        assert_eq!(zero_delta, 1);
        assert_eq!(new_grid.grid.values().filter(|x| **x == -1).count(), 2);
    }

    #[test]
    fn extract_winning_positions_returns_empty_array_with_no_winning_position() {
        #[rustfmt::skip]
        let grid = from_array([
            [1, -1, 1],
            [-1, 0, 1],
            [1, 1, -1],
        ]);
        let positions = grid.extract_winning_positions(&Marker::O);
        assert_eq!(positions, Vec::new())
    }

    #[test]
    fn extract_winning_positions_returns_array_of_tuples() {
        #[rustfmt::skip]
        let grid = from_array([
            [1, 0, 1],
            [-1, 0, -1],
            [1, 1, -1],
        ]);
        let positions = grid.extract_winning_positions(&Marker::O);
        assert_eq!(positions, vec![(1, 1)])
    }

    #[test]
    fn extract_winning_positions_work_on_diagonal_and_anti_diagonal() {
        #[rustfmt::skip]
        let grid = from_array([
            [-1, 1, -1],
            [1, -1, 1],
            [0, 0, 0],
        ]);
        let positions = grid.extract_winning_positions(&Marker::O);
        assert_eq!(
            HashSet::from([(2, 0), (2, 2)]),
            HashSet::from_iter(positions)
        );
    }

    #[test]
    fn extract_winning_positions_allows_to_switch_player() {
        #[rustfmt::skip]
        let grid = from_array([
            [1, -1, 1],
            [0, 0, -1],
            [1, 1, -1],
        ]);
        let positions = grid.extract_winning_positions(&Marker::X);
        assert_eq!(positions.len(), 1);
        assert_eq!(positions, [(1, 0)])
    }

    #[test]
    fn make_cpu_move_should_fill_an_empty_slot_on_low_difficulty() {
        #[rustfmt::skip]
        let grid = from_array([
            [1, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
        ]);
        let grid_after_action = make_cpu_move(&grid, Difficulty::Low);
        assert_eq!(
            grid_after_action
                .grid
                .values()
                .filter(|x| **x == -1)
                .count(),
            1
        );
    }

    #[test]
    fn make_cpu_move_should_fill_block_auto_win_when_medium_difficulty() {
        #[rustfmt::skip]
        let grid = from_array([
            [1, 0, 0],
            [1, -1, 0],
            [0, 0, 0],
        ]);
        let expected = from_array([[1, 0, 0], [1, -1, 0], [-1, 0, 0]]);
        let grid_after_action = make_cpu_move(&grid, Difficulty::Medium);
        assert_eq!(grid_after_action, expected);
    }

    #[test]
    fn make_cpu_move_should_fill_block_auto_win_when_high_difficulty() {
        #[rustfmt::skip]
        let grid = from_array([
            [1, 0, 0],
            [1, -1, 0],
            [0, 0, 0],
        ]);
        let expected = from_array([[1, 0, 0], [1, -1, 0], [-1, 0, 0]]);
        let grid_after_action = make_cpu_move(&grid, Difficulty::High);
        assert_eq!(grid_after_action, expected);
    }

    #[test]
    fn make_cpu_move_should_make_winning_move_on_high_difficulty() {
        #[rustfmt::skip]
        let grid = from_array([
            [1, -1, 0],
            [1, -1, 0],
            [0, 0, 0],
        ]);
        let expected = from_array([[1, -1, 0], [1, -1, 0], [0, -1, 0]]);
        let grid_after_action = make_cpu_move(&grid, Difficulty::High);
        assert_eq!(grid_after_action, expected);
    }
}
