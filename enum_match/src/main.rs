use std::default;

#[derive(Debug)]
enum UsState{
    Alabama,
    Alasks,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        // Coin::Penny => 1,
        // Coin::Nickel => 5,
        // Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
        other => 10,
    }
}


// Enum practice
#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6,
}

// To use IPAddrKind
fn route(ipkind: IPAddrKind){
    println!("IP is kind of:{:?}", ipkind);
}

//Associate values with Enums
#[derive(Debug)]
struct IPAddress{
    kind: IPAddrKind,
    addr: String,
}
// To use IPAddress
fn getAddr(){
    let home = IPAddress{
        kind: IPAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    let loopback = IPAddress{
        kind: IPAddrKind::V6,
        addr: String::from("::1"),
    };
    println!("Home ip addres is {:?}, loopback ip addres is {:?}", home, loopback);

    println!();
}

// Enum directly with values
#[derive(Debug)]
enum newIpKind {
    V4(String),
    V6(String),
}
// To use this kind
fn use_new_ip() {
    let home = newIpKind::V4(String::from("127.0.0.1"));
    let loopback = newIpKind::V6(String::from("::1"));
    println!("Home ip is:{:?}, loopback ip is: {:?}", home, loopback);
}

// Each Enum value with different type values
#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn call(&self){
        println!("method called!");
    }
}
// To use IpAddr
fn use_diff_value_enums(){
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopbakc = IpAddr::V6(String::from("::1"));
    home.call();
    println!();
    println!("Diff_values: Home is: {:?}, loopback is: {:?}", home, loopbakc);
}

// Option<T> practice
fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alasks);
    println!("coin has {:?} cents.", value_in_cents(coin));
    getAddr();
    use_new_ip();
    use_diff_value_enums();

    let num = plus_one(Some(5));
    let x = plus_one(None);
    println!("after plus one, some(5) is: {:?}, x is: {:?}", num,x);
    
}
