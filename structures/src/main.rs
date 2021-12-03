fn main() {
    let mut user1 = User {
        username: String::from("retw"),
        email: String::from("retw@mail.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("another");

    let user2 = User {
        username: String::from("kek"),
        email: String::from("kekemaail.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let user2 = User {
        username: String::from("kek"),
        email: String::from("kekemaail.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let point = Point(1, 3, 6);
    point.0;
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct UserRef {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct Void();