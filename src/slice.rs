// A string slice is a reference to a part of a String. You can also use array slices.

pub fn run(){
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("{word}");

    s.clear();

    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} {world}");

    let mut s = String::from("heelo world");
    let word = first_word(&s);

    println!("{word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// You can change parameter &String to &str so you can use &String and &Str values
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

