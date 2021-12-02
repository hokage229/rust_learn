use std::io;

fn main() {
    let x = 5;
    println!("value: {}", x);
    let x = 6;
    println!("value: {}", x);

    let tup = (-101, 4.5, 101);
    let (x, y, z) = tup;
    let z = tup.0;
    println!("{}", z);

    let a = [1, 23, 4, 4];
    let a: [i32; 5] = [1, 3, 32, 3, 3];
    let months = ["january", "february", "april",
        "september", "november", "november", "december"];

    let b = [111; 4];
    println!("{}", b[1]);

    let a = [1, 2, 3, 4, 5];
    println!("enter an array index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index was entered not a number");

    let element = a[index];
    println!("element at index {} is {}", index, element);

    another();
}

fn another() { println!("another"); }
