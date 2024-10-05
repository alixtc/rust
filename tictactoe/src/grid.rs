use ndarray::prelude::*;
use std::io;

type Point = i8;
pub type Grid = ndarray::ArrayBase<ndarray::OwnedRepr<Point>, ndarray::Dim<[usize; 2]>>;

pub trait GridChecker {
    fn is_grid_full(&self) -> bool;
    fn is_winning_grid(&self) -> Option<Winner>;
}

#[derive(Debug, PartialEq)]
pub enum Player {
    Human,
    Cpu,
}

type Winner = Player;

pub fn create_grid() -> Grid {
    Array::zeros((3, 3))
}
impl GridChecker for Grid {
    fn is_winning_grid(&self) -> Option<Winner> {
        let lines = self.sum_axis(Axis(1));
        let columns = self.sum_axis(Axis(0));

        let diag = self.diag().sum();

        let anti_diag = (0..=2).map(|x| self[[x, 2 - x]]).sum::<Point>();

        for val in [lines.to_vec(), columns.to_vec(), vec![diag, anti_diag]].concat() {
            match val {
                3 => return Some(Winner::Human),
                -3 => return Some(Winner::Cpu),
                _ => (),
            }
        }

        None
    }

    fn is_grid_full(&self) -> bool {
        self.flatten().iter().filter(|x| **x == 0).count() == 0
    }
}

pub fn extract_empty_positions(grid: &Grid) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(flat_idx, val)| if *val == 0 { Some(flat_idx) } else { None })
        .collect()
}


pub fn ask_for_position<R>(mut reader: R) -> String
// Required to facilitate testing
where
    R: io::BufRead,
{
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_user_input() {
        let dummy_input = b"I'm George";
        let answer = ask_for_position(&dummy_input[..]);
        assert_eq!("I'm George", answer);
    }

    #[test]
    fn creates_an_empty_grid() {
        let empty_grid = create_grid();
        assert_eq!(empty_grid, Array::zeros((3, 3)));
        assert_eq!(empty_grid, array![[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn is_wining_grid_should_return_none_with_no_winner() {
        let grid = create_grid();

        let result = grid.is_winning_grid();
        assert!(result.is_none());
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_lines() {
        let grid = array![[0, 0, 0], [1, 1, 1], [0, 0, 0]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[0, 0, 0], [-1, -1, -1], [0, 0, 0]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }
    #[test]
    fn is_wining_grid_should_return_winner_on_columns() {
        let grid = array![[1, 0, 0], [1, 0, 0], [1, 0, 0]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[-1, 0, 0], [-1, 0, 0], [-1, 0, 0]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_diagonal() {
        let grid = array![[1, 0, 0], [0, 1, 0], [0, 0, 1]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[-1, 0, 0], [0, -1, 0], [0, 0, -1]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_antidiagonal() {
        let grid = array![[0, 0, 1], [0, 1, 0], [1, 0, 0]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[0, 0, -1], [0, -1, 0], [-1, 0, 0]];
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_grid_full_should_detect_empty_slots() {
        let grid = array![[0, 0, 1], [0, 0, 0], [1, 0, 0]];
        assert!(!grid.is_grid_full());
    }
    #[test]
    fn is_grid_full_returns_true_with_no_empty_spots() {
        let grid = array![[-1, -1, 1], [-1, -1, -1], [1, -1, -1]];
        assert!(grid.is_grid_full());
    }

    #[test]
    fn extract_empty_positions_returns_an_array() {
        let grid = array![[-1, 1, 1], [0, -1, 1], [1, -1, 0]];
        assert_eq!(extract_empty_positions(&grid), vec![3, 8]);
        let grid = array![[-1, 1, 1], [-1, -1, 1], [1, -1, -1]];
        assert_eq!(extract_empty_positions(&grid), vec![]);
    }
}
