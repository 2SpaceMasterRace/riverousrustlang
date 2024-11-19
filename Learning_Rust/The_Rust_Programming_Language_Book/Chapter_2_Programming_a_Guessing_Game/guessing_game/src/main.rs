use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to the Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input a guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read line");

        // let guess: i32 = guess.trim().parse().expect("Please input a valid integer!");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
            };

        println!("You have entered: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }
        println!("The secret number is {secret_number}");
    }
    }
