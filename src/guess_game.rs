use rand::Rng;
use std::io;


pub fn guess_game() {
    let rand_number = rand::thread_rng().gen_range(1..10);

    println!("Welcome!!, Let's start the game :)");

    let mut counter = 0;
    while counter < 10 {
        let mut guess = String::new();
        println!("Enter your guess : ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read !!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue; // Skip to the next iteration of the loop
            }
        };

        if rand_number == guess {
            println!("Yay!!, you have won this game !!!");
            break;
        } else {
            if rand_number > guess {
                println!("Try again!! ,your guess is smaller than the ans");
            } else {
                println!("Try again!! ,your guess is greater than the ans");
            }
        }

        counter += 1;
    }

    if counter == 10 {
        println!(
            "You have run out of chances :( , correct ans was {}",
            rand_number
        );
    }
}