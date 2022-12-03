enum HandShape {
    Rock,
    Paper,
    Scissors,
}
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

fn line_to_round_description() -> RoundDescription {
    todo!()
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

fn solution() {
    todo!()
}

#[cfg(test)]
mod tests_day_two {
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
}
