fn main() {
    // loop expression
    let mut n = 0;

    loop {
        println!("Hello!");
        n += 1;
        if n == 10 {
            break;
        }
    }

    let mut x = 10;
    // while loop
    while x > 0 {
        println!("{}", x);
        x -=1;
    }

    println!();

    let m = 10;
    let mut i = 1;

    while i < m{
        println!("{}", i);
        i+=1;
    }
}
