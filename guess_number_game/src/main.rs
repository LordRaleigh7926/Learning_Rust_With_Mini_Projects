extern crate rand;
use rand::Rng;
use std::io;

fn guess(x:u32, r:u32) -> bool {
    if x > r {
        println!("Too high!");
        return true;
    } if x < r {
        println!("Too low!");
        return true;
    } else {
        println!("You guessed it! The number is {}", r);
        return false;
    }

}

fn main() {

    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(0..100);
    let mut guess_count: u32 = 0;

    // println!("The secret number is: {}", random_number);

    loop {
        let mut input = String::new();

        println!("Enter an integer:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Try to parse the input as an integer (i32)
        match input.trim().parse::<i32>() {
            
            Ok(number) => {
                println!("You entered: {}", number);

                guess_count += 1;

                if !guess(number.try_into().unwrap(), random_number) {
                    println!("You guessed the number in {} tries!", guess_count);
                    break; // Exit loop if input is a valid integer
                }
            }

            Err(_) => {
                println!("Invalid input! Please enter a valid integer.");
                continue; // Ask for input again if parsing fails
            }
        }


    }

}