enum IpAddr {
    V4,
    V6,
}

enum Message {
    Quit,
    Move{ x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
impl Coin {
fn value_in_cents(&self) -> u8 {
    match self {
        Coin::Quarter(state) => {println!("Quarter from state {state:?}");
        25}
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,

    }
}}

fn option_example(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n),
        None => None,
    }
}

fn main() {
    let mobile = IpAddr::V4;
    let coins = Coin::Quarter((UsState::Alabama));
    coins.value_in_cents();
    option_example(Some(34));
    let max: Option<i32> = None;

    if let Some(n) = max {
        println!("{n}")
    } else {
        println!("No value")
    }
}