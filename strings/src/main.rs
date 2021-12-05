use std::ops::Add;

fn main() {
    // let mut s = String::from("foo");
    // let s2 = "bar";
    // s.push_str(s2);
    // s.push('2');
    // println!("s2: {}", s2);

    // let s1 = String::from("Hello");
    // let s2 = String::from("World");
    // let s3 = s1.add(&s2);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{}-{}-{}", s1, s2, s3);

    // let s1 = String::from("Здравствуйте");
    // let h = &s1[0..1];
    // println!("{}", h);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
