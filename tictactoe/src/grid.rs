use itertools::*;

use std::collections::HashMap;

type Point = i8;
// pub type Grid = ndarray::ArrayBase<ndarray::OwnedRepr<Point>, ndarray::Dim<[usize; 2]>>;
pub type Grid = HashMap<(i32, i32), i32>;
type ManualGrid = [[i32; 3]; 3];
pub trait GridChecker {
    fn is_grid_full(&self) -> bool;
    fn is_winning_grid(&self) -> Option<Winner>;
}

pub fn from_array(array: ManualGrid) -> Grid {
    let mut grid = HashMap::new();
    for (ix_row, row) in array.iter().enumerate() {
        for (ix_col, value) in row.iter().enumerate() {
            grid.insert((ix_row as i32, ix_col as i32), value.to_owned());
        }
    }
    grid
}

#[derive(Debug, PartialEq)]
pub enum Player {
    Human,
    Cpu,
}

type Winner = Player;

pub fn create_grid() -> Grid {
    let mut grid = HashMap::new();
    for x in 0..=8 {
        grid.insert((x / 3, x % 3), 0);
    }
    grid
    // Array::zeros((3, 3)
}
impl GridChecker for Grid {
    fn is_winning_grid(&self) -> Option<Winner> {
        let lines: Vec<i32> = self
            .iter()
            .into_grouping_map_by(|((row, _), _)| row)
            .fold(0, |acc, _key, (_, val)| acc + val)
            .into_values()
            .collect();
        let columns: Vec<i32> = self
            .iter()
            .into_grouping_map_by(|((_, col), _)| col)
            .fold(0, |acc, _key, (_, val)| acc + val)
            .into_values()
            .collect();

        let diag = self
            .iter()
            .filter(|((row, col), _)| row == col)
            .map(|((_, _), val)| val)
            .sum();

        let anti_diag = self
            .iter()
            .filter(|((row, col), _)| [2, 4, 6].contains(&(row + col)))
            .map(|(_, val)| val)
            .sum();

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
        self.values().filter(|x| **x == 0).count() == 0
    }
}

pub fn extract_empty_positions(grid: &Grid) -> HashMap<usize, (i32, i32)> {
    grid.iter()
        .enumerate()
        .filter_map(|(flat_idx, (row_col_idx, val))| {
            if *val == 0 {
                Some((flat_idx, row_col_idx.to_owned()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, (_, _)>>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn creates_an_empty_grid() {
        let empty_grid = create_grid();
        assert_eq!(empty_grid, from_array([[0, 0, 0], [0, 0, 0], [0, 0, 0]]));
    }

    #[test]
    fn is_wining_grid_should_return_none_with_no_winner() {
        let grid = create_grid();

        let result = grid.is_winning_grid();
        assert!(result.is_none());
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_lines() {
        let grid = from_array([[0, 0, 0], [1, 1, 1], [0, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = from_array([[0, 0, 0], [-1, -1, -1], [0, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }
    #[test]
    fn is_wining_grid_should_return_winner_on_columns() {
        let grid = from_array([[1, 0, 0], [1, 0, 0], [1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = from_array([[-1, 0, 0], [-1, 0, 0], [-1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_diagonal() {
        let grid = from_array([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = from_array([[-1, 0, 0], [0, -1, 0], [0, 0, -1]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_antidiagonal() {
        let grid = from_array([[0, 0, 1], [0, 1, 0], [1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = from_array([[0, 0, -1], [0, -1, 0], [-1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_grid_full_should_detect_empty_slots() {
        let grid = from_array([[0, 0, 1], [0, 0, 0], [1, 0, 0]]);
        assert!(!grid.is_grid_full());
    }
    #[test]
    fn is_grid_full_returns_true_with_no_empty_spots() {
        let grid = from_array([[-1, -1, 1], [-1, -1, -1], [1, -1, -1]]);
        assert!(grid.is_grid_full());
    }

    #[test]
    fn extract_empty_positions_returns_an_array() {
        let grid = from_array([
            [-1, 1, 1], 
            [0, -1, 1], 
            [1, -1, 0]]
        );
        assert_eq!(
            HashSet::from_iter(extract_empty_positions(&grid).values().cloned()),
            HashSet::from( [(2, 2), (1, 0)])
        )
    }   
     #[test]
    fn extract_empty_positions_returns_an_empty_array_on_full_grid() {

        let grid = from_array([[-1, 1, 1], [-1, -1, 1], [1, -1, -1]]);
        assert_eq!(extract_empty_positions(&grid), HashMap::new());
    }
}
