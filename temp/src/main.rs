use std::io;

fn farh_to_celsius(f: f32) -> f32{
    (f - 32.0)*5.0/9.0
}
fn celsius_to_farh(c: f32) -> f32{
    c * 9.0/5.0 + 32.0
}

fn main(){
    println!("Temperature conversion");
    println!("Enter a Temperature");
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Error reading the temprerature");
    let inp: f32 = inp
        .trim()
        .parse()
        .expect("Enter a number");
    let celsius = farh_to_celsius(inp);
    let far = celsius_to_farh(inp);
    println!("{inp} celsius is {far} farienhiet");
    println!("{inp} farahienit is {celsius} celsius");
}
