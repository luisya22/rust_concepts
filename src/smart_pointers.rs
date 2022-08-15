// Smart pointers, on the other hand, are data structures that act like a pointer but also have
// additional metadata and capabilities.

// Box<T> for allocating values on the heap
// Boxes don’t have performance overhead, other than storing their data on the heap instead of on
// the stack. But they don’t have many extra capabilities either. You’ll use them most often in
// these situations:

// -When you have a type whose size can’t be known at compile time and you want to use a value of
// that type in a context that requires an exact size
// -When you have a large amount of data and you want to transfer ownership but ensure the data
// won’t be copied when you do so
// -When you want to own a value and you care only that it’s a type that implements a particular
// trait rather than being of a specific type

// Rc<T>, a reference counting type that enables multiple ownership

// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules
// at runtime instead of compile time



use std::ops::Deref;
use crate::smart_pointers::List::{Cons, Nil};

pub fn run(){

    // Using a Box<T> to Store Data on the Heap

    // Small example not really useful, you won't store a single i32 value to the heap
    let b = Box::new(5);
    println!("b = {}", b);

    // Enabling Recursive Types with Boxes
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    // Treating Smart Pointers Like Regular References with the Deref Trait

    //Following the Pointer to the Value
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using Box<T> Like a Reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Define Our Own Smart Pointer
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implicit Deref Coercions with Functions and Methods
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Running Code on Cleanup with the Drop Trait
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Dropping a Value Early with std::mem::drop
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}


enum List{
    Cons(i32, Box<List>),
    Nil
}

// Defining Our Own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

// Running Code on Cleanup with the Drop Trait
struct CustomSmartPointer{
    data: String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}