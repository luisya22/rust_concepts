// The main aim of lifetimes is to prevent dangling references, which cause a program to reference
// data other than the data it’s intended to reference. Consider the program in Listing 10-16,
// which has an outer scope and an inner scope.

use std::fmt::Display;

pub fn run(){
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// This does not work because Rust does not know what reference you will return.
// We also don’t know the concrete lifetimes of the references that will be passed in
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
