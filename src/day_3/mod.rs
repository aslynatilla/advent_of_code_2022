use std::{convert::TryFrom, fs::read_to_string, str::Lines};

struct RucksackItem(char);

impl TryFrom<char> for RucksackItem {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_alphabetic() {
            Ok(RucksackItem(value))
        } else {
            Err(format!("Unexpected string: {}", value))
        }
    }
}

impl RucksackItem {
    fn priority(&self) -> u64 {
        let a_prio: u64 = 'a'.to_ascii_lowercase().into();
        let uppercase_a_prio: u64 = 'A'.to_ascii_uppercase().into();

        match self.0 {
            ch @ 'a'..='z' => <char as Into<u64>>::into(ch.to_ascii_lowercase()) - a_prio + 1,
            ch @ 'A'..='Z' => {
                <char as Into<u64>>::into(ch.to_ascii_uppercase()) - uppercase_a_prio + 27
            }
            _ => panic!("Unexpected input"),
        }
    }
}

struct Rucksack {
    items: Vec<RucksackItem>,
}

impl From<&str> for Rucksack {
    fn from(line: &str) -> Self {
        Rucksack {
            items: line
                .chars()
                .map(|ch| RucksackItem::try_from(ch).unwrap())
                .collect(),
        }
    }
}

impl Rucksack {
    fn compartments(&self) -> (&[RucksackItem], &[RucksackItem]) {
        let sack_length = self.items.len();
        assert!(sack_length.rem_euclid(2) == 0);
        let half_size = sack_length / 2;
        (&self.items[0..half_size], &self.items[half_size..])
    }
}

struct RucksackList {
    rucksacks: Vec<Rucksack>,
}

impl<'a> From<Lines<'a>> for RucksackList {
    fn from(lines: Lines) -> Self {
        RucksackList {
            rucksacks: { lines.map(Rucksack::from).collect() },
        }
    }
}

impl IntoIterator for RucksackList {
    type Item = Rucksack;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rucksacks.into_iter()
    }
}

pub fn solution() -> (u64, u64) {
    let result = 0u64;

    let _list: RucksackList = match read_to_string("assets/input_d3.txt") {
        Ok(data) => data,
        Err(e) => panic!("Input file not placed correctly\nReported as: {}", e),
    }
    .lines()
    .into();

    (result, 0u64)
}

mod tests {

    #[test]
    fn priorities() {
        let characters: Vec<_> = vec!['p', 'L', 'P', 'v', 't', 's']
            .into_iter()
            .map(super::RucksackItem::try_from)
            .map(Result::unwrap)
            .collect();
        let expected_priorities = vec![16u64, 38, 42, 22, 20, 19];
        assert!(characters
            .iter()
            .zip(expected_priorities.iter())
            .all(|(char, &prio)| {
                println!("{}", char.priority());
                char.priority() == prio
            }))
    }
}
