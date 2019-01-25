fn main() {
    println!("Hello, world!");

    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    }

    let loopback = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("::1"),
    }
}

enum IPAddrKind {
    V4,
    V6,
}

enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_type: IPAddrKind) { }
