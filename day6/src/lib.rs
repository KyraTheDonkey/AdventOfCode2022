pub fn run(input: &String, part: i32) -> i32 {
    let mut i: usize = 0;
    let length;
    if part == 1 {
        length = 4
    } else {
        length = 14
    }
    while i < input.len() - length {
        let s = &input[i..i+length];
        if !check_duplicate(s) {
            break;
        }
        i += 1;
    }
    return (i+length).try_into().unwrap();
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