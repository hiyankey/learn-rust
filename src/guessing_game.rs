use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn main() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Result::Ok(num) => num,
            Result::Err(_) => continue,
        };
        
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
