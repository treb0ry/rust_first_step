#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
struct IpAddr {
    kind:IpAddrKind,
}
#[derive(Debug)]
enum Message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}


fn main() {
    let four=IpAddrKind::V4;
    let six=IpAddrKind::V6;
    let home=IpAddr{
        kind:IpAddrKind::V4(String::from("127.0.0.1"))
        
    };
    let loopback=IpAddr{
        kind:IpAddrKind::V6(String::from("::1"))
    };
    let some_number=Some(5);
    let some_string=Some("a string");
    let absent_number:Option<i32> = None;


}

