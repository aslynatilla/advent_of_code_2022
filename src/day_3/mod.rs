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
