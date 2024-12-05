use std::str::Chars;

use crate::aoc_core::AocTask;
use itertools::PeekingNext;

pub struct Day03Manual;

struct Scanner<'a> {
    iter: Chars<'a>,
    tokens: &'a mut Vec<(i32, i32)>,
}

impl Scanner<'_> {
    fn new<'a>(source: &'a String, tokens: &'a mut Vec<(i32, i32)>) -> Scanner<'a> {
        Scanner {
            iter: source.chars(),
            tokens,
        }
    }

    #[inline]
    fn peeking_next(&mut self, expected: char) -> Option<()> {
        self.iter.peeking_next(|c| *c == expected)?;
        Some(())
    }

    fn consume_digits(&mut self) -> Option<String> {
        let mut digits = String::from("");
        loop {
            let digit = self.iter.peeking_next(char::is_ascii_digit);
            match digit {
                Some(d) => {
                    digits.push(d);
                }
                None => {
                    break;
                }
            }
        }
        if digits.is_empty() {
            return None;
        }
        return Some(digits);
    }

    fn token_mul(&mut self) -> Option<()> {
        self.peeking_next('(')?;
        let f1 = self.consume_digits()?;
        self.peeking_next(',')?;
        let f2 = self.consume_digits()?;
        self.peeking_next(')')?;
        self.tokens
            .push((f1.parse::<i32>().unwrap(), f2.parse::<i32>().unwrap()));
        Some(())
    }

    fn scan(&mut self) -> &Vec<(i32, i32)> {
        loop {
            match self.iter.next() {
                Some(c) => {
                    if c == 'm'
                        && self.iter.peeking_next(|c| *c == 'u').is_some()
                        && self.iter.peeking_next(|c| *c == 'l').is_some()
                    {
                        self.token_mul();
                    }
                }
                None => {
                    break;
                }
            }
        }
        &self.tokens
    }
}

impl AocTask for Day03Manual {
    fn solve_a(&self, contents: String) -> crate::aoc_core::AocResult {
        let mut tokens = Vec::new();
        let mut scanner = Scanner::new(&contents, &mut tokens);
        scanner.scan();
        Ok(tokens.iter().map(|(x, y)| (*x as i64) * (*y as i64)).sum())
    }

    fn solve_b(&self, _contents: String) -> crate::aoc_core::AocResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::aoc_core::read_file;

    use super::*;

    #[test]
    fn test_solve_a_sample() -> Result<(), String> {
        let contents = read_file(Path::new(file!()), "sample.txt");

        Day03Manual.solve_a(contents)?;
        assert!(false);
        Ok(())
    }

    #[test]
    fn test_solve_a_input() -> Result<(), String> {
        let contents = read_file(Path::new(file!()), "input.txt");

        Day03Manual.solve_a(contents)?;
        assert!(false);
        Ok(())
    }
}
