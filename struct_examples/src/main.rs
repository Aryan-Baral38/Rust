struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let l = 3;
    let b = 5;

    println!(
        "NORMAL: The area of rectangle is with length {} and breadth {} is  {}",
        l,
        b,
        area(l, b)
    );

    //tuple refactor
    let rect1: (u32, u32) = (2, 10);

    println!(
        "TUPLE: The area of rectangle is with length {} and breadth {} is  {}",
        rect1.0,
        rect1.1,
        tuple_area(rect1)
    );

    //Struct refactor
    let rect1 = Rectangle {
        width: 9,
        height: 3,
    };

    println!("STRUCT: The area of rectangle is  {}", struct_area(rect1));
}

fn area(l: u32, b: u32) -> u32 {
    l * b
}

fn tuple_area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn struct_area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
