use std::ptr::null;

#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn run(){
    let v: Vec<i32> = Vec::new();
    let mut vector_values = vec![1, 2, 3];

    vector_values.push(4);
    vector_values.push(5);
    vector_values.push(6);
    vector_values.push(7);
    vector_values.push(8);

    // Two ways to access vector values
    let third: &i32 = &vector_values[2];
    println!("The third element is {}", third);

    match vector_values.get(2){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    //  Recall the rule that states you can’t have mutable and immutable references in the
    // same scope. That rule applies in Listing 8-7, where we hold an immutable reference to the
    // first element in a vector and try to add an element to the end. This program won’t work if
    // we also try to refer to that element later in the function
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    //
    // let first = &v[0];
    //
    //
    // v.push(6);
    //
    // println!("The first element is: {}", first);

    for i in &vector_values {
        println!("{i}");
    }

    for i in &mut vector_values {
        *i += 50;
    }

    println!("{:?}", vector_values);

    // Different types on same vector with enums
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}