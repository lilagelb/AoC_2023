use std::collections::HashMap;
use crate::{Answer, Day};
use std::fs;
use regex::Regex;

pub struct Day3;

impl Day for Day3 {
    type PartOneType = usize;
    type PartTwoType = usize;

    fn solve(&mut self, input_file: &str) -> Answer<Self::PartOneType, Self::PartTwoType> {
        let input= fs::read_to_string(input_file)
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let mut part_1 = 0;
        let mut part_2 = 0;
        let mut gears = HashMap::new();
        let number_regex = Regex::new(r"(\d+)").unwrap();

        for (line_number, line) in input.iter().enumerate() {
            for regex_match in number_regex.find_iter(line) {
                let part_number = regex_match.as_str().parse::<usize>().unwrap();
                if process_part_number(line_number, regex_match.start(), regex_match.end(), part_number, &input, &mut gears) {
                    part_1 += part_number;
                }
            }
        }

        for part_numbers in gears.values() {
            if part_numbers.len() != 2 {
                continue;
            }
            part_2 += part_numbers[0] * part_numbers[1];
        }

        Answer { part_1: Some(part_1), part_2: Some(part_2) }
    }
}

fn process_part_number(line_number: usize, start: usize, end: usize, part_number: usize, input: &Vec<String>, gears: &mut HashMap<(usize, usize), Vec<usize>>) -> bool {
    // done this way to get around ownership problems in the loop
    let search_range_start = if start == 0 { 0 } else { start - 1 };
    let search_range_end = if end == input[0].len() { end } else { end + 1 };
    let line_search_range_start = if line_number == 0 { 0 } else { line_number - 1 };
    let line_search_range_end = (if line_number == input.len() - 1 { line_number } else { line_number + 1 }) + 1;

    let lines_to_search = &input[line_search_range_start..line_search_range_end];
    for (line_offset, line) in lines_to_search.iter().enumerate() {
        for (char_offset, character) in line[search_range_start..search_range_end].chars().enumerate() {
            if character != '.' && character.is_ascii_punctuation() {
                if character == '*' {
                    gears.entry((line_search_range_start + line_offset, search_range_start + char_offset))
                        .or_insert_with(|| Vec::new())
                        .push(part_number);
                }
                return true;
            }
        }
    }

    false
}