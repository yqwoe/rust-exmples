
#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Message{
    Quit,
    Move{x: i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Message{
    fn call(&self){
        println!("self is {:?}",&self)
    }
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

    dbg!(&four);
    dbg!(&six);

    let m = Message::Move { x: 1, y: 2 };
    m.call();

    let v = value_in_cents(Coin::Quarter);
    println!("value_in_cents(coin) = {}", v);

    let dice_roll = 9;

    match dice_roll {
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        6 => println!("six"),
        other => println!("something else {}",other),
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
