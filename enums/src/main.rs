#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarte(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaskam,
}
fn value_in_centc(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarte(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let c = Coin::Quarte(UsState::Alabama);
    println!("{}", value_in_centc(c));
    let two = Some(2);
    let three = plus_one(two);
    let none = plus_one(None);
    println!("{:?}", two);
    println!("{:?}", three);
    println!("{:?}", none);
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
    };
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let some_u8_value = Some(0u8);
}
