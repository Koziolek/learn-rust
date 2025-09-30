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

    let home_struct = IpAddrStruct::V4(IpV4Addr{addr: String::from("127.0.0.1")});
    let loopback_struct = IpAddrStruct::V6(IpV6Addr{addr: String::from("127.0.0.1")});
    home_struct.show();
    loopback_struct.show();
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
    let none = None;
    let none1= add_one(none);
    // println!("{}", none1.unwrap()); // unwrap na none1 powoduje PANIC
    println!("{:?}", none1); // przez debug

// ---- any match

    let roll = 9;
    move_hero(roll);

// ---- if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("maximum is {max}"),
        _ => println!("nope")
    }
    // równoznaczne z 
    if let Some(max) = config_max {
        println!("maximum is {max}");
    }  

    let coin = Coin::Quarter(PlState::Mazowieckie);
    let coin1 = Coin::Quarter(PlState::Dolnoslaskie);
    else_let_match(&coin); // tu trochę magii z pożyczaniem, bo nie chce mi się wymyślać nazw
                           // zmiennych
    else_let_match(&Coin::Dime);

// ---- else let 
    let desc0 = describe_state_quarter0(&coin);
    println!("{desc0:?}");
    let desc01 = describe_state_quarter0(&coin1);
    println!("{desc01:?}");
    // refaktoryzacja 
    let desc1 = describe_state_quarter1(&coin);
    println!("{desc1:?}");
    let desc11 = describe_state_quarter1(&coin1);
    println!("{desc11:?}");
    // refaktoryzacja 
    let desc2 = describe_state_quarter2(&coin);
    println!("{desc2:?}");
    let desc21 = describe_state_quarter2(&coin1);
    println!("{desc21:?}");



}

// ---- else let
fn describe_state_quarter0(coin: &Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1111) {
             Some(format!("{state:?} is quite old place"))
        } else {
             Some(format!("{state:?} is old but not so old"))
        }
    } else {
        None
    }
}
// refaktoryzacja - rozdzielenie logiki wyboru rodzaju monety i wypisywania
fn describe_state_quarter1(coin: &Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin{
        state
    } else {
        return None; // paskuda
    };

    if state.existed_in(1111) {
        Some(format!("{state:?} is quite old place"))
    } else {
        Some(format!("{state:?} is old but not so old"))
    }
}
// refaktoryzacja - uproszczenie pierwszego ifa do let else

fn describe_state_quarter2(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    }; 

    if state.existed_in(1111) {
        Some(format!("{state:?} is quite old place"))
    } else {
        Some(format!("{state:?} is old but not so old"))
    }
}

// pomocnicze
impl PlState {
    fn existed_in(&self, year: u16) -> bool {
        match self{
            PlState::Dolnoslaskie => year >= 1100,
            PlState::Mazowieckie => year >= 1300,
        }
    }

}
// ---- if let else pomocniczo
fn else_let_match(coin: &Coin) -> &Coin{
    let mut count = 0;
   
   //match coin{
   //    Coin::Quarter(s) => println!("Have quater from {s:?}"),
   //    _ => count += 1,
   // }


    // równoważne
    if let Coin::Quarter(state) = coin {
        println!("Have quater from {state:?}");
    } else {
        count += 1; 
    }

    println!("{count}");
    coin
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
           Message::Move{x: a, y} => println!("Moving to x:{}, y:{}", a, y), // wnioskowanie nazw
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

impl IpAddrStruct {
    fn show(&self) {
        match self{
            IpAddrStruct::V4(addr) => println!("{}", addr.addr),
            IpAddrStruct::V6(addr) => println!("{}", addr.addr),
        }
    }
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

impl std::string::ToString for IpAddr3 {
    fn to_string(&self) -> String {
        match self {
            IpAddr3::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddr3::V6(val) => format!("{})", val),
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
        // write!(f, "{}", self) // -- wywolanie rekurencyjne bez stopu
        match self {
            IpAddrKind::V4 => f.write_str("V4"),
            IpAddrKind::V6 => f.write_str("V6"),
        }
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


