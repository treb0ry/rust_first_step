fn main() {
    println!("Hello, world!");
    another_function(5, 8);
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    another_function(x, y);
    let x = five();
    println!("The value of x is: {}", x)
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
fn five() -> i8 {
    5
}
