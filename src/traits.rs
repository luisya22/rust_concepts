// A trait defines functionality a particular type has and can share with other types. We can use
// traits to define shared behavior in an abstract way. We can use trait bounds to specify that a
// generic type can be any type that has certain behavior.
//
// Note: Traits are similar to a feature often called interfaces in other languages, although with
// some differences.

mod traits_summary_example;

use crate::traits::traits_summary_example::{Tweet, Summary, NewsArticle};

pub fn run(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    notify(&article);

}

fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize());
}

fn notify_bounds<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}

//We can also specify more than one trait bound. Say we wanted notify to use display formatting
// as well as summarize on item: we specify in the notify definition that item must implement
// both Display and Summary. We can do so using the + syntax:

// pub fn notify(item: &(impl Summary + Display)) {

// The + syntax is also valid with trait bounds on generic types:

// pub fn notify<T: Summary + Display>(item: &T) {



// Using too many trait bounds has its downsides. Each generic has its own trait bounds, so
// functions with multiple generic type parameters can contain lots of trait bound information
// between the function’s name and its parameter list, making the function signature hard to read.
// For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause
// after the function signature. So instead of writing this:
//
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     we can use a where clause, like this:
//
//
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
//     {
// This function’s signature is less cluttered: the function name, parameter list, and return
// type are close together, similar to a function without lots of trait bounds.


// Returning Types that Implement Traits
// We can also use the impl Trait syntax in the return position to return a value of some type
// that implements a trait, as shown here:


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// However, you can only use impl Trait if you’re returning a single type. For example, this code
// that returns either a NewsArticle or a Tweet with the return type specified as impl Summary
// wouldn’t work:

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }