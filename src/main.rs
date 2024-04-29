use std::io;

fn main() {
    println!("guess the number");
    println!("enter the number");

    let mut guess_string = String::new();
    io::stdin()
        .read_line(&mut guess_string)
        .expect("something went wrong");

    let guess_number: u32 = guess_string.trim().parse().expect("thats not a number...");

    println!("your number: {guess_number}");
}
