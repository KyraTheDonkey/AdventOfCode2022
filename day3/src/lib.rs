use substring::Substring;

pub fn run(input: &String, part: i32) -> u32 {
    match part {
        1 => run_p1(input),
        2 => run_p2(input),
        _ => 0,
    }
}

fn run_p1(input: &String) -> u32 {
    let mut sum_priorty: u32 = 0;
    let backpacks = input.split("\n");
    for backpack in backpacks {
        let len = backpack.len();
        let first_half = backpack.substring(0, len/2);
        let second_half = backpack.substring(len/2, len);
        print!("{} {} ", first_half, second_half);

        let mut duplicate_item: char = '_';
        for char in first_half.chars() {
            if second_half.contains(char) {
                duplicate_item = char;
                break;
            }
        }

        let priority = match duplicate_item {
            'a'..='z' => duplicate_item as u32 - 96,
            'A'..='Z' => duplicate_item as u32 - 64 + 26,
            _ => panic!("There wasn't a duplicate item in the backpack"),
        };
        println!("with duplicate \"{}\" and priority {}", duplicate_item, priority);
        
        sum_priorty += priority;
    }

    return sum_priorty;
}

fn run_p2 (input: &String) -> u32 {
    let mut sum_priorty: u32 = 0;
    let backpacks: Vec<&str> = input.split("\n").collect();
    let backpacks_len = backpacks.len();
    let mut i = 0;
    while i + 2 < backpacks_len {
        let elf_1 = backpacks[i];
        let elf_2 = backpacks[i+1];
        let elf_3 = backpacks[i+2];
        print!("{} {} {}", elf_1, elf_2, elf_3);

        let mut duplicate_item: char = '_';
        for char in elf_1.chars() {
            if elf_2.contains(char) && elf_3.contains(char) {
                duplicate_item = char;
                break;
            }
        }

        let priority = match duplicate_item {
            'a'..='z' => duplicate_item as u32 - 96,
            'A'..='Z' => duplicate_item as u32 - 64 + 26,
            _ => panic!("There wasn't a duplicate item in the backpack"),
        };
        println!("with duplicate \"{}\" and priority {}", duplicate_item, priority);
        
        sum_priorty += priority;
        i += 3;
    }
    return sum_priorty;
}