fn main() {
    // println!("area = {}", area(32, 32));
    // println!("area = {}", area_cortex((32, 32)));
    // let rect = Rectangle {
    //     width: 32,
    //     height: 32,
    // };
    // println!("{:?}", rect);
    // println!("{}", rect.area());

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };
    //
    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(32);
    println!("{}", square.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_cortex(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width * self.height >= rect.width * rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
