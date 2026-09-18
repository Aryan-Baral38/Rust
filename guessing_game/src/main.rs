use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("__GUESSING GAME__");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

    
}
