use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    // let v1 = c.value(1);
    // let v2 = c.value(2);

    let ss = String::from("23");
    assert_eq!(c.value(ss), "23");
    // assert_eq!(c.value(ss), "23");

    println!("{}", ss);
}

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    //
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y))
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(&intensity));
        }
    }
}

struct Cacher<T, U, V>
    where
        T: Fn(U) -> V,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
    where
        T: Fn(U) -> V,
        U: Eq + Hash + Copy,
        V: Copy
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher { calculation, value: HashMap::new() }
    }

    fn value(&mut self, arg: U) -> &V {
        self.value.entry(arg)
            .or_insert((self.calculation)(arg))
    }
}