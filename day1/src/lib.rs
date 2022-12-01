pub fn run(input: &String) -> (u64, u64, u64) {
    // Splits each elf into a chunk
    let split_input = input.split("\n\n");

    let mut entries: Vec<u64> = Vec::new();
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
        entries.insert(0, current_total);
    }
    entries.sort_unstable_by(|a, b| b.cmp(a));
    (entries[0], entries[1], entries[2])
}