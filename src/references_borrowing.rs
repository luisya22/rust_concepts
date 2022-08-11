
pub fn run(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");

    // You can't have more than one mutable reference to the same variable

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    // Note that a reference’s scope starts from where it is introduced and continues through
    // the last time that reference is used. For instance, this code will compile because
    // the last usage of the immutable references, the println!, occurs before the mutable
    // reference is introduced:


    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have rust_concepts of what
// it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}