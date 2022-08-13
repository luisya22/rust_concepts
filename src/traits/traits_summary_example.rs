pub trait Summary {
    fn summarize(&self) -> String;
}

// Sometimes it’s useful to have default behavior for some or all of the methods in a trait
// instead of requiring implementations for all methods on every type. Then, as we implement
// the trait on a particular type, we can keep or override each method’s default behavior.
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}