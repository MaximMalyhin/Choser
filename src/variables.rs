const THIS_IS_CONST_OUTSIDE: u32 = 600 * 600 * 30;
pub(crate) fn variables(){
    const THIS_IS_CONST_INSIDE: u32 = 60 * 60 * 3;
    println!("The value of inside const is: {THIS_IS_CONST_INSIDE}");
    println!("The value of outside const is: {THIS_IS_CONST_OUTSIDE}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("\n");

    let x = -10;
    println!("The {x}");
    {
        let x = "Aboba";
        println!("The {x}");
    }
    println!("The {x}");

    println!("\n");

    let y: u32 = "12".parse().expect("couldnt parse");
}