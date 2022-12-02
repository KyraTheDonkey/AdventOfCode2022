use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    let input = fs::read_to_string(input_path).expect("Expected to read the file");

    let output = day2::run(&input);
    println!("Answer is {}", output);
}
