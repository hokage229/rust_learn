use std::fmt::Display;

fn main() {
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {}", r);

    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("longest is {}", result);

    let novel = String::from("call me. some year");
    let first_sentence = novel.split('.').next().expect("could not find .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where T: Display,
{
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
