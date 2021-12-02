fn main() {
    another(5, 6);

    let y = {
        let x = 3;
        x + 4
    };
    // println!("{}", y);

    println!("{}", increment(five()));
}

fn another(x: i32, y: i32) {
    // println!("{}", x);
    // println!("{}", y);
}

fn five() -> i32 {
    6
}

fn increment(x: i32) -> i32 {
    x + 1
}