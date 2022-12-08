pub fn run(input: &String, _part: i32) -> i32 {
    let tree_array = read_input(input);
    print_trees(&tree_array);

    let mut visible_trees = 0;
    for row in 0..tree_array.len() {
        for column in 0..tree_array[row].len() {
            if check_tree_visible(&tree_array, row, column) {
                visible_trees += 1;
            }
        }
    }
    return visible_trees;
}

fn check_tree_visible(trees: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    let this_tree = trees[row][column];
    let mut visible = false;
    
    // Check in the row to the left
    let mut blocked = false;
    for c in 0..column {
        if trees[row][c] >= this_tree { 
            blocked = true;
            break;
        }
    }
    if !blocked { visible = true }

    // Check in the row to the right
    blocked = false;
    for c in column+1..trees[row].len() {
        if trees[row][c] >= this_tree { 
            blocked = true;
            break;
        }
    }
    if !blocked { visible = true }

    blocked = false;
    for r in 0..row {
        if trees[r][column] >= this_tree {
            blocked = true;
            break;
        }
    }
    if !blocked { visible = true }

    blocked = false;
    for r in row+1..trees.len() {
        if trees[r][column] >= this_tree {
            blocked = true;
            break;
        }
    }
    if !blocked { visible = true }

    return visible;
}

fn read_input(input: &String) -> Vec<Vec<char>> {
    let mut input_array: Vec<Vec<char>> = Vec::new();
    let rows: Vec<&str> = input.split("\n").collect();
    for row in rows {
        let mut current_row: Vec<char> = Vec::new();
        for column in row.chars() {
            current_row.push(column);
        }
        input_array.push(current_row);
    }
    input_array
}

pub fn print_trees(trees: &Vec<Vec<char>>) {
    for row in 0..trees.len() {
        let mut s = String::from("");
        for column in 0..trees[row].len() {
            s.push(trees[row][column])
        }
        println!("{}", s)
    }
}