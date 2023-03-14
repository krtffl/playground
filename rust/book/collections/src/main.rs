fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let second: Option<&i32> = v.get(1);

    match second {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element!"),
    }

    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{}", n_plus_one)
    }

    for n_ref in &mut v2 {
        *n_ref += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("haha")),
        SpreadsheetCell::Float(10.12212),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
