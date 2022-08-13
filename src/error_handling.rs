use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn run(){
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    //
    // v[99];

    // File implements Result type
    // let f = File::open("hello.txt");
    //
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error)
    // };

    // Matching on Different Errors
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Using match works well enough, but it can be a bit verbose and doesn’t always communicate
    // intent well. The Result<T, E> type has many helper methods defined on it to do various, more
    // specific tasks. The unwrap method is a shortcut method implemented just like the match
    // expression we wrote in Listing 9-4. If the Result value is the Ok variant, unwrap will
    // return the value inside the Ok. If the Result is the Err variant, unwrap will call the
    // panic! macro for us. Here is an example of unwrap in action:
    // let f = File::open("bye.txt").unwrap();

    // Similarly, the expect method lets us also choose the panic! error message. Using expect
    // instead of unwrap and providing good error messages can convey your intent and make tracking
    // down the source of a panic easier. The syntax of expect looks like this:
    let f = File::open("bye.txt").expect("Failed to open bye.txt");
}

// Propagating Errors
// When a function’s implementation calls something that might fail, instead of handling the error
// within the function itself, you can return the error to the calling code so that it can decide
// what to do. This is known as propagating the error and gives more control to the calling code,
// where there might be more information or logic that dictates how the error should be handled than
// what you have available in the context of your code.

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// This pattern of propagating errors is so common in Rust that Rust provides the question mark
// operator ? to make this easier.
fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}