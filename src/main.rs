fn main() {
    // without mut it'll give an error tha means x's value cannot be changed.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // Constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of Y in the inner scope is : {y}");
    }

    println!("the value of y is {y}");

    // Length
    // let space = "  ";
    // space = space.len();

}