use grid::*;
use std::io;

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
    let mut difficulty = None;
    while difficulty.is_none() {
        println!("Please select difficulty");
        difficulty = parse_difficulty(io::stdin().lock());
    }
    println!("difficulty selected: {:?}", difficulty);

    println!("Please select something from main menu!");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Unable to read");

    match buffer.as_str() {
        "d" => (),
        "q" => std::process::exit(0),
        _ => (),
    }
}

fn ask_user_input_template(message: &str) -> io::StdinLock {
    io::stdin().lock()
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

        // let test = io::stdin().lock();
        let answer = ask_for_position(&dummy_input[..]);
        assert_eq!("I'm George", answer);
    }
}
