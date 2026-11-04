#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 9,
        height: 3,
    };
    println!("struct {rect1:#?}");
}
