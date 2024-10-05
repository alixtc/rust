use grid::*;

mod cpu;
mod grid;

fn main() {
    let empty_grid = grid::create_grid();
    println!("{}", empty_grid.is_grid_full());
    println!("{:?}", empty_grid.is_winning_grid());

    let available_positions = extract_empty_positions(&empty_grid);
    println!("{available_positions:?}")

    // let mut input = String::new();
    // println!("Please select one of the empty positions to play: {available_positions:?}");
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Unable to read user input");
    // println!("{input}")
}
