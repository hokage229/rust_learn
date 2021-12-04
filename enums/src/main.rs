fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(four);
    // route(six);

    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::127"),
    // };

    // let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrEnum::V6(String::from("::127"));

    // let home = IpAddrEnum::V4(127, 0, 0, 1);

    // let m = Message::Write(String::from("hello world"));
    // let m = Message::Move { y: 3, x: 4 };
    // m.call();

    // let some_number = Some(1);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;
    //
    // let opt = Option::Some(localhost);
    // let none_ip_kind :Option<IpAddrKind> = None;
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

