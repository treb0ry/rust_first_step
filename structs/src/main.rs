struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
    let _black = Color(0, 0, 0);
    let mut user1 = User {
        email: String::from("fjdk@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user(
        String::from("anotheruser"),
        String::from("user2@example.com"),
    );
    let user3 = User {
        email: String::from("djaskl@gmail.com"),
        username: String::from("nameuser3"),
        ..user2
    };
    user1.email = String::from("anotheemail@example.com");
    println!(
        "[{};{};{};{}]",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );
    println!(
        "[{};{};{};{}]",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );
    println!(
        "[{};{};{};{}]",
        user3.email, user3.username, user3.active, user3.sign_in_count
    )
}
