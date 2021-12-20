fn main() {
    // let x = Some(5);
    // let y = 10;
    //
    // match x {
    //     Some(50) => println!("got 50"),
    //     Some(y) => println!("matched, y = {:?}", y),
    //     _ => {println!("Default, x = {:?}", x)}
    // }
    //
    // println!("at the end: x = {:?}, y = {:?}", x, y);

    // let x = 1;
    //
    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => {}
    // }

    // let x = 5;
    // match x {
    //     1..=5 => println!("one through five"),
    //     _ => println!("something else")
    // }
    //
    // let x = 'c';
    // match x {
    //     'a'..='j' => println!("early letter"),
    //     'k'..='z' => println!("late letter"),
    //     _ => println!("something else")
    // }

    // let _p = Point { x: 0, y: 7 };

    // let Point { x: a, y: b } = p;
    // assert_eq!(0, a);
    // assert_eq!(7, b);

    // let Point { x, y } = p;
    // assert_eq!(0, x);
    // assert_eq!(7, y);

    // match p {
    //     Point { x, y: 0 } => println!("on the x axit at {}", x),
    //     Point { x: 0, y } => println!("on the y axis at {}", y),
    //     Point { x, y } => println!("on neither axis: {}, {}", x, y),
    // }

    // let msg = Message::ChangeColor(Color::Hsv(3, 3, 3));
    // match msg {
    //     Message::Quit => {}
    //     Message::Move { x, y } => {}
    //     Message::Write(text) => {}
    //     Message::ChangeColor(Color::Rgb(r, g, b)) => {}
    //     Message::ChangeColor(Color::Hsv(h, s, v)) => {}
    // }

    // let ((feet, inches), Point { x, y }) = ((3, 10), p);
    // foo(3, 5);

    // let mut setting_value = Some(5);
    // let new_setting_value = Some(10);
    //
    // match (setting_value, new_setting_value) {
    //     (Some(_), Some(_)) => println!("cant overwrite an existint customized value"),
    //
    //     _ => { setting_value = new_setting_value }
    // }
    // println!("setting is {:?}", setting_value);

    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (first, _, third, _, fifth) => {
    //         println!("some numbers:{},{},{}", fifth, third, first);
    //     }
    // }
    // let _x=5;
    // let y =10;

    // let s = Some(String::from("hello"));
    //
    // if let Some(s) = s {
    //     println!("found string")
    // }
    //
    // println!("{:?}", s)

    // let origin = Point { x: 0, y: 0, z: 0 };
    // match origin {
    //     Point { x, .. } => {
    //         println!("{}", x);
    //     }
    // }

    let num = Some(5);
    match num {
        Some(x)if x < 6 => println!("less than five"),

        Some(x) => {}
        None => {}
    }
}

// fn foo(_: i32, y: i32) {
//     println!("{}", y);
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }
//
// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }
//
struct Point {
    x: i32,
    y: i32,
    z: i32,
}