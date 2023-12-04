use std::collections::HashSet;
use std::fs;
use std::ops::Not;
use regex::Regex;
use crate::{Answer, Day};

pub struct Day4;

impl Day for Day4 {
    type PartOneType = usize;
    type PartTwoType = usize;

    fn solve(&mut self, input_file: &str) -> Answer<Self::PartOneType, Self::PartTwoType> {
        let parse_regex= Regex::new(r"Card +\d+: (?P<winning>.*) \| (?P<elf_has>.*)").unwrap();
        let input_string = fs::read_to_string(input_file)
            .unwrap();
        let input_vec = input_string.lines()
            .collect::<Vec<&str>>();

        let mut part_1 = 0;
        let mut card_counts = Vec::new();
        card_counts.resize_with(input_vec.len(), || 1);

        for (card_number, line) in input_vec.iter().enumerate()
        {
            let captures = parse_regex.captures(line).unwrap();
            let winning_numbers = captures.name("winning")
                .unwrap()
                .as_str()
                .split(" ")
                .filter_map(|number| number.parse().ok() )
                .collect::<HashSet<usize>>();
            let elf_has = captures.name("elf_has")
                .unwrap()
                .as_str()
                .split(" ")
                .filter_map(|number| number.parse().ok())
                .collect::<HashSet<usize>>();

            let intersection_length = winning_numbers.intersection(&elf_has).collect::<HashSet<&usize>>().len();
            if intersection_length != 0 {
                part_1 += 1 << (intersection_length- 1);
            }
            for i in (card_number+1)..=(card_number + intersection_length) {
                card_counts[i] += card_counts[card_number];
            }
        }

        Answer { part_1: Some(part_1), part_2: Some(card_counts.iter().sum()) }
    }
}