pub mod days;

use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub trait Day {
    type PartOneType;
    type PartTwoType;

    fn solve(&mut self, input_file: &str) -> Answer<Self::PartOneType, Self::PartTwoType>;

    fn timed(&mut self, input_file: &str) -> TimedAnswer<Self::PartOneType, Self::PartTwoType> {
        let start = Instant::now();
        let answer = self.solve(input_file);
        let time = start.elapsed();
        TimedAnswer { answer, time }
    }
}

pub struct Answer<T, U> {
    part_1: Option<T>,
    part_2: Option<U>,
}
impl<T: Display, U: Display> Display for Answer<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "    Part 1: {:>10}\n    Part 2: {:>10}",
               self.part_1.as_ref().map_or_else(|| "uncomputed".to_string(), |value| format!("{}", value)),
               self.part_2.as_ref().map_or_else(|| "uncomputed".to_string(), |value| format!("{}", value)),
        )
    }
}

pub struct TimedAnswer<T, U> {
    answer: Answer<T, U>,
    time: Duration,
}
impl<T: Display, U: Display> Display for TimedAnswer<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let millis_more_precise = self.time.as_micros() as f64 / 1000.0;
        write!(f, "{}\n    Time:   {:>10} ms", self.answer, millis_more_precise)
    }
}