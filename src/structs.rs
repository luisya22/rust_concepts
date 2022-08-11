#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Tuple Struct without named fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like Struct
struct AlwaysEqual;

pub fn run(){
    let mut user1 = User{
        email: String::from("sommeone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:#?}", user2);

    //--------------------------------------------------------------------
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // --------------------------------------------------------------------
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}