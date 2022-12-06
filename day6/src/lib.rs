pub fn run(input: &String, _part: i32) -> i32 {
    let mut i: usize = 0;
    while i < input.len() - 4 {
        let s = &input[i..=i+3];
        if !check_duplicate(s) {
            break;
        }
        i += 1;
    }
    return (i+4).try_into().unwrap();
}

fn check_duplicate(input: &str) -> bool {
    println!("Checking {}", input);
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len()-1 {
        for j in i+1..chars.len() {
            if chars[i] == chars[j] {
                return true;
            }
        }
    }


    return false;
}