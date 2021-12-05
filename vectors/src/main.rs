fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // let mut v = Vec::new();
    // v.push(4);
    // v.push(40);
    // v.push(14);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("third {}", third);
    //
    // match v.get(2) {
    //     Some(v) => println!("{}", v),
    //     None => { println!("there is no third element"); }
    // }

    // let mut v = vec![122, 33, 44, 45];
    // for i in &v {
    //     println!("{}", i);
    // }
    //
    // for i in &mut v {
    //     *i += 50;
    // }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.4),
        SpreadsheetCell::Text(String::from("default")),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}