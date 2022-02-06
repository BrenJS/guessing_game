use std::io;

fn main() {

    let input = loop {
        match get_input() {
            Some(x) => {break x;}
            None => {}
        }
    };
    println!("Hello, {}", &input);
}


fn get_input() -> Option<u16> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_) => {return None},
        _ => {}
    }
    input.trim().parse::<u16>().ok()
}
