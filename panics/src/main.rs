use std::fs::File;
use std::{fs, io};
use std::error::Error;
use std::io::{ErrorKind, Read};
use std::net::IpAddr;

fn main() {
    // panic!("crash")

    // let v = vec![1, 2, 3];
    // v[99];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(hello_file) => hello_file,
    //             Err(e) => panic!("problem creating file: {:?}", e)
    //         },
    //         other_error => {
    //             panic!("problem opening file: {:?}", other_error)
    //         }
    //     }
    // };

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("problem creating file: {:?}", error)
    //         })
    //     } else {
    //         panic!("problem opening file: {:?}", error)
    //     }
    // });

    // let f = File::open("hello1.txt").unwrap();
    // let f = File::open("hello1.txt").expect("failed to open");

    // let result = read_username_from_file();

    // let f = File::open("hello.txt")?;
    // Ok(())

    // let home: IpAddr = "127.0.0.1".parse().unwrap();

    let guess = Guess::new(3);
    println!("{}", guess.value());

    let guess = Guess {
        value: 3
    };
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guess number must be between 1 and 100");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e)
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e)
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file()->Result<String,io::Error>{
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}