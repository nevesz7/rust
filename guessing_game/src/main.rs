use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's play a game!");

    let number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your number guess:");

    
    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        match guess.cmp(&number) {
            Ordering::Less => println!("Try bigger"),
            Ordering::Greater => println!("Try smaller"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
