// Processing a guess:
//  Ask for user input
//  Process that input
//  Check that the input is as expected

use std::io;

fn main() {

    println!("Guess a number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess: {}", guess);
}
