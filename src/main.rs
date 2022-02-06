use rand::Rng;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let max_number = 256;
    println!("Guess a number between 0 and {}", max_number);
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=max_number);
    print!("Your guess: ");
    io::stdout().flush().unwrap();
    let input: i32 = loop {
        match get_input() {
            Some(x) => {
                if x < random_number {
                    println!("Too small, try again!");
                    continue;
                } else if x > random_number {
                    println!("Too large, try again!");
                    continue;
                } else {
                    break x;
                }
            }
            None => {
                println!(
                    "Not a valid input, try and number between 0 and {}!",
                    max_number
                );
                print!("Your guess: ");
                io::stdout().flush().unwrap();
            }
        }
    };
    println!("Congratulations, {} is right!", &input);
}

fn get_input<T: FromStr>() -> Option<T> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_) => return None,
        _ => {}
    }
    input.trim().parse::<T>().ok()
}
