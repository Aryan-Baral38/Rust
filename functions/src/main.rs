
fn f(a : u32) -> u32{
    a * a
}

fn main(){
    let mut a = 3;
    let b = f(a);
    print!("{b}");
    a = 5;
    println!("{a}");
    let m = if a < 5 {a} else {5}; 
    println!("{m}");
}
