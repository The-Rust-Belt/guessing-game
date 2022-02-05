// Processing a guess:
//  Ask for user input
//  Process that input
//  Check that the input is as expected

use std::io;
use rand::Rng;

fn main() {

    println!("Guess a number:");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess: {}", guess);
}
