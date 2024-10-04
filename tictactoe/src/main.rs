use ndarray::{prelude::*, ViewRepr};

type Point = i8;
type Grid = ndarray::ArrayBase<ndarray::OwnedRepr<Point>, ndarray::Dim<[usize; 2]>>;
type GridSlice<'a> = ArrayBase<ViewRepr<&'a Point>, Dim<[usize; 1]>>;

#[derive(Debug, PartialEq)]
enum Player {
    Human,
    CPU,
}

type Winner = Player;

fn create_grid() -> Grid {
    Array::zeros((3, 3))
}

fn is_winning_grid(grid: Grid) -> Option<Winner> {
    let lines = grid.sum_axis(Axis(1));
    let columns = grid.sum_axis(Axis(0));

    let diag = grid.diag().sum();

    let anti_diag = (0..=2).map(|x| grid[[x, 2 - x]]).sum::<Point>();

    for val in [lines.to_vec(), columns.to_vec(), vec![diag, anti_diag]].concat() {
        match val {
            3 => return Some(Winner::Human),
            -3 => return Some(Winner::CPU),
            _ => (),
        }
    }

    None
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_grid() {
        let empty_grid = create_grid();
        assert_eq!(empty_grid, Array::zeros((3, 3)));
        assert_eq!(empty_grid, array![[0, 0, 0], [0, 0, 0], [0, 0, 0],]);
    }

    #[test]
    fn is_wining_grid_should_return_none_with_no_winner() {
        let empty_grid = create_grid();

        let result = is_winning_grid(empty_grid);
        assert!(result.is_none());
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_lines() {
        let grid = array![[0, 0, 0], [1, 1, 1], [0, 0, 0]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[0, 0, 0], [-1, -1, -1], [0, 0, 0]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::CPU);
    }
    #[test]
    fn is_wining_grid_should_return_winner_on_columns() {
        let grid = array![[1, 0, 0], [1, 0, 0], [1, 0, 0]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[-1, 0, 0], [-1, 0, 0], [-1, 0, 0]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::CPU);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_diagonal() {
        let grid = array![[1, 0, 0], [0, 1, 0], [0, 0, 1]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[-1, 0, 0], [0, -1, 0], [0, 0, -1]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::CPU);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_antidiagonal() {
        let grid = array![[0, 0, 1], [0, 1, 0], [1, 0, 0]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = array![[0, 0, -1], [0, -1, 0], [-1, 0, 0]];
        let result = is_winning_grid(grid);
        assert_eq!(result.unwrap(), Winner::CPU);
    }
}
