// When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the
// types that might be used with the code that’s using trait objects, so it doesn’t know which
// method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside
// the trait object to know which method to call. This lookup incurs a runtime cost that doesn’t
// occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline
// a method’s code, which in turn prevents some optimizations. However, we did get extra
// flexibility in the code that we wrote in Listing 17-5 and were able to support in Listing 17-9,
// so it’s a trade-off to consider.

pub fn run(){
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    pub components: Vec<Box<dyn Draw>> // This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button{
    fn draw(&self) {
        // todo!()
    }
}

// If someone using our library decides to implement a SelectBox struct that has width, height,
// and options fields, they implement the Draw trait on the SelectBox type as well
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}