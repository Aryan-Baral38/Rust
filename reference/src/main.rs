fn main() {
    let refrence_to_nothing = dangle();
    println!("{refrence_to_nothing}");
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}
