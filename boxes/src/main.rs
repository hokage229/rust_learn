use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::slice::IterMut;
use crate::List::{Cons, Nil};

fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    // let list = Cons(1, Box::new(
    //     Cons(2, Box::new(
    //         Cons(3, Box::new(Nil))))));

    // let x = "5";
    // let y = &x;
    // let y = MyBox::new(x);
    // let m = MyBox::new(String::from("rust"));
    //
    // assert_eq!("5", x);
    // assert_eq!("5", *y);
    //
    // hello(&y);
    // hello(x);
    // hello(&m);

    // let c = CustomSmartPointer {
    //     data: String::from("my stuff")
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff")
    // };
    // drop(d);
    // println!("created");

    // let value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // // println!("{}", Rc::strong_count(&a));
    //
    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // // println!("{}", Rc::strong_count(&a));
    //
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    //
    // *value.borrow_mut() += 10;
    //
    // println!("a: {:?}", a);
    // println!("b: {:?}", b);
    // println!("c: {:?}", c);
    // println!("{}", Rc::strong_count(&a));

    // println!("{}", Rc::strong_count(&a));

    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());
    //
    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    //
    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());
    //
    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }
    //
    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // println!("leaf parent {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
enum List {
    // Cons(Rc<RefCell<i32>>, Rc<List>),
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("warning: you ve used up over 90");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("warning: you ve used up over 75");
        }
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping with data {}", self.data);
    }
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {}!", name);
}