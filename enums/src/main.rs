enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {
        x:i32,
        y:i32
    },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    //associated function
    fn some_function() {
        println!("Let's Get Rusty!");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let _localhost = IpAddrKind::V4(127, 0, 0, 1);

    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn route(_ip_kind:IpAddrKind) {}

//return the value of the coin in cents
fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}