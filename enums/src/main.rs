fn main() {

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // nie przejdzie przez brak impl formatera
    // println!("ip v4 {0}", home);
    
    print_ip_addr(home);
    print_ip_addr(loopback);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
// ---- pola w enumie
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));
    println!("{}", home2.to_string());
    println!("{}", loopback2.to_string());
    let _home3 = IpAddr3::V4(127, 0 , 0, 1);
    let _loopback3 = IpAddr3::V6(String::from("::1"));

    let _home_struct = IpAddrStruct::V4(IpV4Addr{addr: String::from("127.0.0.1")});
    let _loopback_struct = IpAddrStruct::V6(IpV6Addr{addr: String::from("127.0.0.1")});

// ---- przykladowa implemantacja komunikatów
    let message_write = Message::Write(String::from("Hello"));
    message_write.call();
    Message::Quit.call();
    Message::Move{x: 1, y: 1}.call();
    Message::Color(1, 1, 1).call();

// ---- match

    println!("{}", coin_to_cent(Coin::Penny));
    println!("{}", coin_to_cent(Coin::Nickel));
    println!("{}", coin_to_cent(Coin::Dime));
    println!("{}", coin_to_cent(Coin::Quarter(PlState::Dolnoslaskie)));
    println!("{}", coin_to_cent(Coin::Quarter(PlState::Mazowieckie)));
// ---- Option match
    let five = Some(5);
    let six = add_one(five);
    println!("{}", six.unwrap());

// ---- any match

    let roll = 9;
    move_hero(roll);

}

// ---- any match

fn move_hero(roll: i32) {
    match roll {
        3 => println!("Hat"),
        7 => println!("Sword"),
        _ => println!("Move"),
    }

}

// ---- Option match

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }    
}

// ---- match

#[derive(Debug)]
enum PlState {
    Dolnoslaskie,
    Mazowieckie,
}


enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter(PlState)
}

fn coin_to_cent(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quater from {state:?}");
            25
        },
    } 
}

// ---- przykladowa implemantacja komunikatów

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Color(u8, u8, u8)
}

impl Message {
    fn call(&self) {
        match self{
           Message::Quit => (),
           Message::Move{x: _, y: _} => (),
           Message::Write(_v) => (),
           Message::Color(_r, _g, _b) => ()
        }
    }
}

// ---- pola w enumie

enum IpAddrStruct{
    V4(IpV4Addr),
    V6(IpV6Addr),
}

struct IpV4Addr{
    addr: String
}

struct IpV6Addr{
    addr: String
}


enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3{
    V4(u8, u8, u8, u8),
    V6(String)
}
// -- offtopic użycie traitu ToString

impl std::string::ToString for IpAddr2 {
    fn to_string(&self) -> String {
        match self {
            IpAddr2::V4(val) => val.to_string(),
            IpAddr2::V6(val) => val.to_string(),
        }
    }
}


// ----- podstawy
fn print_ip_addr(ip_addr: IpAddr){
    println!("ip {0} {1}", ip_addr.kind, ip_addr.address);
}

enum IpAddrKind{
    V4, 
    V6,
}
// offtopic – implementacja Display, ale bez importów, więc dużo pełnych nazw. 
impl std::fmt::Display for IpAddrKind{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
// end
fn route(ip_kind: IpAddrKind ){
    println!("{0}", ip_kind);
}

struct IpAddr{
    kind: IpAddrKind,
    address: String
}


