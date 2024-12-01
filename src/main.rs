use aoc_core::{run, AocTask};
use day01::Day01;

mod aoc_core;
mod day01;

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
    run(days.vec);
}
