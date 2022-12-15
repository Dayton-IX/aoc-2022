use std::fs;

#[derive(Debug, Clone)]
enum Action {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Action {
    pub fn value(&self) -> u32 {
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
    pub fn value(&self) -> u32 {
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
    points: u32,
}

fn main() {
    let coded_rounds = read_rounds();

    let mut total_points: u32 = 0;

    for coded_round in coded_rounds {
        println!("coded_round: {}", coded_round);
        if coded_round.len() != 2 {
            continue;
        }
        let player_round: Round = calculate_round(
            coded_round.chars().nth(0).unwrap(),
            coded_round.chars().nth(1).unwrap(),
        );
        println!("player_round: {:?}", player_round);

        total_points += player_round.points;
    }

    println!("total_points: {}", total_points);
}

fn read_rounds() -> Vec<String> {
    let content = fs::read_to_string("input.txt").expect("Should read the file");

    let split_rounds = content.split("\n");
    let mut coded_rounds: Vec<String> = vec![];

    for split_round in split_rounds {
        println!("split_round: {}", split_round);
        coded_rounds.push(split_round.split_whitespace().collect())
    }

    return coded_rounds;
}

fn interpret_player_action(opp_action: Action, encrypted_player_action: char) -> Action {
    let player_action: Action;
    if opp_action == Action::SCISSORS {
        match encrypted_player_action {
            'X' => player_action = Action::ROCK,
            'Y' => player_action = Action::PAPER,
            'Z' => player_action = Action::SCISSORS,
            _ => panic!("Invalid player code"),
        }
    }
}

fn calculate_round(encrypted_opponent_action: char, encrypted_player_action: char) -> Round {
    let opp_action: Action;
    let result: Result;

    println!(
        "coded actions: {} {}",
        encrypted_opponent_action, encrypted_player_action
    );

    match encrypted_opponent_action {
        'A' => opp_action = Action::ROCK,
        'B' => opp_action = Action::PAPER,
        'C' => opp_action = Action::SCISSORS,
        _ => panic!("Invalid opponent code"),
    }

    let player_action = interpret_player_action(opp_action, encrypted_player_action);

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
