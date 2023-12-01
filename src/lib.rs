pub mod days;

use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub trait Day {
    type PartOneType;
    type PartTwoType;

    fn solve(&mut self, input_file: &str) -> Answer<Self::PartOneType, Self::PartTwoType>;

    fn benchmark(&mut self, input_file: &str) -> BenchmarkedAnswer<Self::PartOneType, Self::PartTwoType> {
        let start = Instant::now();
        let answer = self.solve(input_file);
        let time = start.elapsed();
        BenchmarkedAnswer { answer, time }
    }
}

pub struct Answer<T, U> {
    part_1: Option<T>,
    part_2: Option<U>,
}
impl<T: Display, U: Display> Display for Answer<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "    Part 1: {}\n    Part 2: {}",
               self.part_1.as_ref().map_or_else(|| "uncomputed".to_string(), |value| format!("{}", value)),
               self.part_2.as_ref().map_or_else(|| "uncomputed".to_string(), |value| format!("{}", value)),
        )
    }
}

pub struct BenchmarkedAnswer<T, U> {
    answer: Answer<T, U>,
    time: Duration,
}
impl<T: Display, U: Display> Display for BenchmarkedAnswer<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n    Time:   {} ms", self.answer, self.time.as_millis())
    }
}