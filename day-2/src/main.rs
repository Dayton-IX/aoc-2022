#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
enum Result {
    LOSE = 0,
    TIE = 3,
    WIN = 6,
}

impl Result {
    pub fn value(&self) -> u8 {
        match *self {
            Result::LOSE => 0,
            Result::TIE => 3,
            Result::WIN => 6,
        }
    }
}

#[derive(Debug)]
struct Round {
    opp_action: Action,
    player_action: Action,
    result: Result,
    points: u8,
}

fn main() {
    let player_round: Round = calculate_round("B", "Z");
    println!("player_round: {:?}", player_round);
}

fn calculate_round(encrypted_opponent_action: &str, encrypted_player_action: &str) -> Round {
    let opp_action: Action;
    let player_action: Action;
    let result: Result;

    match encrypted_opponent_action {
        "A" => opp_action = Action::ROCK,
        "B" => opp_action = Action::PAPER,
        "C" => opp_action = Action::SCISSORS,
        _ => panic!("Invalid opponent code"),
    }

    match encrypted_player_action {
        "X" => player_action = Action::ROCK,
        "Y" => player_action = Action::PAPER,
        "Z" => player_action = Action::SCISSORS,
        _ => panic!("Invalid player code"),
    }

    if (opp_action.value() == 1 && player_action.value() == 2)
        || (opp_action.value() == 2 && player_action.value() == 3)
        || (opp_action.value() == 3 && player_action.value() == 1)
    {
        result = Result::WIN;
    } else if opp_action.value() == player_action.value() {
        result = Result::TIE;
    } else {
        result = Result::LOSE;
    }

    let round: Round = Round {
        opp_action,
        player_action: player_action.clone(),
        result: result.clone(),
        points: player_action.value() + result.value(),
    };

    println!("round.opponent_action: {:?}", round.opp_action);
    println!(
        "round.opponent_action.value(): {:?}",
        round.opp_action.value()
    );
    println!("round.player_action: {:?}", round.player_action);
    println!(
        "round.player_action.value(): {:?}",
        round.player_action.value()
    );
    println!("round.result: {:?}", round.result);
    println!("round.result.value(): {:?}", round.result.value());
    println!("round.points: {}", round.points);
    return round;
}
