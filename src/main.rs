use std::io;
use std::str::FromStr;

fn main() {
    
    let max_number = 256;
    println!("Guess a number between 0 and {}", max_number);
    let input: i32 = loop {
        match get_input() {
            Some(x) => {break x;}
            None => {}
        }
    };
    println!("Hello, {}", &input);
}


fn get_input<T: FromStr>() -> Option<T> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_) => {return None},
        _ => {}
    }
    input.trim().parse::<T>().ok()
}
