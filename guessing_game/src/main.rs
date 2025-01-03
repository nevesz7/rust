use std::io;

fn main() {
    println!("Please input your number guess:");

    let mut guess = String::new(); //mut makes it so variable can change values

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
