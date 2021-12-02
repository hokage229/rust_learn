fn main() {
    // let mut s = String::from("hello");
    // let len = calculate_length_ref(&s1);
    // println!("{}, length = {}", s1, len);
    // change(&s1);
    // change_mut(&mut s);

    // not compiling
    // let mut s = String::from("hello");
    // let s1 = &mut s;
    // let s2 = &mut s;
    // println!("{},{}", s1, s2);

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    //not compiling
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{},{},{}", r1, r2, r3);

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{},{}", r1, r2);
    //
    // let r3 = &mut s;
    // println!("{}", r3);

    // let ref_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

fn no_dangle(s: String) -> String {
    let s = String::from("hello");
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &String) {
    // s.push_str("ff");
}

fn change_mut(s: &mut String) {
    s.push_str("  changed");
}
