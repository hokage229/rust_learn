fn main() {
    // let mut  s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();

    // let mut s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // let slice = &s[0..2];
    // let slice = &s[..2];
    // let slice = &s[3..s.len()];
    // let slice = &s[3..];
    // let slice = &s[0..s.len()];
    // let slice = &s[..];

    // let s = String::from("hello world");
    // let word = first_word_new(&s[..]);
    // println!("{}", word);

    let a = [1, 2, 4, 5, 6];
    let slice = &a[0..3];
    assert_eq!(slice, &[1, 2, 4]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return i;
        };
    }
    s.len()
}

fn first_word_new(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


