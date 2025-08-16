#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//Option<T> 사용
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

pub fn matchs() {
    let penny = value_in_cents(Coin::Penny);
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let quarter = value_in_cents(Coin::Quarter);

    println!("penny : {}, nickel : {}, dime : {}, quarter : {}", penny, nickel, dime, quarter);

    let five = plus_one(Some(5));
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five : {:?}, six : {:?}, none : {:?}", five, six, none);
}