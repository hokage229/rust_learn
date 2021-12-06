fn main() {
    // let number_list = vec![1, 234, 55, 32, 3];
    // println!("{}", largest_i32(&number_list));

    // let integer = Point { x: 5, y: 4 };
    // let integer = Point { x: 5.0, y: 4 };

    // let s = Option::Some(3);

    // let integer = Point { x: 5, y: 4 };
    // println!("{}", integer.x());

    // let float = Point { x: 2.2 as f32, y: 4.4 };
    // println!("{}", float.distance_from_origin())

    let p1 = Point { x: 5, y: 10.5 };
    let p2 = Point { x: "hello", y: 'q' };
    let p3 = p1.mixup(p2);

    println!("{},{}", p3.x, p3.y);
}

fn largest_i32(v: &[i32]) -> i32 {
    let mut largest = v[0];
    for &number in v {
        if number > largest {
            largest = number;
        };
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest<T: PartialOrd>(list:&[T])->T{
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}