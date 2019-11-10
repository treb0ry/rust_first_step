fn main() {
    let number = 8;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number in not divisible by 4,3, or2");
    }
    let condition = true;
    let num = if condition { 13 } else { 10 };
    print_number("the value of num", num);
}
fn print_number(s: &str, x: i32) {
    println!("{} {}", s, x);
}
