use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess = match guess.trim().parse::<u32>() {
            Ok(numb) => numb,
            Err(_) => continue,
        };
        println!("You guesses {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To Small!"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
