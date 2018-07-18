use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let instructions: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    println!("{} steps", part1(instructions.clone()));
    println!("{} steps", part2(instructions.clone()));
}

fn part1(mut instructions: Vec<i32>) -> usize {
    let mut i = 0;
    let mut steps = 0;

    loop {
        // Because I don't want to define a custom Vec that allows indexing by isize.
        if i < 0 {
            break;
        }

        match instructions.get_mut(i as usize) {
            None => break,
            Some(val) => {
                steps += 1;

                i += *val;
                *val += 1;
            }
        }
    }

    steps
}

#[test]
fn test_part1() {
    assert_eq!(5, part1(vec![0, 3, 0, 1, -3]));
}

fn part2(mut instructions: Vec<i32>) -> usize {
    let mut i = 0;
    let mut steps = 0;

    loop {
        // Because I don't want to define a custom Vec that allows indexing by isize.
        if i < 0 {
            break;
        }

        match instructions.get_mut(i as usize) {
            None => break,
            Some(val) => {
                steps += 1;

                i += *val;
                *val += if *val >= 3 { -1 } else { 1 };
            }
        }
    }

    steps
}

#[test]
fn test_part2() {
    assert_eq!(10, part2(vec![0, 3, 0, 1, -3]));
}
