pub fn run(input: &String, _part: i32) -> i32 {
    let split_input: Vec<&str> = input.split("\n").collect();
    
    let mut total = 0;
    for entry in split_input {
        let ( elf_1, elf_2 ) = read_elves(entry);
        if (elf_1.lower_bound <= elf_2.lower_bound && elf_1.upper_bound >= elf_2.upper_bound) || 
            (elf_1.lower_bound >= elf_2.lower_bound && elf_1.upper_bound <= elf_2.upper_bound) {
                total += 1;
            }
    }

    return total;
}

struct Elf {
    lower_bound: i32,
    upper_bound: i32
}

impl Elf {
    fn new(input: &str) -> Elf {
        let s: Vec<&str> = input.split("-").collect();
        let lower_bound = match s[0].trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("The number read for the lower bound was not valid (tried {})", s[0]),
        };
        let upper_bound = match s[1].trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("The number read for the upper bound was not valid (tried {})", s[1]),
        };
        Elf { lower_bound, upper_bound }
    }
}

fn read_elves(input: &str) -> (Elf, Elf) {
    let split_input: Vec<&str> = input.split(",").collect();

    (Elf::new(split_input[0]), Elf::new(split_input[1]))
}