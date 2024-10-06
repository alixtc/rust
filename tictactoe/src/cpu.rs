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


fn is_winning_slice(slice:  ArrayBase<ndarray::ViewRepr<&i8>, Dim<[usize; 1]>>) -> bool {
    let mut s = slice.to_owned().to_vec();
    s.sort();
    s == vec![-1, -1, 0] 
}

fn extract_winning_positions(grid: &Grid) -> Vec<(usize, usize)> {
    let mut winning_position = Vec::<(usize, usize)>::new();



    for ((x, y,), _) in grid.indexed_iter().filter(|(_, val)| **val == 0) {
        if is_winning_slice(grid.row(x)) || is_winning_slice(grid.column(y))   {
            winning_position.push((x, y));
        }
    }

    if is_winning_slice(grid.diag()) {
        for (idx, value) in grid.diag().indexed_iter() {
            if *value == 0 {
                winning_position.push((idx, idx));
            }
        }
    }

    let mut anti_diag = (0..=2).map(|x| grid[[x, 2 - x]]).collect::<Vec<_>>();
    anti_diag.sort();
    if anti_diag == vec![-1, -1, 0] {
        for (idx, value) in anti_diag.iter().enumerate() {
            if *value ==0 {
                winning_position.push((idx, 2 - idx));
            }
        }
    }


    winning_position
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

    #[test]
    fn extract_winning_positions_returns_empty_array_with_no_winning_position() {
        #[rustfmt::skip]
        let grid = array![
            [1, -1, 1], 
            [-1, 0, 1], 
            [1, 1, -1]
        ];
        let positions = extract_winning_positions(&grid);
        assert_eq!(positions, Vec::new())
    

    }

    #[test]
    fn extract_winning_positions_returns_array_of_tuples() {
        #[rustfmt::skip]
        let grid = array![
            [1, 0, 1], 
            [-1, 0, -1], 
            [1, 1, -1]
        ];
        let positions = extract_winning_positions(&grid);
        assert_eq!(positions, vec![(1, 1)])
    

    }

    #[test]
    fn extract_winning_positions_work_on_diagonal_and_anti_diagonal() {
        #[rustfmt::skip]
        let grid = array![
            [-1, 1, -1], 
            [1, -1, 1], 
            [0, 0, 0]
        ];
        let positions = extract_winning_positions(&grid);
        assert_eq!(positions, vec![(2, 2), (2, 0) ])
    

    }
}
