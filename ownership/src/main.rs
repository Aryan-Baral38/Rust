fn main() {
    let s = String::from("Hello, world");
    println!("{s} is the string");
    let first = first_word(&s);
    println!("{first}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
