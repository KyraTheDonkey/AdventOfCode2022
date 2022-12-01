use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    let input = fs::read_to_string(input_path).expect("Expected to read the file");

    let answer = day1::run(&input);
    let sum = answer.0 + answer.1 + answer.2;

    println!("Your highest are {}, {} and {}! The sum is {}", answer.0, answer.1, answer.2, sum);
}
