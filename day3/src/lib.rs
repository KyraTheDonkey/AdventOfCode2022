use substring::Substring;

pub fn run(input: &String, _part: i32) -> u32 {
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