use core::fmt;

pub fn run(input: &String, _part: i32) -> i32 {
    let mut head: Location = Location::new(0,0);
    let mut tail: Location = Location::new(0,0);
    let mut visited_locations: Vec<Location> = Vec::new();
    add_visited_location(&mut visited_locations, &tail);


    let split_input = input.split("\n");
    for line in split_input {
        let line = String::from(line);
        let direction = &line.chars().nth(0).unwrap();
        let times: i32 = match (&line[2..line.len()]).parse() {
            Ok(num) => num,
            Err(_) => panic!("Whoops"),
        };
        println!("Moving in direction {} {} times", direction, times);
        for _i in 0..times {
            move_head(&mut head, &direction);
            move_tail(&head, &mut tail);
            add_visited_location(&mut visited_locations, &tail);
            println!("Head: {} Tail {}", head, tail);
        }
        
    }

    return visited_locations.len().try_into().unwrap();
}

fn move_head(head: &mut Location, direction: &char) {
    match direction {
        'U' => head.column += 1,
        'D' => head.column -= 1,
        'L' => head.row -= 1,
        'R' => head.row += 1,
        _ => panic!("Received invalid direction ({}) to move head in!", direction),
    }
}

/*
XXUXX
X---X
L-T-R
X---X
XXDXX
*/
fn move_tail(head: &Location, tail: &mut Location) {
    if (head.row - tail.row).abs() <= 1 && (head.column - tail.column).abs() <= 1 {
        return;
    }
    tail.row += (head.row - tail.row).min(1).max(-1);
    tail.column += (head.column - tail.column).min(1).max(-1);
}

fn add_visited_location(visited_locations: &mut Vec<Location>, tail: &Location) {
    if !visited_locations.contains(tail) {
        visited_locations.push(tail.clone());
    }
}

#[derive(Clone,PartialEq)]
struct Location {
    row: i32,
    column: i32,
}

impl Location {
    fn new(row:i32, column:i32) -> Location {
        Location { row, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}