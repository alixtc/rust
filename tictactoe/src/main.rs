use grid::*;
use std::io::{self};

use crate::cpu::parse_difficulty;

mod cpu;
mod grid;
mod mocktest;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Leaderboard {
    cpu: i32,
    player: i32,
}

impl Leaderboard {
    fn update_score(&self, player: Option<Player>) -> Leaderboard {
        if player.is_none() {
            return self.to_owned();
        }
        let winner = player.unwrap();
        match winner {
            Player::Cpu => Leaderboard {
                cpu: self.cpu + 1,
                player: self.player,
            },
            Player::Human => Leaderboard {
                cpu: self.cpu,
                player: self.player + 1,
            },
        }
    }
}

fn main() {
    main_menu();
}

fn print_main_screen_menu(selected_difficulty: cpu::Difficulty) {
    println!(
        "
Please select something from main menu!
1 (s) - Start Game
2 (d) - Set Difficulty (currently selected: {difficulty:?})
3 (l) - Leaderboard and Score Display
4 (q) - Quit
",
        difficulty = selected_difficulty
    );
}

fn main_menu() {
    let mut selected_difficulty = cpu::Difficulty::Medium;
    let mut leaderboard = Leaderboard { cpu: 0, player: 0 };
    loop {
        print_main_screen_menu(selected_difficulty);
        let user_input = ask_user_input(|| io::stdin().lock())
            .trim()
            .to_lowercase()
            .to_owned();

        if (user_input == "s") | (user_input == "1") {
            let winner = play_game(selected_difficulty);
            leaderboard = leaderboard.update_score(winner);
        }
        if (user_input == "d") | (user_input == "2") {
            selected_difficulty = get_user_input_with(parse_difficulty, || io::stdin().lock());
        }
        if (user_input == "q") | (user_input == "4") {
            std::process::exit(0);
        }
    }
}

fn play_game(difficulty: cpu::Difficulty) -> Option<grid::Player> {
    println!("Starting a new game!");
    let mut game_grid = grid::create_grid();

    while !game_grid.is_grid_full() {
        game_grid = grid::make_user_turn(&game_grid, || io::stdin().lock());

        if game_grid.is_grid_full() || game_grid.is_winning_grid().is_some() {
            return game_grid.is_winning_grid();
        }

        game_grid = cpu::make_cpu_move(&game_grid, difficulty);

        if game_grid.is_grid_full() || game_grid.is_winning_grid().is_some() {
            return game_grid.is_winning_grid();
        }
    }
    None
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

pub fn ask_user_input<G, R>(mut reader: G) -> String
where
    G: FnMut() -> R,
    R: io::BufRead,
{
    let mut buffer = String::new();
    reader().read_line(&mut buffer).expect("Unable to read");
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_user_input() {
        let mock_reader = move || "I'm George".as_bytes();
        let answer = ask_user_input(mock_reader);
        assert_eq!("I'm George", answer);
    }

    #[test]
    fn test_get_user_input_with_consecutive_invalid_then_valid_input() {
        let mut mock_inputs = vec!["invalid\n", "l\n"];

        let get_mock_reader = move || {
            let input_str = mock_inputs.remove(0);
            std::io::Cursor::new(input_str.as_bytes())
        };

        let selected_difficulty = get_user_input_with(cpu::parse_difficulty, get_mock_reader);

        assert_eq!(selected_difficulty, crate::cpu::Difficulty::Low);
    }

    #[test]
    fn update_score_should_not_change_without_player() {
        let board = Leaderboard { cpu: 1, player: 3 };
        let player: Option<Player> = None;
        assert_eq!(board, board.update_score(player));
    }

    #[test]
    fn update_score_should_increase_score_for_winning_player() {
        let board = Leaderboard { cpu: 1, player: 3 };
        let player = Some(Player::Human);
        assert_eq!(&board.player + 1, board.update_score(player).player);
        let player = Some(Player::Cpu);
        assert_eq!(&board.cpu + 1, board.update_score(player).cpu);
    }
}
