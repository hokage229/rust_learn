use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert("Red", 101);
    // scores.insert("Blue", 50);

    // let teams = vec![String::from("Red"), String::from("Blue")];
    // let initial_scores = vec![10, 50];
    // let mut scores: HashMap<_, _> =
    //     teams.into_iter().zip(initial_scores.into_iter()).collect();

    // let mut scores = HashMap::new();
    // scores.insert(String::from("hello"), 10);
    // scores.insert(String::from("world"), 50);
    //
    // let hello = "hello".to_string();
    // let score = scores.get(&hello);
    //
    // for (key, value) in &scores {
    //     println!("{}:{}", key, value);
    // }

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{:?}", scores);

    // let text = "hello world watafak world";
    // let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:?}", map);
    let mut v = vec![1.0, 2.0, 3.0, 4.0, 0.0];
    println!("{}", average(&mut v));
    println!("{}", median(&mut v));

    let mut v = vec![1, 2, 3, 4, 0, 0];
    println!("{}", mode(&v));
}

fn average(v: &mut Vec<f64>) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

// fn mode(v: &Vec<i32>) -> i32 {
//     let mut counter = HashMap::new();
//     for value in v {
//         *counter.entry(value).or_insert(0) += 1;
//     }
// }

fn add_to_company(s: &str) {

}
