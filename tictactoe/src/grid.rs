use itertools::*;

use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Player {
    Human,
    Cpu,
}

type Winner = Player;

type ManualGrid = [[i32; 3]; 3];
#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    pub grid: HashMap<(i32, i32), i32>,
    pub size: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Marker {
    X = 1,
    O = -1,
}

impl From<ManualGrid> for Grid {
    fn from(array: ManualGrid) -> Grid {
        from_array(array)
    }
}

pub trait GridChecker {
    fn is_grid_full(&self) -> bool;
    fn is_winning_grid(&self) -> Option<Winner>;
    fn insert(&mut self, key: (i32, i32), value: i32);
    fn extract_winning_positions(&self, marker: &Marker) -> Vec<(i32, i32)>;
    fn extract_empty_positions(&self) -> HashMap<usize, (i32, i32)>;
}

pub fn from_array(array: ManualGrid) -> Grid {
    let mut grid = HashMap::new();
    for (ix_row, row) in array.iter().enumerate() {
        for (ix_col, value) in row.iter().enumerate() {
            grid.insert((ix_row as i32, ix_col as i32), value.to_owned());
        }
    }
    Grid { grid, size: 3 }
}

pub fn create_grid() -> Grid {
    let mut grid = HashMap::new();
    for x in 0..=8 {
        grid.insert((x / 3, x % 3), 0);
    }
    Grid { grid, size: 3 }
}

impl GridChecker for Grid {
    fn is_winning_grid(&self) -> Option<Winner> {
        let local_grid = &self.grid;
        let lines: Vec<i32> = local_grid
            .iter()
            .into_grouping_map_by(|((row, _), _)| row)
            .fold(0, |acc, _key, (_, val)| acc + val)
            .into_values()
            .collect();
        let columns: Vec<i32> = local_grid
            .iter()
            .into_grouping_map_by(|((_, col), _)| col)
            .fold(0, |acc, _key, (_, val)| acc + val)
            .into_values()
            .collect();

        let diag = local_grid
            .iter()
            .filter(|((row, col), _)| row == col)
            .map(|((_, _), val)| val)
            .sum();

        let anti_diag = local_grid
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
        self.grid.values().filter(|x| **x == 0).count() == 0
    }

    fn insert(&mut self, key: (i32, i32), value: i32) {
        let mut new_grid = self.grid.clone();
        new_grid.insert(key, value);
        self.grid = new_grid;
    }

    fn extract_winning_positions(&self, marker: &Marker) -> Vec<(i32, i32)> {
        let mut winning_position = Vec::<(i32, i32)>::new();

        for ((x, y), _) in self.grid.iter().filter(|(_, val)| **val == 0) {
            let mut attempt_grid = self.clone();
            attempt_grid.insert((*x, *y), *marker as i32);
            if attempt_grid.is_winning_grid().is_some() {
                winning_position.push((*x, *y));
            }
        }

        winning_position
    }

    fn extract_empty_positions(&self) -> HashMap<usize, (i32, i32)> {
        extract_empty_positions(self)
    }
}

fn extract_empty_positions(grid: &Grid) -> HashMap<usize, (i32, i32)> {
    grid.grid
        .iter()
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
        let grid = Grid::from([[0, 0, 0], [1, 1, 1], [0, 0, 0]]);
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
        let grid = from_array([[-1, 1, 1], [0, -1, 1], [1, -1, 0]]);
        assert_eq!(
            HashSet::from_iter(grid.extract_empty_positions().values().cloned()),
            HashSet::from([(2, 2), (1, 0)])
        )
    }
    #[test]
    fn extract_empty_positions_returns_an_empty_array_on_full_grid() {
        let grid = from_array([[-1, 1, 1], [-1, -1, 1], [1, -1, -1]]);
        assert_eq!(grid.extract_empty_positions(), HashMap::new());
    }
}
