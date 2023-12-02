use crate::{Answer, Day};
use std::fs;
use std::ops::Not;
use regex::Regex;

pub struct Day2;

impl Day for Day2 {
    type PartOneType = usize;
    type PartTwoType = usize;

    fn solve(&mut self, input_file: &str) -> Answer<Self::PartOneType, Self::PartTwoType> {
        let input = fs::read_to_string(input_file).unwrap();
        let line_parsing_regex =
            Regex::new(r"Game (?P<id>\d+): (?P<reveals>.*)").unwrap();
        let mut part_1 = 0_usize;
        let mut part_2 = 0_usize;

        for line in input.lines() {
            let mut part_1_impossible = false;
            let mut part_2_minimums: [usize; 3] = [0, 0, 0];

            let captures = line_parsing_regex.captures(line).unwrap();
            let reveals = captures.name("reveals")
                .unwrap()
                .as_str()
                .split("; ")
                .map(|reveal| {
                    let mut counts = CubeReveal::new();
                    for colour in reveal.split(", ") {
                            let colour = colour.split(" ").collect::<Vec<&str>>();
                            let count = colour[0].parse::<usize>().unwrap();
                            let colour = colour[1];
                            if colour == "red" {
                                counts.red_count = count;
                            } else if colour == "green" {
                                counts.green_count = count;
                            } else { // blue
                                counts.blue_count = count;
                            }
                    }
                    counts
                })
                .collect::<Vec<CubeReveal>>();

            for reveal in reveals {
                if part_1_impossible.not()
                    && (reveal.red_count > 12 || reveal.green_count > 13 || reveal.red_count > 14)
                {
                    part_1_impossible = true;
                }
                part_2_minimums = [
                    part_2_minimums[0].max(reveal.red_count),
                    part_2_minimums[1].max(reveal.green_count),
                    part_2_minimums[2].max(reveal.blue_count),
                ];
            }

            part_1 += if part_1_impossible {
                0
            } else {
                captures.name("id")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap()
            };

            part_2 += part_2_minimums[0] * part_2_minimums[1] * part_2_minimums[2];
        }

        Answer { part_1: Some(part_1), part_2: Some(part_2) }
    }
}

struct CubeReveal {
    red_count: usize,
    green_count: usize,
    blue_count: usize,
}
impl CubeReveal {
    fn new() -> CubeReveal {
        CubeReveal {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        }
    }
}