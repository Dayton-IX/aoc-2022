enum Action {
    ROCK,
    PAPER,
    SCISSORS,
}

struct Move {
    action: Action,
    points: u32,
}

struct Round {
    opponent_move: Move,
    player_move: Move,
    result: u32,
}

fn main() {
    println!("Hello, world!");
}

fn calculate_round_result(round: Round) -> u32 {
    let mut result: u32 = 0;

    return result;
}
