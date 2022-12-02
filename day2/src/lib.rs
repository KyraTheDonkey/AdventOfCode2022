pub fn run(input: &String) -> i64 {
    let mut score: i64 = 0;

    let games = input.split("\n");
    for game in games {
        let mut parse = game.chars();
        let opp_move = parse.nth(0).expect("Should get an opponent's move");
        let my_move = parse.nth(1).expect("Should get my move");
        score += calc_score(&opp_move, &my_move);
    }

    return score;
}

fn calc_score(opp_move: &char, my_move: &char) -> i64 {
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
