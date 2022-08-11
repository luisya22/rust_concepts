enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}


pub fn run(){
    let coin = Coin::Dime;

    let coin_cents_value = value_in_cents(coin);

    println!("Coin value: {coin_cents_value}");

    //--------------------
    let dice_roll = 9;
    match dice_roll{
        3 => println!("You have a new Fancy Hat"),
        7 => println!("Your Fancy Hat has been removed"),
        other => println!("Move {other} spaces")
    }


    // If let control flow

    let config_max = Some(3u8);
    if let Some(max) = config_max{
        println!("The maximum is configured to be {max}")
    }
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}