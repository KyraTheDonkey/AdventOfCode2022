use std::cmp::Ordering;

pub fn run(input: &String) -> u64 {
    // Splits each elf into a chunk
    let split_input = input.split("\n\n");

    let mut highest: u64 = u64::MIN;
    for elf in split_input {
        let mut current_total: u64 = 0;

        let foods = elf.split("\n");
        for food in foods {
            // Parse entry into an integer
            let calories: u64 = match food.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            current_total += calories;
        }
        match current_total.cmp(&highest) {
            Ordering::Equal => println!("{} equaled {}", current_total, highest),
            Ordering::Greater => {
                highest = current_total;
                println!("{} was greater than {}", current_total, highest)
            },
            Ordering::Less => println!("{} was less than {}", current_total, highest)
        }
    }

    return highest;
}