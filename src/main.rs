use aoc_core::{run, AocTask};
use day01::Day01;
use day02::Day02;
use day03::Day03;

mod aoc_core;
mod day01;
mod day02;
mod day03;

struct Days {
    vec: Vec<Box<dyn AocTask>>,
}
impl Days {
    fn register<T>(&mut self, task: T)
    where
        T: AocTask + 'static,
    {
        self.vec.push(Box::new(task));
    }
}

fn main() {
    let mut days = Days { vec: Vec::new() };
    days.register(Day01);
    days.register(Day02);
    days.register(Day03);
    run(days.vec);
}
