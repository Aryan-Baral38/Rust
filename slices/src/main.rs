fn main() {
    let s = String::from("Hello world");
    let i :usize = first_word(&s);

}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
