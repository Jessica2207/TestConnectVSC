use std::io;

fn main() {
    let mut input_string: String = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    println!("{}", input_string.trim())
}