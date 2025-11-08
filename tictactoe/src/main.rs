use grid::*;
use std::io::{self, Write};

mod cpu;
mod grid;
mod mocktest;

fn main() {
    main_menu();
    let empty_grid = grid::create_grid();
    println!("{}", empty_grid.is_grid_full());
    println!("{:?}", empty_grid.is_winning_grid());

    let available_positions = empty_grid.extract_empty_positions();
    println!("{available_positions:?}");
    let new_grid = cpu::make_cpu_move(&empty_grid, cpu::Difficulty::Low);
    println!("New Grid: {new_grid:?}");
}

fn main_menu() {
    let input = ask_for_input();
    let input = input.trim();
    match input {
        "d" => (),
        "q" => (),
        _ => (),
    }
}

fn ask_for_input() -> String {
    io::stdout()
        .write_all(b"Please select an option from the menu:\n")
        .unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
    input.to_owned()
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
}
