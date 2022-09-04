enum IpAddrKind {
    V4(),
    V6(),
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another example of enum
// Drfining an enum with variants such as the example below is
// similar to defining defferent kinds of struct definition without
// one with this, you dont need the struct keyword
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We are also able to define methods on enums
impl Message {
    fn call(&self) {
        // Method body would be defined here
        // println!("The message that was given is"); //{}", self.Write());
    }
}

// if you want to try to address this problem with struct
// struct IpAddr {
// kind: IpAddrKind,
// address: String,
// }

// Enum Option, can be used in place of NULL
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    // A different way to store the enum value
    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));

    // Addressing the problem with struct
    // let home = IpAddr {
    // kind: IpAddrKind::V4,
    // address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    // kind: IpAddrKind::V6,
    // address: String::from("::1"),
    // };

    // Initializing and calling the message enum
    let m = Message::Write(String::from("hello"));
    m.call();

    // using the option enum
//     let some_number = Some(5);
//     let some_char = Some('e');

//     let absent_number: Option<i32> = None; // Check it back ...
} 

fn route(ip_kind: IpAddrKind) {}
