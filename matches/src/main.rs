use std::cmp::Ordering;

fn main() {
    // println!("{}", Coin::Penny.value_in_cents());
    //
    // let coin = Coin::Quarter(UsState::Colorado);
    // println!("{}", Coin::value_in_cents_of_coin(coin));

    // let five = Some(5);
    // let six = plus_one(five);
    // let non = plus_one(None);

    // let some_u8_value = Some(32u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    // let some_u8_value = Some(0u8);
    // if let Some(3) = some_u8_value { println!("three"); }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Colorado);
    match &coin {
        Coin::Quarter(state) => { println!("{:?}", state) }
        _ => count += 1,
    }
    if let Coin::Quarter(state) = &coin { println!("{:?}", state) } else { count += 1 }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => 25
        }
    }
    fn value_in_cents_of_coin(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("penny");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                print!("state {:?} ", state);
                25
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington,
    Colorado,
    California,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1)
    }
}

fn print_some_i8_value(value: i8) {
    match value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => (),
    }
}