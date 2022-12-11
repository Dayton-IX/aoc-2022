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

    pub fn opponent_code(&self) -> String {
        match *self {
            Action::ROCK => "A".to_string(),
            Action::PAPER => "A".to_string(),
            Action::SCISSORS => "C".to_string(),
        }
    }
}

struct Round {
    opponent_action: Action,
    player_action: Action,
    result: u8,
}

fn main() {
    calculate_round("B".to_string(), "Z".to_string());
}

fn calculate_round(encrypted_opponent_action: String, encrypted_player_action: String) -> Round {
    let round: Round;

    println!("round.opponent_action: {:?}", round.opponent_action);
    println!(
        "round.opponent_action.value(): {:?}",
        round.opponent_action.value()
    );
    println!("round.player_action: {:?}", round.player_action);
    println!(
        "round.player_action.value(): {:?}",
        round.player_action.value()
    );
    println!("round.result: {}", round.result);
    return round;
}
