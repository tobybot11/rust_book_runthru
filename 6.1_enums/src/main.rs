#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddrNG {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = IpAddrNG::V4(String::from("127.0.0.1"));
    let loopback = IpAddrNG::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = IpAddr2::V4(127,0,0,1);
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let some_number = Some(5);
    println!("some_number: {:?}", some_number);

    let some_string = Some("a string");
    println!("some_number: {:?}", some_string);

    let absent_number: Option<i32> = None;
    println!("absent_number: {:?}", absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.expect("the world is ending");
    println!("sum of x {} + y {} = sum {}", x, y.expect("the world is ending"), sum);

    let y: Option<i8> = None;

    let sum = x + y.unwrap_or(0);
    println!("sum of x {} + y {} = sum {}", x, y.unwrap_or(0), sum);

    let sum = x + y.expect("the world is ending");
    // will never get to this code..
    println!("sum of x {} + y {} = sum {}", x, y.expect("the world is ending"), sum);
}

fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type);
}
