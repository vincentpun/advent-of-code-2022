use crate::io::read_contents;

const INPUT_PATH: &str = "inputs/d2.txt";

pub fn run() {
    // Part one.
    let play_guide = parse_play_strategy();

    let winning_strategy = WinningStrategy {};
    let total_score: i32 = play_guide
        .iter()
        .map(|round| winning_strategy.get_score(round))
        .sum();
    println!("Part one: total score = {total_score}");

    // Part two.
    let play_guide = parse_play_strategy_part_two();
    let winning_strategy = WinningStrategy {};
    let total_score: i32 = play_guide
        .iter()
        .map(|round| winning_strategy.get_score(round))
        .sum();
    println!("Part two: total score = {total_score}");
}

#[derive(Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Unexpected character"),
        }
    }
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Round {
    opponent_play: Shape,
    self_play: Shape,
}

struct WinningStrategy {}

impl WinningStrategy {
    fn get_score(&self, round: &Round) -> i32 {
        let outcome_score = match (&round.opponent_play, &round.self_play) {
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => 3,
            _ => 0,
        };

        round.self_play.score() + outcome_score
    }
}

fn parse_play_strategy() -> Vec<Round> {
    let playbook_contents = read_contents(INPUT_PATH).expect("input should be readable");

    let mut rounds = Vec::new();
    for round in playbook_contents.lines() {
        let mut chars = round.chars();
        let opponent_play = Shape::from(chars.next().expect("Should have opponent play"));
        assert!(chars.next().expect("Should have opponent play") == ' ');
        let self_play = Shape::from(chars.next().expect("Should have opponent play"));

        rounds.push(Round {
            opponent_play,
            self_play,
        })
    }

    rounds
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unexpected character"),
        }
    }
}

fn get_play_for_outcome(opponent_play: &Shape, outcome: &Outcome) -> Shape {
    match (opponent_play, outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (_, Outcome::Draw) => opponent_play.clone(),
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
    }
}

fn parse_play_strategy_part_two() -> Vec<Round> {
    let playbook_contents = read_contents(INPUT_PATH).expect("input should be readable");

    let mut rounds = Vec::new();
    for round in playbook_contents.lines() {
        let mut chars = round.chars();
        let opponent_play = Shape::from(chars.next().expect("Should have opponent play"));
        assert!(chars.next().expect("Should have opponent play") == ' ');
        let outcome = Outcome::from(chars.next().expect("Should have outcome"));
        let self_play = get_play_for_outcome(&opponent_play, &outcome);

        rounds.push(Round {
            opponent_play,
            self_play,
        })
    }

    rounds
}
