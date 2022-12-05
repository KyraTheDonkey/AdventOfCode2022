pub fn run(input: &String, _part: i32) -> String {
    let mut crate_stack: Vec<Vec<char>> = create_crates();
    print_crate_stack(&crate_stack);
    println!();
    let split_input: Vec<&str> = input.split("\n").collect();
    for i in 0..split_input.len() {
        if i < 10 {
            continue;
        }
        let action: Vec<&str> = split_input[i].split_ascii_whitespace().collect();
        let number = action[1]
            .trim()
            .parse::<i32>()
            .expect("Expected a number for how many crates to move");
        let from = action[3]
            .trim()
            .parse::<usize>()
            .expect("Expected a number for where to move the crate from");
        let to = action[5]
            .trim()
            .parse::<usize>()
            .expect("Expected a number for where to move the crate to");
        for _i in 1..=number {
            let curr_crate = match crate_stack[from-1].pop() {
                Some(char) => char,
                None => {
                    print_crate_stack(&crate_stack);
                    panic!("There wasn't a crate to pop from stack {} in iteration {}", from, i)
                }
            };
            crate_stack[to-1].push(curr_crate);
        }
    }
    print_crate_stack(&crate_stack);

    return String::from("");
}

pub fn create_crates() -> Vec<Vec<char>> {
    let mut crate_stack: Vec<Vec<char>> = Vec::new();
    crate_stack.push(vec!['R', 'S', 'L', 'F', 'Q']);
    crate_stack.push(vec!['N', 'Z', 'Q', 'G', 'P', 'T']);
    crate_stack.push(vec!['S', 'M', 'Q', 'B']);
    crate_stack.push(vec!['T', 'G', 'Z', 'J', 'H', 'C', 'B', 'Q']);
    crate_stack.push(vec!['P', 'H', 'M', 'B', 'N', 'F', 'S']);
    crate_stack.push(vec!['P', 'C', 'Q', 'N', 'S', 'L', 'V', 'G']);
    crate_stack.push(vec!['W', 'C', 'F']);
    crate_stack.push(vec!['Q', 'H', 'G', 'Z', 'W', 'V', 'P', 'M']);
    crate_stack.push(vec!['G', 'Z', 'D', 'L', 'C', 'N', 'R']);

    return crate_stack;
}

pub fn print_crate_stack(crate_stack: &Vec<Vec<char>>) {
    for i in 0..=8 {
        let mut s = String::from("");
        s.push_str(&(i + 1).to_string());
        s.push_str(": ");
        let crates = &crate_stack[i];
        for c in crates {
            s.push(*c);
        }
        println!("{}", s)
    }
}
