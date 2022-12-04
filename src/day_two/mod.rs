use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for HandShape {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(HandShape::Rock),
            "B" | "Y" => Ok(HandShape::Paper),
            "C" | "Z" => Ok(HandShape::Scissors),
            _ => Err("Unexpected string in input line"),
        }
    }
}

impl HandShape {
    fn from_string_slice(value: &str) -> Self {
        match Self::try_from(value) {
            Ok(shape) => shape,
            Err(error_str) => panic!("{}", error_str),
        }
    }
}

#[derive(Debug, PartialEq)]
struct RoundDescription {
    theirs: HandShape,
    ours: HandShape,
}

impl RoundDescription {
    fn new(first: HandShape, second: HandShape) -> RoundDescription {
        RoundDescription {
            theirs: first,
            ours: second,
        }
    }
}

fn line_to_round_description(line: &str) -> RoundDescription {
    let handshapes = line
        .split(char::is_whitespace)
        .map(HandShape::from_string_slice)
        .collect::<Vec<HandShape>>();

    assert!(handshapes.len() == 2);

    RoundDescription::new(
        *handshapes.first().expect("Line parsing error"),
        *handshapes.last().expect("Line parsing error"),
    )
}

fn round_score(round: &RoundDescription) -> u32 {
    let our_shape_score = match &round.ours {
        HandShape::Rock => 1u32,
        HandShape::Paper => 2u32,
        HandShape::Scissors => 3u32,
    };

    let our_outcome_score = match (&round.theirs, &round.ours) {
        (HandShape::Rock, HandShape::Paper)
        | (HandShape::Paper, HandShape::Scissors)
        | (HandShape::Scissors, HandShape::Rock) => 6u32,

        (HandShape::Rock, HandShape::Rock)
        | (HandShape::Paper, HandShape::Paper)
        | (HandShape::Scissors, HandShape::Scissors) => 3u32,

        _ => 0u32,
    };

    our_shape_score + our_outcome_score
}

enum RoundResult {
    Lose,
    Draw,
    Win,
}

fn result_and_their_move_to_round_description(result: &str, theirs: &str) -> RoundDescription {
    let expected_result = match result {
        "X" => RoundResult::Lose,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        _ => panic!("Unexpected string in input line"),
    };

    let their_shape = match theirs {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scissors,
        _ => panic!("Unexpected string in input line"),
    };

    let right_move = match (their_shape, expected_result) {
        (HandShape::Rock, RoundResult::Lose) => HandShape::Scissors,
        (HandShape::Rock, RoundResult::Draw) => HandShape::Rock,
        (HandShape::Rock, RoundResult::Win) => HandShape::Paper,
        (HandShape::Paper, RoundResult::Lose) => HandShape::Rock,
        (HandShape::Paper, RoundResult::Draw) => HandShape::Paper,
        (HandShape::Paper, RoundResult::Win) => HandShape::Scissors,
        (HandShape::Scissors, RoundResult::Lose) => HandShape::Paper,
        (HandShape::Scissors, RoundResult::Draw) => HandShape::Scissors,
        (HandShape::Scissors, RoundResult::Win) => HandShape::Rock,
    };

    RoundDescription::new(their_shape, right_move)
}

pub fn solution() -> (u32, u32) {
    let input_data = match read_to_string("assets/input_day_two.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Input file not placed correctly\nReported as: {}", e),
    };

    let shape_against_shape_score = input_data
        .lines()
        .map(line_to_round_description)
        .map(|line| round_score(&line))
        .sum();

    let shape_and_result_score = input_data
        .lines()
        .map(|s| s.split(char::is_whitespace).collect::<Vec<&str>>())
        .map(|line| {
            result_and_their_move_to_round_description(
                line.last().expect("Wrong format"),
                line.first().expect("Wrong format"),
            )
        })
        .map(|r| round_score(&r))
        .sum();

    (shape_against_shape_score, shape_and_result_score)
}

#[cfg(test)]
mod tests_day_two {
    use crate::day_two::line_to_round_description;

    use super::{round_score, HandShape, RoundDescription};

    #[test]
    fn score_test() {
        let first_round = RoundDescription::new(HandShape::Rock, HandShape::Paper);
        let second_round = RoundDescription::new(HandShape::Paper, HandShape::Rock);
        let third_round = RoundDescription::new(HandShape::Scissors, HandShape::Scissors);

        assert_eq!(round_score(&first_round), 8);
        assert_eq!(round_score(&second_round), 1);
        assert_eq!(round_score(&third_round), 6);
    }

    #[test]
    fn conversion_test() {
        let first_round = line_to_round_description("A Y");
        let second_round = line_to_round_description("B X");
        let third_round = line_to_round_description("C Z");
        assert_eq!(first_round.theirs, HandShape::Rock);
        assert_eq!(first_round.ours, HandShape::Paper);
        assert_eq!(
            second_round,
            RoundDescription::new(HandShape::Paper, HandShape::Rock)
        );
        assert_eq!(
            third_round,
            RoundDescription::new(HandShape::Scissors, HandShape::Scissors)
        );
    }
}
