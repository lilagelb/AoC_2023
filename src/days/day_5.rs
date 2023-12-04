use std::fs;
use crate::{Answer, Day};

pub struct Day5;

impl Day for Day5 {
    type PartOneType = usize;
    type PartTwoType = usize;

    fn solve(&mut self, input_file: &str) -> Answer<Self::PartOneType, Self::PartTwoType> {
        let input = fs::read_to_string(input_file).unwrap();
        todo!()
        Answer { part_1: None, part_2: None }
    }
}