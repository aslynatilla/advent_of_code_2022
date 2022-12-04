use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
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
        .map(|s| match s {
            "A" | "X" => HandShape::Rock,
            "B" | "Y" => HandShape::Paper,
            "C" | "Z" => HandShape::Scissors,
            _ => panic!("Unexpected string in input line"),
        })
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

pub fn solution() -> u32 {
    let input_data = match read_to_string("assets/input_day_two.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Input file not placed correctly\nReported as: {}", e),
    };

    input_data
        .lines()
        .map(line_to_round_description)
        .map(|line| round_score(&line))
        .sum()
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
