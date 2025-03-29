use std::io;

fn main() {
    // array with datatype
    let numArr : [i32;5] = [1,2,3,4,5]; 

    print!("{:?}", numArr);

    // array without datatype
    let numArr2 = [1,2,3,4,5];

    print!("{:?}", numArr2);

    // array with default values
    let numArr3 : [i32;5] = [3;5];

    print!("{:?}", numArr3);


    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed");
    let length: usize = input.trim().parse().expect("please input");

    let mut arr: Vec<i32> = Vec::new();

    for i in 0..length {
        input.clear();
        println!("Enter value {}:", i + 1);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Please enter a valid integer");
        arr.push(num);
    }

    print!("Your array : {:?}", arr);
}
