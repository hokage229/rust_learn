fn main() {
    let number = 3;

    if number < 5 {
        println!("less than");
    } else {
        println!("not less than");
    };


    let number = 6;
    if number % 4 == 0 {
        println!("4");
    } else if number % 3 == 0 {
        println!("3");
    } else if number % 2 == 0 {
        println!("2");
    } else {
        println!("00");
    }


    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);
}
