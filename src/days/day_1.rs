use crate::{Answer, Day};
use std::fs;
use std::ops::Not;

pub struct Day1;

impl Day for Day1 {
    type PartOneType = usize;
    type PartTwoType = usize;

    fn solve(&mut self, input_file: &str) -> Answer<Self::PartOneType, Self::PartTwoType> {
        let input = fs::read_to_string(input_file).unwrap();
        let mut part_1 = 0;
        let mut part_2 = 0;

        for line in input.lines() {
            let mut digits = vec![];
            let mut digits_part_2 = vec![];

            let mut line = line;
            while line.is_empty().not() {
                let first_char: char = line.as_bytes()[0].into();
                if first_char.is_ascii_digit() {
                    digits.push(first_char as usize - 48);
                    digits_part_2.push(first_char as usize - 48);
                    line = &line[1..];
                } else if line.starts_with("one") {
                    digits_part_2.push(1);
                    // only advance by 2 as the 'e' could be the start of 'eight'
                    line = &line[2..];
                } else if line.starts_with("two") {
                    digits_part_2.push(2);
                    // only advance by 2 as the 'o' could be the start of 'one'
                    line = &line[2..];
                } else if line.starts_with("three") {
                    digits_part_2.push(3);
                    // only advance by 4 as the 'e' could be the start of 'eight'
                    line = &line[4..];
                } else if line.starts_with("four") {
                    digits_part_2.push(4);
                    line = &line[4..];
                } else if line.starts_with("five") {
                    digits_part_2.push(5);
                    // only advance by 3 as the 'e' could be the start of 'eight'
                    line = &line[3..];
                } else if line.starts_with("six") {
                    digits_part_2.push(6);
                    line = &line[3..];
                } else if line.starts_with("seven") {
                    digits_part_2.push(7);
                    // only advance by 4 as the 'n' could be the start of 'nine'
                    line = &line[4..];
                } else if line.starts_with("eight") {
                    digits_part_2.push(8);
                    // only advance by 4 as the 't' could be the start of 'two'
                    line = &line[4..];
                } else if line.starts_with("nine") {
                    digits_part_2.push(9);
                    // only advance by 3 as the 'e' could be the start of 'eight'
                    line = &line[3..];
                } else {
                    line = &line[1..];
                }
            }

            part_1 += 10 * digits[0] + digits.last().unwrap();
            part_2 += 10 * digits_part_2[0] + digits_part_2.last().unwrap();
        }

        Answer { part_1: Some(part_1), part_2: Some(part_2) }
    }
}