use std::io;
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn new(w: u32, h: u32) -> Self{
        Rectangle {width: w, height: h}
    }
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn scale(&mut self, s: u32){
        self.width *= s;
        self.height += s;
    }
    fn destroy(self) -> String{
        format!("Destoryed rect of area {}", self.area())
    }
}

fn main(){
    let mut rect = Rectangle::new(10,5);
    println!("Initial area: {}", rect.area());
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("cannot read");
    let s : u32 = match inp.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Error");
            return ;
        }
    };
    rect.scale(s);
    println!("scaled area: {}", rect.area());

    let msg = rect.destroy();
    println!("{msg}");
}
