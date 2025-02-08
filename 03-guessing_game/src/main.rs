use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("guess the number!");

    // docs use: rand::thread_rng().gen_range(1..=100);
    // but in version 0.9.0 it is recommended to use rng().random_range(1..=100) instead.
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("please input your guess.");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line!");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("to big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
