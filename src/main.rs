use aoc_core::run;

mod aoc_core;
mod days;

use aoc_register::aoc_register;

use crate::aoc_core::AocTask;

aoc_register!();

fn main() {
    run(days());
}
