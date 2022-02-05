use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = input.trim();
            input = trimmed_input.to_string();
        }
        Err(e) => {println!("{:?}", e);}
    }
    println!("Hello, {}!", {input});
}