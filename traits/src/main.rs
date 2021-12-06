use std::fmt::{Debug, Display, format};

fn main() {
    let tweet = Tweet {
        username: String::from("retween"),
        content: String::from("hello"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})",
                self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

// pub fn notify<T: Summary>(item: &T) {
//     println!("breaking news! {}", item.summarize());
// }

// pub fn notify(item: &(impl Summary + Display)) {}

// pub fn notify<T: Summary + Display>(item: &T) {}

// fn some_function<T: Display + Clone, U: Clone + Debug>
// (t: &T, u: &U) -> i32 { 0 }

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ 0 }

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("retween"),
        content: String::from("hello"),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("{}", self.x);
        } else {
            println!("{}", self.y);
        }
    }
}