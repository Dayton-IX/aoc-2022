#[derive(Debug)]
enum Action {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Action {
    pub fn value(&self) -> u8 {
        match *self {
            Action::ROCK => 1,
            Action::PAPER => 2,
            Action::SCISSORS => 3,
        }
    }
}

enum Result {
    LOSE = 0,
    TIE = 3,
    WIN = 6,
}

struct Round {
    opponent_action: Action,
    player_action: Action,
    opponent_result: Result,
    player_result: Result,
    opponenet_points: u8,
    player_points: u8,
}

fn main() {
    calculate_round("B", "Z");
}

fn calculate_round(encrypted_opponent_action: &str, encrypted_player_action: &str) -> Round {
    let round: Round;

    match encrypted_opponent_action {
        "A" => round.opponent_action = Action::ROCK,
        "B" => round.opponent_action = Action::PAPER,
        "C" => round.opponent_action = Action::SCISSORS,
        _ => panic!("Invalid opponent code"),
    }

    match encrypted_player_action {
        "X" => round.player_action = Action::ROCK,
        "Y" => round.player_action = Action::PAPER,
        "Z" => round.player_action = Action::SCISSORS,
        _ => panic!("Invalid player code"),
    }

    // println!("round.opponent_action: {:?}", round.opponent_action);
    // println!(
    //     "round.opponent_action.value(): {:?}",
    //     round.opponent_action.value()
    // );
    // println!("round.player_action: {:?}", round.player_action);
    // println!(
    //     "round.player_action.value(): {:?}",
    //     round.player_action.value()
    // );
    // println!("round.result: {}", round.result);
    return round;
}
