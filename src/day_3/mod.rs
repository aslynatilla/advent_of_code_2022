use itertools::Itertools;
use std::{fs::read_to_string, str::Lines};

#[derive(Clone, Copy, PartialEq)]
struct RucksackItem(char);

impl From<char> for RucksackItem {
    fn from(value: char) -> Self {
        if value.is_ascii_alphabetic() {
            RucksackItem(value)
        } else {
            panic!("Unexpected string: {value}")
        }
    }
}

impl RucksackItem {
    fn priority(&self) -> u64 {
        let a_prio: u64 = 'a'.to_ascii_lowercase().into();
        let uppercase_a_prio: u64 = 'A'.to_ascii_uppercase().into();

        match self {
            RucksackItem(ch @ 'a'..='z') => 1 + u64::from(*ch) - a_prio,
            RucksackItem(ch @ 'A'..='Z') => 27 + u64::from(*ch) - uppercase_a_prio,
            _ => panic!("Unexpected input"),
        }
    }
}

#[derive(Clone)]
struct Rucksack {
    items: Vec<RucksackItem>,
}

impl From<&str> for Rucksack {
    fn from(line: &str) -> Self {
        Rucksack {
            items: line.chars().map(RucksackItem::from).collect(),
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

    fn out_of_place_item(&self) -> Option<RucksackItem> {
        let (left, right) = self.compartments();
        left.iter().find(|&item| right.contains(item)).copied()
    }
}

impl IntoIterator for Rucksack {
    type Item = char;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .into_iter()
            .map(|item| item.0)
            .collect::<Vec<_>>()
            .into_iter()
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
    let list: RucksackList = match read_to_string("assets/input_d3.txt") {
        Ok(data) => data,
        Err(e) => panic!("Input file not placed correctly\nReported as: {}", e),
    }
    .lines()
    .into();

    let rucksacks: Vec<_> = list.into_iter().collect();
    let first_part_result = rucksacks
        .iter()
        .map(|rucksack| rucksack.out_of_place_item())
        .map(|item| item.map_or(0, |i| i.priority()))
        .sum();

    let rucksack_chunks = rucksacks.iter().chunks(3);
    let mut second_part_result = 0u64;
    for chunk in &rucksack_chunks {
        let three_sacks: Vec<_> = chunk.collect();
        second_part_result += three_sacks[0]
            .clone()
            .into_iter()
            .find(|first_ch| {
                three_sacks[1]
                    .clone()
                    .into_iter()
                    .any(|char| char.eq(first_ch))
                    && three_sacks[2]
                        .clone()
                        .into_iter()
                        .any(|char| char.eq(first_ch))
            })
            .map(RucksackItem::from)
            .map(|x| x.priority())
            .unwrap();
    }

    (first_part_result, second_part_result)
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
            .all(|(char, &prio)| { char.priority() == prio }))
    }

    #[test]
    fn out_of_place_items() {
        let rucksacks: Vec<_> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
            "CrZsJsPPZsGzwwALwLmpwMDw",
        ]
        .into_iter()
        .map(super::Rucksack::from)
        .collect();

        assert!(rucksacks
            .iter()
            .zip(
                vec![
                    Some('p'),
                    Some('L'),
                    Some('P'),
                    Some('v'),
                    Some('t'),
                    Some('s'),
                    None
                ]
                .iter()
            )
            .all(
                |(rucksack, &out_of_place_char)| rucksack.out_of_place_item()
                    == out_of_place_char
                        .map(super::RucksackItem::try_from)
                        .map(Result::unwrap)
            ))
    }
}
