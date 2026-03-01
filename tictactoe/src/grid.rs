use itertools::*;

use std::collections::HashMap;
use std::convert::From;
use std::io;

#[derive(Debug, PartialEq)]
pub enum Player {
    Human,
    Cpu,
}

type Winner = Player;
type ManualGrid = [[i32; 3]; 3];
type Glyph = [String; 3];

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    pub grid: HashMap<(i32, i32), Marker>,
    pub size: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Marker {
    X = 1,
    O = -1,
    Null = 0,
}

impl Marker {
    pub fn to_int(self) -> i32 {
        match self {
            Marker::X => 1,
            Marker::O => -1,
            Marker::Null => 0,
        }
    }
}

pub trait GridChecker {
    fn is_grid_full(&self) -> bool;
    fn is_winning_grid(&self) -> Option<Winner>;
    fn insert(&mut self, key: (i32, i32), value: Marker);
    fn extract_winning_positions(&self, marker: &Marker) -> Vec<(i32, i32)>;
    fn extract_empty_positions(&self) -> HashMap<usize, (i32, i32)>;
}

impl Grid {
    pub fn render(&self) -> String {
        let glyph_list_by_row = self.regroup_glyphs_by_row();

        let joined_row_glyphs = glyph_list_by_row
            .iter()
            .sorted_by_key(|(key, _)| *key)
            .map(|(_, glyphs)| glyphs.join("|"))
            .collect::<Vec<_>>()
            .join("\n---+---+---\n");

        format!("\n{}\n", joined_row_glyphs)
    }

    fn regroup_glyphs_by_row(&self) -> HashMap<i32, Vec<String>> {
        self.grid
            .iter()
            .sorted_by_key(|((x, y), _)| (x, y))
            .enumerate()
            .map(|(idx, ((x, y), val))| {
                (
                    x,
                    y,
                    match val {
                        Marker::Null => format!(" {glyph} ", glyph = idx + 1),
                        Marker::X => " X ".to_owned(),
                        Marker::O => " O ".to_owned(),
                    },
                )
            })
            .into_grouping_map_by(|(row, _, _)| **row)
            .fold(Vec::new(), |mut acc, _key, val| {
                acc.push(val.2.to_owned());
                acc
            })
    }

    fn convert_to_glyphs(&self) -> Vec<Glyph> {
        self.grid
            .iter()
            .sorted_by_key(|((x, y), _)| (x, y))
            .map(|((x, y), marker)| match marker {
                Marker::O => O_GLYPH.map(|x| x.to_owned()),
                Marker::X => X_GLYPH.map(|x| x.to_owned()),
                Marker::Null => [
                    "       ".to_owned(),
                    format!("   {}   ", x * 3 + (y + 1)),
                    "       ".to_owned(),
                ],
            })
            .collect()
    }

    fn render_grid_from_glyphs(&self) -> String {
        self.convert_to_glyphs()
            .chunks(3)
            .map(|row| {
                (0..3)
                    .map(|glyph_subline| {
                        row.iter()
                            .map(move |glyph| glyph[glyph_subline].to_owned())
                            .join(" | ")
                    })
                    .join("\n")
            })
            .join("\n========+=========+========\n")
    }
}

impl From<ManualGrid> for Grid {
    fn from(array: ManualGrid) -> Grid {
        let mut grid = HashMap::new();
        for (ix_row, row) in array.iter().enumerate() {
            for (ix_col, value) in row.iter().enumerate() {
                grid.insert(
                    (ix_row as i32, ix_col as i32),
                    match value {
                        1 => Marker::X,
                        -1 => Marker::O,
                        _ => Marker::Null,
                    },
                );
            }
        }
        Grid { grid, size: 3 }
    }
}

pub fn create_grid() -> Grid {
    let mut grid = HashMap::new();
    for x in 0..=8 {
        grid.insert((x / 3, x % 3), Marker::Null);
    }
    Grid { grid, size: 3 }
}

impl GridChecker for Grid {
    fn is_winning_grid(&self) -> Option<Winner> {
        let sum_per_row: Vec<i32> = self
            .grid
            .iter()
            .into_grouping_map_by(|((row, _), _)| row)
            .fold(0, |acc, _key, (_, mk)| acc + mk.to_int())
            .into_values()
            .collect();
        let sum_per_column: Vec<i32> = self
            .grid
            .iter()
            .into_grouping_map_by(|((_, col), _)| col)
            .fold(0, |acc, _key, (_, mk)| acc + mk.to_int())
            .into_values()
            .collect();

        let diagonal_sum = self
            .grid
            .iter()
            .filter(|((row, col), _)| row == col)
            .map(|((_, _), mk)| mk.to_int())
            .sum();

        let anti_diagonal_sum = self
            .grid
            .iter()
            .filter(|(row_col, _)| [(2, 0), (1, 1), (0, 2)].contains(row_col))
            .map(|(_, mk)| mk.to_int())
            .sum();

        for val in [
            sum_per_row,
            sum_per_column,
            vec![diagonal_sum, anti_diagonal_sum],
        ]
        .concat()
        {
            match val {
                3 => return Some(Winner::Human),
                -3 => return Some(Winner::Cpu),
                _ => (),
            }
        }

        None
    }

    fn is_grid_full(&self) -> bool {
        self.grid.values().filter(|x| **x == Marker::Null).count() == 0
    }

    fn insert(&mut self, key: (i32, i32), value: Marker) {
        let mut new_grid = self.grid.clone();
        new_grid.insert(key, value);
        self.grid = new_grid;
    }

    fn extract_winning_positions(&self, marker: &Marker) -> Vec<(i32, i32)> {
        let mut winning_position = Vec::<(i32, i32)>::new();

        for ((x, y), _) in self.grid.iter().filter(|(_, val)| **val == Marker::Null) {
            let mut attempt_grid = self.clone();
            attempt_grid.insert((*x, *y), *marker);
            if attempt_grid.is_winning_grid().is_some() {
                winning_position.push((*x, *y));
            }
        }

        winning_position
    }

    fn extract_empty_positions(&self) -> HashMap<usize, (i32, i32)> {
        self.grid
            .iter()
            .sorted_by_key(|((row, col), _)| (row, col))
            .enumerate()
            .filter_map(|(flat_idx, (row_col_idx, val))| {
                if *val == Marker::Null {
                    Some((flat_idx + 1, row_col_idx.to_owned()))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, (_, _)>>()
    }
}

pub fn make_user_turn<R, G>(grid: &Grid, mut reader: G) -> Grid
where
    G: FnMut() -> R,
    R: io::BufRead,
{
    let empty_postions = &grid.extract_empty_positions();
    let list_of_choices = &empty_postions
        .keys()
        .map(|x| x.to_string())
        .sorted()
        .collect::<Vec<_>>()
        .join(", ");
    let mut positions: Option<(i32, i32)> = None;
    while positions.is_none() {
        println!("{}", grid.render_grid_from_glyphs());
        println!(
            "Please select one of the available positions:\n{}",
            list_of_choices
        );
        let mut string_buffer = String::new();
        reader()
            .read_line(&mut string_buffer)
            .expect("Unable to read user input during play turn");
        let candidate: usize = string_buffer.trim().parse().unwrap_or(0);
        positions = empty_postions.get(&candidate).cloned();
    }
    let final_coordinates: (i32, i32) = positions.unwrap();
    let mut grid_after_move = grid.clone();
    grid_after_move.insert(final_coordinates, Marker::X);
    grid_after_move
}

#[rustfmt::skip]
const X_GLYPH: [&str; 3] = [
    r" \\ // ", 
    r"   X   ", 
    r" // \\ "
];

#[rustfmt::skip]
const O_GLYPH: [&str; 3] = [
    r" //‾\\ ",
    r"(     )", 
    r" \\_// ",
];

#[rustfmt::skip]
const EMPTY_GLYPH: [&str; 3] = [
    r"       ", 
    r"       ", 
    r"       ",
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn markers_shoud_be_convertible_to_integers() {
        assert_eq!(Marker::X.to_int(), 1);
        assert_eq!(Marker::O.to_int(), -1);
        assert_eq!(Marker::Null.to_int(), 0);
    }

    #[test]
    fn creates_an_empty_grid() {
        let empty_grid = create_grid();
        assert_eq!(empty_grid, Grid::from([[0, 0, 0], [0, 0, 0], [0, 0, 0]]));
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

        let grid = Grid::from([[0, 0, 0], [-1, -1, -1], [0, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_wining_grid_should_report_human_wins() {
        let grid = Grid::from([[0, 0, 1], [0, 1, -1], [1, 0, -1]]);
        let result = grid.is_winning_grid().unwrap();
        assert_eq!(result, Winner::Human);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_columns() {
        let grid = Grid::from([[1, 0, 0], [1, 0, 0], [1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = Grid::from([[-1, 0, 0], [-1, 0, 0], [-1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_diagonal() {
        let grid = Grid::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = Grid::from([[-1, 0, 0], [0, -1, 0], [0, 0, -1]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_wining_grid_should_return_winner_on_antidiagonal() {
        let grid = Grid::from([[0, 0, 1], [0, 1, 0], [1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Human);

        let grid = Grid::from([[0, 0, -1], [0, -1, 0], [-1, 0, 0]]);
        let result = grid.is_winning_grid();
        assert_eq!(result.unwrap(), Winner::Cpu);
    }

    #[test]
    fn is_grid_full_should_detect_empty_slots() {
        let grid = Grid::from([[0, 0, 1], [0, 0, 0], [1, 0, 0]]);
        assert!(!grid.is_grid_full());
    }
    #[test]
    fn is_grid_full_returns_true_with_no_empty_spots() {
        let grid = Grid::from([[-1, -1, 1], [-1, -1, -1], [1, -1, -1]]);
        assert!(grid.is_grid_full());
    }

    #[test]
    fn extract_empty_positions_returns_an_array() {
        let grid = Grid::from([[-1, 1, 1], [0, -1, 1], [1, -1, 0]]);
        assert_eq!(
            HashSet::from_iter(grid.extract_empty_positions().values().cloned()),
            HashSet::from([(2, 2), (1, 0)])
        )
    }

    #[test]
    fn extract_empty_positions_should_be_in_row_col_order() {
        let grid = Grid::from([[-1, 1, 1], [0, -1, 1], [1, -1, 0]]);
        assert_eq!(
            grid.extract_empty_positions()
                .keys()
                .cloned()
                .sorted()
                .collect::<Vec<_>>(),
            vec![4, 9]
        )
    }

    #[test]
    fn extract_empty_positions_returns_an_empty_array_on_full_grid() {
        let grid = Grid::from([[-1, 1, 1], [-1, -1, 1], [1, -1, -1]]);
        assert_eq!(grid.extract_empty_positions(), HashMap::new());
    }

    #[test]
    fn render_should_have_column_separator() {
        let grid = Grid::from([[-1, 1, 1], [1, -1, 1], [1, -1, -1]]);
        assert!(grid.render().contains(" O | X | X "));
        assert!(grid.render().contains(" X | O | X "));
        assert!(grid.render().contains(" X | O | O "));
    }

    #[test]
    fn render_should_display_empty_position_as_numbers() {
        let grid = Grid::from([[0, 1, 1], [0, -1, 1], [0, -1, 0]]);
        assert!(grid.render().contains(" 1 | X | X "));
        assert!(grid.render().contains(" 4 | O | X "));
        assert!(grid.render().contains(" 7 | O | 9 "));
    }

    #[test]
    fn render_should_have_rows_consecutively() {
        let grid = Grid::from([[0, 1, 1], [0, -1, 1], [0, -1, 0]]);

        assert!(grid
            .render()
            .contains("-\n 4 | O | X \n---+---+---\n 7 | O | 9 "));
    }
    #[test]
    fn render_should_have_row_separator() {
        let grid = Grid::from([[-1, 1, 1], [-1, -1, 1], [1, -1, -1]]);
        assert!(grid.render().contains("\n---+---+---\n"));
    }

    #[test]
    fn render_should_start_and_end_with_line_breaks() {
        let grid = Grid::from([[-1, 1, 1], [-1, -1, 1], [1, -1, -1]]);
        let rendered = grid.render();
        let first_char = rendered.chars().next().unwrap();
        let last_char = rendered.chars().last().unwrap();
        assert_eq!(first_char, '\n');
        assert_eq!(last_char, '\n');
    }

    #[test]
    fn make_user_turn_should_fill_one_empty_position_in_grid() {
        let original_grid = Grid::from([[1, -1, 0], [1, -1, 0], [0, 0, 0]]);

        let get_mock_reader = || std::io::Cursor::new("3".as_bytes());

        let grid_after_turn = make_user_turn(&original_grid, get_mock_reader);
        assert!(
            grid_after_turn.extract_empty_positions().len()
                < original_grid.extract_empty_positions().len()
        )
    }

    #[test]
    fn make_user_turn_should_prompt_until_user_selects_an_empty_position() {
        let mut mock_inputs = vec!["invalid", "l", "1"];

        let get_mock_reader = || {
            let input_str = mock_inputs.remove(0);
            std::io::Cursor::new(input_str.as_bytes())
        };

        let grid = Grid::from([[0, -1, 0], [1, -1, 0], [0, 0, 0]]);
        let filled_grid = make_user_turn(&grid, get_mock_reader);

        let new_empty_positions = filled_grid
            .extract_empty_positions()
            .keys()
            .cloned()
            .sorted()
            .collect::<Vec<usize>>();
        assert_eq!(new_empty_positions, vec![3, 6, 7, 8, 9]);
    }

    #[test]
    fn make_convert_to_glyphs_return_o_and_x_glyphs() {
        let grid = Grid::from([[0, -1, 0], [1, -1, 0], [0, 0, 0]]);

        let glyphs = grid.convert_to_glyphs();

        assert_eq!(glyphs[1], O_GLYPH);
        assert_eq!(glyphs[3], X_GLYPH);
    }

    #[test]
    fn make_convert_to_glyphs_have_indexes_for_empty_glyphs() {
        let grid = Grid::from([[0, -1, 0], [1, -1, 0], [0, 0, 0]]);

        let glyphs = grid.convert_to_glyphs();

        assert_eq!(glyphs[0], [r"       ", r"   1   ", r"       "]);
        assert_eq!(glyphs[2], [r"       ", r"   3   ", r"       "]);
        assert_eq!(glyphs[5], [r"       ", r"   6   ", r"       "]);
        assert_eq!(glyphs[8], [r"       ", r"   9   ", r"       "]);
    }

    #[test]
    fn make_show_grid() {
        let grid = Grid::from([[0, -1, 0], [1, -1, 0], [0, 0, 0]]);

        println!("{}", grid.render_grid_from_glyphs());
    }
}
