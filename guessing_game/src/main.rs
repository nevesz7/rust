use std::io;
use rand::Rng;

fn main() {
    println!("Let's play a game!");

    let number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your number guess:");

    let mut guess = String::new(); //mut makes it so variable can change values

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed: {}", guess); //guess is coming with the \n -> how not to take it?
    println!("The secret number was: {number}");
}
