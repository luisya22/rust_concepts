
#[derive(Debug)]
enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message{
    fn call(&self){
        println!("Calling: {:#?}", self);
    }
}

pub fn run(){
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind){
    println!("{:?}", ip_kind);

    match ip_kind {
        IpAddrKind::V4(value1, value2, value3, value4) => println!("value: {value1}.{value2}.{value3}.{value4}"),
        IpAddrKind::V6(value) => println!("value: {value}")
    }
}