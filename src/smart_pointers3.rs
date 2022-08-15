// Interior mutability is a design pattern in Rust that allows you to mutate data even when
// there are immutable references to that data; normally, this action is disallowed by the
// borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend
// Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to the compiler
// that we’re checking the rules manually instead of relying on the compiler to check them for us.

// We can use types that use the interior mutability pattern only when we can ensure that the
// borrowing rules will be followed at runtime, even though the compiler can’t guarantee that.
// The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.
//
// Let’s explore this concept by looking at the RefCell<T> type that follows the interior
// mutability pattern.

// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the
// compiler is unable to understand and guarantee that.

use std::cell::RefCell;
use std::rc::Rc;
use crate::smart_pointers3::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn run(){

    // A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have
    // multiple owners of some data, but it only gives immutable access to that data. If you have
    // an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and
    // that you can mutate!

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger,{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

//We can’t modify the MockMessenger to keep track of the messages, because the send method takes
// an immutable reference to self. We also can’t take the suggestion from the error text to use
// &mut self instead, because then the signature of send wouldn’t match the signature in the
// Messenger trait definition (feel free to try and see what error message you get).
//
// This is a situation in which interior mutability can help!
// #[cfg(test)]
// mod tests{
//     use super::*;
//
//     struct MockMessenger{
//         sent_messages: Vec<String>
//     }
//
//     impl MockMessenger{
//         fn new() -> MockMessenger{
//             MockMessenger{
//                 sent_messages: vec![]
//             }
//         }
//     }
//
//     impl Messenger for MockMessenger{
//         fn send(&self, message: &str){
//             self.sent_messages.push(String::from(message))
//         }
//     }
//
//     #[test]
//     fn it_sends_an_over_75_percent_warning_message(){
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
//
//         limit_tracker.set_value(80);
//
//         assert_eq!(mock_messenger.sent_messages.len(), 1);
//     }
// }

// When creating immutable and mutable references, we use the & and &mut syntax, respectively.
// With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that
// belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and borrow_mut
// returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like
// regular references.


#[cfg(test)]
mod tests{
    use std::cell::RefCell;
    use super::*;

    struct MockMessenger{
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger{
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, message: &str){
            self.sent_messages.borrow_mut().push(String::from(message))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message(){
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}