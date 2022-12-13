use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    let input = fs::read_to_string(config.input_path).expect("Expected to read the file");

    day11::run(&input, config.part);
}

struct Config {
    input_path: String,
    part: i32,
}

impl Config {
    fn new(input: &String, part: i32) -> Config {
        let input_path = input.clone();
        let part: i32 = part;

        Config { input_path, part }
    }
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 3 || args.len() > 3 {
        panic!("Please provide 2 arguments");
    }
    let part = match args[2].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("2nd argument not a number"),
    };
    if part < 1 || part > 2 {
        panic!("The second argument should either 1 or 2");
    }

    Config::new(&args[1], part)
}

