use grid::*;
use std::io::{self};

mod cpu;
mod grid;
mod mocktest;

fn main_menu() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let input = input.trim();
    match input {
        "d" => (),
        "q" => (),
        _ => (),
    }
}

fn main() {
    let empty_grid = grid::create_grid();
    println!("{}", empty_grid.is_grid_full());
    println!("{:?}", empty_grid.is_winning_grid());

    let available_positions = extract_empty_positions(&empty_grid);
    println!("{available_positions:?}")
}

fn ask_for_input() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
    println!("{input}")
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
