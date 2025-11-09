use grid::*;
use std::io::{self};

use crate::cpu::parse_difficulty;

mod cpu;
mod grid;
mod mocktest;

fn main() {
    main_menu();
    let empty_grid = grid::create_grid();
    println!("Is grid full? {}", empty_grid.is_grid_full());
    println!("Is grid wining? {:?}", empty_grid.is_winning_grid());

    let available_positions = empty_grid.extract_empty_positions();
    println!("{available_positions:?}");
    let new_grid = cpu::make_cpu_move(&empty_grid, cpu::Difficulty::Low);
    println!("New Grid:\n{}", new_grid.render());
}

fn main_menu() {
    let selected_difficulty = get_user_input_with(
        parse_difficulty,
        || io::stdin().lock(),
    );

    println!("Selected difficulty is: {:?}", selected_difficulty);

    println!("Please select something from main menu!");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Unable to read");

    match buffer.as_str() {
        "d" => (),
        "q" => std::process::exit(0),
        _ => (),
    }
}

fn get_user_input_with<F, G, R, T>(parser: F, mut reader: G) -> T
where
    G: FnMut() -> R,
    R: io::BufRead,
    F: Fn(R) -> Option<T>,
{
    let mut input = None;
    while input.is_none() {
        input = parser(reader());
    }
    input.unwrap()
}

pub fn ask_for_position<R>(mut reader: R) -> String
// Required to facilitate testing
where
    R: io::BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).expect("Unable to read");
    buffer
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
    fn test_get_user_input_with_consecutive_invalid_then_valid_input() {
        let mut mock_inputs = vec!["invalid\n", "l\n"];

        let get_mock_reader = move || {
            let input_str = mock_inputs.remove(0);
            std::io::Cursor::new(input_str.as_bytes())
        };

        let selected_difficulty = get_user_input_with(
            cpu::parse_difficulty,
            get_mock_reader,
        );

        assert_eq!(selected_difficulty, crate::cpu::Difficulty::Low);
    }
}
