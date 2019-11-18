fn main() {
    let my_string = String::from("hello world");
    let f_word = first_word(&my_string[..]);
    println!("{}", f_word);
    let s_word = second_word(&my_string[..]);
    println!("{}", s_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}
