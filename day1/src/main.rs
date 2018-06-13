/// http://adventofcode.com/2017/day/1
///
/// Initially began solving this as a few loops + index variables but quickly
/// became displeased with how messy things were getting.
///
/// Next, I tried using the std::iter::Iterator and std::iter::IntoIterator traits
/// which led to a very simple solution to Part 1.
///
/// Because Part 2 requires the ability to "jump ahead", I decided to scrap the
/// Iterator approach and instead implement std::ops::Index to allow for rewriting
/// the index access location for arbitrary values of "looping" through the input vec.

use std::io::{self, Read};

#[test]
fn part1() {
    assert_eq!(3, Captcha::new("1122".to_string()).solve1());
    assert_eq!(4, Captcha::new("1111".to_string()).solve1());
    assert_eq!(0, Captcha::new("1234".to_string()).solve1());
    assert_eq!(9, Captcha::new("91212139".to_string()).solve1());
}

#[test]
fn part2() {
    assert_eq!(6, Captcha::new("1212".to_string()).solve2());
    assert_eq!(0, Captcha::new("1221".to_string()).solve2());
    assert_eq!(4, Captcha::new("123425".to_string()).solve2());
    assert_eq!(12, Captcha::new("123123".to_string()).solve2());
    assert_eq!(4, Captcha::new("12131415".to_string()).solve2());
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    println!("Algorithm 1: {}", Captcha::new(input.clone()).solve1());
    println!("Algorithm 2: {}", Captcha::new(input.clone()).solve2());
}

struct Captcha {
    input: Vec<u32>,
}

impl Captcha {
    fn new(input: String) -> Self {
        let mut inp: Vec<u32> = Vec::new();

        for c in input.chars() {
            let digit = c.to_digit(10);
            if digit.is_some() {
                inp.push(digit.unwrap());
            }
        }

        Captcha { input: inp }
    }

    fn solve1(self) -> u32 {
        let mut accumulator = 0;

        for i in 0..self.input.len() {
            if self[i] == self[i + 1] {
                accumulator += self[i];
            }
        }

        return accumulator;
    }

    fn solve2(self) -> u32 {
        let mut accumulator = 0;
        let halfway = self.input.len() / 2;

        for i in 0..self.input.len() {
            if self[i] == self[i + halfway] {
                accumulator += self[i];
            }
        }

        return accumulator;
    }
}

// Implementing a custom function for Vec<u32> would probably be more
// readable in Captcha.solve().
impl std::ops::Index<usize> for Captcha {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.input[index % self.input.len()]
    }
}
