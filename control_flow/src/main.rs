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
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    print_number("The result is", result);
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("DONE WHILE");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        print_number("The value is", a[index]);
        index += 1;
    }
    for element in a.iter() {
        println!("value is: {}", element)
    }
    for element in (1..4).rev() {
        println!("{}!", element);
    }
    let f = fib(6);
    print_number("Fib 5:", f)
}
fn print_number(s: &str, x: i32) {
    println!("{} {}", s, x);
}
fn fib(x: i32) -> i32 {
    if x <= 1 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}
