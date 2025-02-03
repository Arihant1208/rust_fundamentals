use rand::Rng;
use std::io;


pub fn inital_stuff() {
    println!("Hello, world!");
    println!("Enter the number : ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("secret number is  =  {} ", secret_number);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read this line");

    println!("you guessed : {} ", guess);

    let mut tup = (1, 2, 3, 4.2);

    tup.0 = 4;

    println!("{}", tup.0);

    let mut arr = [1, 2, 3, 4, 5];

    arr[2] = 10;

    println!("{}", arr[2]);
}