#[derive(Debug)]
struct Rectangle{
    length:u32,
    width:u32,
}
fn main() {
   let rect1=Rectangle{length:50,width:50};
   println!("{:#?}",rect1);
    println!("The area of rectangle is {} square pixels.",area(&rect1));
}
fn area(rectangle:&Rectangle)->u32{
    rectangle.length*rectangle.width
}
