// Patterns are a special syntax in Rust for matching against the structure of types, both
// complex and simple. Using patterns in conjunction with match expressions and other constructs
// gives you more control over a program’s control flow. A pattern consists of some combination
// of the following:

// Literals
// Destructured arrays, enums, structs, or tuples
// Variables
// Wildcards
// Placeholders

// Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible
// value passed are irrefutable. An example would be x in the statement let x = 5; because x
// matches anything and therefore cannot fail to match. Patterns that can fail to match for some
// possible value are refutable. An example would be Some(x) in the expression
// if let Some(x) = a_value because if the value in the a_value variable is None rather than Some,
// the Some(x) pattern will not match.

pub fn run(){
    let x: Option<i32> = Some(5);

    // match

    match x {
        None => println!("Nothing here"),
        Some(i) => println!("Value here: {i}")
    }

    // if let (also can have regular else if)

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop(){
        println!("{}", top);
    }

    // for loop
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }

    // let Statements
    let x = 5;

    // let PATTERN = EXPRESSION;

    // To see the pattern matching aspect of let more clearly, consider Listing 18-4, which uses
    // a pattern with let to destructure a tuple.
    let (x, y, z) = (1, 2, 3);

    // Function Parameters

    let point = (3, 5);
    print_coordinates(&point);

    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching range of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring struct
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // destructuring enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    // extra conditionals with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // @ Bindings - The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.
    let msg = BidingMessage::Hello { id: 5 };

    match msg {
        BidingMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        BidingMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        BidingMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}


struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum BidingMessage {
    Hello { id: i32 },
}

