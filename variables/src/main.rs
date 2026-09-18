fn main() {
    let x = 100_00;
    print!("{x}!");
    let x = x + 1;
    {
        let x = x * 2;
        print!("{x}!");
    }
    println!("{x}");
    let mut x = (21, 2.3, 'a');
    x.0 = 3;
    println!("({}, {}, {})", x.0, x.1,x.2);
    let mut a : [i32; 5] = [1, 2,3,4,5];
    a[1] = 9;
    println!("{}", a[1]);

}
