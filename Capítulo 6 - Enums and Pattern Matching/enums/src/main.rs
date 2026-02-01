// Defining an Enum
enum IpAddrKind {
    V4,
    V6,
}
/////

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    //Enum Values
    let four = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    route(IpAddrKind::V4);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //Melhor forma de usar enum, nesse caso:
    enum IpAddrs {
        V4(String),
        V6(String),
    }
    let home = IpAddrs::V4(String::from("127.0.0.1"));
    let loopback = IpAddrs::V4(String::from("::1"));

    // Another Advantage:
    enum IpAddrss {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrss::V4(127, 0, 0, 1);
    let loopback = IpAddrss::V6(String::from("::1"));
}
fn route(ip_kind: IpAddrKind) {}

fn option_enum() {
    //The Option Enum and Its Advantages Over Null Values:
    let some_number = Some(5);
    let some_char = Some("e");
    let absent_number: Option<i32> = None;
}

fn match_control_flow() {
    //The match Control Flow Construct
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(state),
    }
    #[derive(Debug)]
    struct state {};
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

//Matches Are Exhaustive
//Catch-All Patterns and the _ Placeholder

fn exhaustive()->(){
    let dice_roll = 9;
    match dice_roll {
        3=> add_fancy_hat(),
        7=>remove_fancy_hat(),
        other=>move_player(other)
        
    }


    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn move_player(num_spaces :u8){}

}