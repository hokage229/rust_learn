fn main() {
    println!("{}",fahrenheit_to_celsius(32.0))
}

fn loop_fn() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

fn while_fn() {
    let mut counter = 3;
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }
}

fn while_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("{}", a[index]);
        index += 1;
    }
}

fn for_fn() {
    let a = [10, 20, 30, 40, 50];

    for number in a.iter() {
        println!("{}", number);
    };
}

fn for_range() {
    for i in (0..5).rev() {
        println!("{}", i);
    }
}

fn fib_number(x: i32) -> i32 {
    if x == 0 { return 0; } else if x == 1 { return 1; }
    fib_number(x - 1) + fib_number(x - 2)
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}