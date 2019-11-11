fn main() {
    let mut s = String::from("Hello");
    let s1 = s.clone();
    s.push_str(", world");
    println!("{}", s);
    println!("{},{}", s1, s);
    takes_ownerchip(s1);
    //println!("{}",s1);
    let x = 5;
    makes_copy(x);
    println!("{}", x);
    let str1 = give_owenrchip();
    println!("{}", str1);
    let str2 = String::from("hi");
    let str3 = takes_and_gives_back(str2);
    //println!("{}",str2);
    println!("{}", str3);
    let (str4, len) = calculate_length(str3);
    println!("{} length: {}", str4, len);
}

fn takes_ownerchip(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn give_owenrchip() -> String {
    let some_string = String::from("China");
    println!("{}", some_string);
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
