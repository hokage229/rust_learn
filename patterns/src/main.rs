fn main() {
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();
    //
    // if let Some(color) = favorite_color {
    //     println!("Using your favorite color, {}, as the background", color);
    // } else if is_tuesday {
    //     println!("tuesday");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else { println!("Using blue as the background color"); }

    // let mut stack = Vec::new();
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    //
    // while let Some(top) = stack.pop() {
    //     println!("{}", top);
    // }

    // let v = vec!['a', 'b', 'c'];
    //
    // for (index, value) in v.iter().enumerate(){
    //     println!("{}:{}", index, value);
    // }

    // let point = (3, 5);
    // print_coords(&point);

    // let Some(x)= favorite_color;
}

fn print_coords(&(x, y): &(i32, i32)) {
    println!("{}, {}", x, y);
}