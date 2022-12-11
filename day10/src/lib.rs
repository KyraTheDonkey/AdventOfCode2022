use std::collections::VecDeque;

const TIME_FOR_NOOP: i32 = 1;
const TIME_FOR_ADDX: i32 = 2;
pub fn run(input: &String, _part: i32) {
    let mut cpu_info = read_cpu_info(input);
    let mut cpu_info_output: Vec<CpuInfo> = Vec::new();
    print_cpu_input(&cpu_info);
    
    let mut current_time = 0;
    let mut current_x = 1;
    let mut output_string = String::new();
    while cpu_info.len() > 0 {
        let info = match cpu_info.pop_front() {
            Some(info) => info,
            None => panic!("Ran out of enteries in cpu_info too quickly!"),
        };
        while current_time < info.time {
            
            // Part 1
            if current_time % 40 == 20 {
                cpu_info_output.push(CpuInfo::new(current_time, current_x));
            }

            // Part 2
            if (current_x - ((current_time-1)%40)).abs() <= 1 { output_string.push('#') } else { output_string.push('.') };
            if current_time % 40 == 0 {
                output_string.push('\n');
            }
            current_time += 1;
        }
        current_x = info.x;
    }
    print_cpu_info(&cpu_info_output);
    print!("{}", output_string);
}

fn read_cpu_info(input: &String) -> VecDeque<CpuInfo> {
    let split_input: Vec<&str> = input.split("\n").collect();

    let mut current_time = 1;
    let mut current_x = 1;
    let mut cpu_info: VecDeque<CpuInfo> = VecDeque::new();
    for i in 0..split_input.len() {
        match split_input[i].chars().nth(0).unwrap() {
            'n' => {
                // Noop
                current_time += TIME_FOR_NOOP;
            },
            'a' => {
                // Addx
                let number: i32 = match split_input[i][5..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("failed to parse number"),
                };
                current_time += TIME_FOR_ADDX;
                current_x += number;
            },
            _ => panic!("Did not receive expected input"),
        };
        cpu_info.push_back(CpuInfo::new(current_time, current_x));
    }
    cpu_info
}

fn print_cpu_info(input: &Vec<CpuInfo>) {
    println!("Time | X");
    let mut multiplied_x = 0;
    for cpu_info in input {
        println!("{} | {}", cpu_info.time, cpu_info.x);
        multiplied_x += cpu_info.time * cpu_info.x;
    };
    println!("Multiplied: {}", multiplied_x);
}

fn print_cpu_input(input: &VecDeque<CpuInfo>) {
    println!("Time | X");
    for cpu_info in input {
        println!("{} | {}", cpu_info.time, cpu_info.x);
    };
}

struct CpuInfo {
    time: i32,
    x: i32,
}

impl CpuInfo {
    fn new(time: i32, x: i32) -> CpuInfo {
        CpuInfo { time, x }
    }
}