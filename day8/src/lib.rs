pub fn run(input: &String) -> (i32, i32) {
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
    let mut highest_scenic_score = 0;
    for row in 0..tree_array.len() {
        for column in 0..tree_array[row].len() {
            let scenic_score = get_scenic_score(&tree_array, row, column);
            if scenic_score > highest_scenic_score { highest_scenic_score = scenic_score }
        }
    }
    return (visible_trees, highest_scenic_score);
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

    // Check to the column above
    blocked = false;
    for r in 0..row {
        if trees[r][column] >= this_tree {
            blocked = true;
            break;
        }
    }
    if !blocked { visible = true }

    // Check to the column below
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

fn get_scenic_score(trees: &Vec<Vec<char>>, row: usize, column: usize) -> i32 {
    let this_tree = trees[row][column];

    // Check in the row to the left
    let mut left_view = 0;
    let mut c_order = (0..column).collect::<Vec<usize>>();
    c_order.reverse();
    for c in c_order {
        left_view = column - c;
        if trees[row][c] >= this_tree {
            break;
        }
    }
    
    // Check in the row to the right
    let mut right_view = 0;
    for c in column+1..trees[row].len() {
        right_view = c - column;
        if trees[row][c] >= this_tree { 
            break;
        }
    }

    // Check to the column above
    let mut up_view = 0;
    let mut r_order = (0..row).collect::<Vec<usize>>();
    r_order.reverse();
    for r in r_order {
        up_view = row - r;
        if trees[r][column] >= this_tree {
            break;
        }
    }

    // Check to the column below
    let mut down_view = 0;
    for r in row+1..trees.len() {
        down_view = r - row;
        if trees[r][column] >= this_tree {
            break;
        }
    }

    println!("({},{}): {} * {} * {} * {} = {}", row, column, left_view, right_view, up_view, down_view, left_view * right_view * up_view * down_view);
    (left_view * right_view * up_view * down_view).try_into().unwrap()
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