use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut random_generator = thread_rng();

    let secret_number = random_generator.gen_range(1, 101); // [1, 101)

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        println!("You guessed: {}", guess);
        // This binding *shadows* the previous binding.
        // trim line character.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
