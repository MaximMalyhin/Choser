use std::io;

fn main() {
    println!("guess the number");
    println!("enter the number");

    let mut guess_number = String::new();
    io::stdin()
        .read_line(&mut guess_number)
        .expect("something went wrong");

    println!("your number: {guess_number}");
}
