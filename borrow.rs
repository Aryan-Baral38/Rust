
fn main(){
    let mut greeting = String::from("Hello");
    let len = calc_len(&greeting);
    println!("Length of greeting {greeting}: {len}");
    append_word(&mut greeting);
    println!("Length of greeting {greeting}: {}", calc_len(&greeting));
}

fn calc_len(s: &String) -> usize{
    s.len()
}

fn append_word(s: &mut String){
    s.push_str(", world!");
}
