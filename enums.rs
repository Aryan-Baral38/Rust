#[derive(Debug)]
enum Command {
    Start,
    Stop,
    SetSpeed(u32),
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let cmd = Command::SetSpeed(80);

    match cmd {
        Command::Start => println!("System starting..."),
        Command::Stop => println!("System stopping..."),
        Command::SetSpeed(speed) => println!("Setting speed to {} mph", speed),
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
