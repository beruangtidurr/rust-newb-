fn main() {
    // let guess : u32 = "42".parse().expect("not a number");

    // addition
    // addition
    let _sum = 5 + 10;
    println!("{_sum}");

    // subtraction
    let _difference = 95.5 - 4.3;
    println!("{_difference}");

    // multiplication
    let _product = 4 * 30;
    println!("{_product}");

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    println!("{_quotient}");
    println!("{_truncated}");

    // remainder
    let _remainder = 43 % 5;
    println!("{_remainder}");

    // boolean type

    let t = true;

    let f : bool = false;

    // char type

    let c = 'z';

    let c1 : char = 'g';
    
    // compound type

    let tup = (300, 200, 100);

    let(x,y,z) = tup;

    let total = x + y;

    println!("the value of x is {x}");
    println!("and the total of x and y is {total}");
}
