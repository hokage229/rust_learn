use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("guess the number\ninput you guess");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("too small"); }
            Ordering::Greater => { println!("too big"); }
            Ordering::Equal => {
                println!("you win");
                break;
            }
        };
    };
}