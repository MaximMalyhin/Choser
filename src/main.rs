use std::io;
use rand::Rng;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    priintln!("the secret number is {secret_number}");

    println!("enter the number");

    let mut guess_number = String::new();
    io::stdin()
        .read_line(&mut guess_number)
        .expect("something went wrong");

    println!("your number: {guess_number}");
}
