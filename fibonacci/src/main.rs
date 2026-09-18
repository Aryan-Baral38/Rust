use std::io;

fn fib(a: usize) -> usize{
    if a == 0 {return 0;}
    else if a == 1 {return 1;}
    return fib(a - 1) + fib(a - 2);
}
fn main(){
    println!("nth Fibbonachi number");
    println!("Enter n:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Error reading");

    let n = n
        .trim()
        .parse()
        .expect("Enter a number");

    let term = fib(n);
    println!("{n} Fibbonachi term is {term}");
}
