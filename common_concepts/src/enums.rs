use std::net::Ipv4Addr;

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V4Std(Ipv4Addr),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor {r: i32, g: i32, b: i32}
}

impl Message {
    fn call(&self) {

    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn test() {
    let ipv4 = IpAddrKind::V4(127, 0, 0, 1);
    let some_numb = Some(11);
    let no_number: Option<i32> = None;

    let matched: i32 = match no_number {
        Some(x) => x + 1,
        None => 2
    };

    let some_value = Some(2);
    if let Some(3) = some_value {
        println!("It's 3!");
    };

}

fn route(addr_kind: IpAddrKind) {

}