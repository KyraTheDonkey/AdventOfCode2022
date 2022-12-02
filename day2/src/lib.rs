pub fn run(input: &String, part: i32) -> i64 {
    let mut score: i64 = 0;

    let games = input.split("\n");
    let calc_score: CalcScore;
    match part {
        1 => {
            calc_score = calc_score_p1;
        },
        2 => {
            calc_score = calc_score_p2;
        },
        _ => panic!("Didn't get a valid part number"),
    }
    for game in games {
        let mut parse = game.chars();
        let opp_move = parse.nth(0).expect("Should get an opponent's move");
        let my_move = parse.nth(1).expect("Should get my move");
        score += calc_score(&opp_move, &my_move);
    }
    return score;
    
}

fn calc_score_p1(opp_move: &char, my_move: &char) -> i64 {
    match opp_move {
        // Rock
        'A' => match my_move {
            'X' => 1 + 3,
            'Y' => 2 + 6,
            'Z' => 3 + 0,
            _ => -1,
        },
        // Paper
        'B' => match my_move {
            'X' => 1 + 0,
            'Y' => 2 + 3,
            'Z' => 3 + 6,
            _ => -1,
        },
        // Scissors
        'C' => match my_move {
            'X' => 1 + 6,
            'Y' => 2 + 0,
            'Z' => 3 + 3,
            _ => -1,
        },
        _ => {
            return -1;
        }
    }
}

fn calc_score_p2(opp_move: &char, my_result: &char) -> i64 {
    match opp_move {
        // Rock
        'A' => match my_result {
            'X' => 3 + 0, // Play Scissors
            'Y' => 1 + 3, // Play Rock
            'Z' => 2 + 6, // Play Paper
            _ => -1,
        },
        // Paper
        'B' => match my_result {
            'X' => 1 + 0, // Play Rock
            'Y' => 2 + 3, // Play Paper
            'Z' => 3 + 6, // Play Scissors
            _ => -1,
        },
        // Scissors
        'C' => match my_result {
            'X' => 2 + 0, // Play Paper
            'Y' => 3 + 3, // Play Scissors
            'Z' => 1 + 6, // Play Rock
            _ => -1,
        },
        _ => {
            return -1;
        }
    }
}

type CalcScore = fn(&char, &char) -> i64;