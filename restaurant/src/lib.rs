mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){
            super::serving::take_order();
            println!("Adding to wait list");
        }

        fn seat_at_table(){
            println!("Seating at table");
        }
    }

    mod serving{
        pub fn take_order() {
            println!("Taking Order!");
        }

        fn server_order(){
            println!("Serving Order!");
        }

        fn take_payments(){
            println!("Taking Payment!");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use crate::front_of_house::hosting;
pub fn eat_at_restaurant_use(){
    hosting::add_to_waitlist();
}
