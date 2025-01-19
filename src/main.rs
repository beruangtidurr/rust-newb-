use std::io;

fn main() {
    println!("Guessing Games.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You guessed: {}", input);
}
