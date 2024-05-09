use std::cmp::Ordering;
use std::io;
use rand::Rng;

mod variables;

fn main() {

    variables::variables();

    println!("guess the number");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    loop {
        println!("enter the number");

        let mut guess_string = String::new();
        io::stdin()
            .read_line(&mut guess_string)
            .expect("something went wrong");

        if guess_string.trim() == "quit" {
            break;
        }

        let guess_number: u32 = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("thats not a number");
                continue
            }
        };

        println!("your number: {guess_number}");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you won!");
                break;
            }
        }
    }
}
