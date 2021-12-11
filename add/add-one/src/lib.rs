#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
