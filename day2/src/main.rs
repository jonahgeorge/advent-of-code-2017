#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::{self, Read};
use test::Bencher;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let sheet = Sheet::from_string(input);
    println!("Algorithm 1: {}", sheet.clone().checksum1());
    println!("Algorithm 2: {}", sheet.clone().checksum2());
}

#[derive(PartialEq, Debug, Clone)]
struct Sheet {
    rows: Vec<Row>,
}

impl Sheet {
    fn new(rows: Vec<Row>) -> Self {
        Sheet { rows: rows }
    }

    fn from_string(buf: String) -> Self {
        let rows = buf.lines()
            .map(|line| {
                let columns = line.trim()
                            .split_whitespace()
                            .map(|column| column.parse::<u32>().unwrap()) // Question: Better way to handle invalid input?
                            .collect::<Vec<_>>();

                Row::new(columns)
            })
            .collect::<Vec<_>>();

        Sheet::new(rows)
    }

    fn checksum1(&self) -> u32 {
        self.rows.iter().fold(0, |acc, row| {
            return acc + row.checksum1();
        })
    }

    fn checksum2(&self) -> u32 {
        self.rows.iter().fold(0, |acc, row| {
            let checksum = row.checksum2();
            if checksum.is_some() {
                return acc + checksum.unwrap();
            }
            return acc;
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Row {
    columns: Vec<u32>,
}

// Permutates vals returning a vector of tuples
fn permutations(vals: &[u32]) -> Vec<(u32, u32)> {
    let mut permutations: Vec<(u32, u32)> = Vec::new();

    for (a, a_item) in vals.iter().enumerate() {
        for (b, b_item) in vals.iter().enumerate() {
            if a == b {
                continue;
            }

            permutations.push((*a_item, *b_item));
        }
    }

    permutations
}

impl Row {
    fn new(columns: Vec<u32>) -> Self {
        Row { columns: columns }
    }

    fn checksum2(&self) -> Option<u32> {
        for perm in permutations(&self.columns) {
            let remainder = perm.0 % perm.1;

            if remainder == 0 {
                return Some(std::cmp::max(perm.0, perm.1) / std::cmp::min(perm.0, perm.1));
            }
        }

        None
    }

    fn checksum1(&self) -> u32 {
        let res = &self.columns.iter().fold((None, None), |mut acc, column| {
            if acc.0.is_none() {
                acc.0 = Some(column);
            }

            if acc.1.is_none() {
                acc.1 = Some(column);
            }

            if column > acc.0.unwrap() {
                acc.0 = Some(column);
            }

            if column < acc.1.unwrap() {
                acc.1 = Some(column);
            }

            acc
        });

        return res.0.unwrap() - res.1.unwrap();

        // let mut max = None;
        // let mut min = None;

        // for c in &self.columns {
        //     if max.is_none() {
        //         max = Some(c);
        //     }

        //     if min.is_none() {
        //         min = Some(c);
        //     }

        //     if c > max.unwrap() {
        //         max = Some(c);
        //     }

        //     if c < min.unwrap() {
        //         min = Some(c);
        //     }
        // }

        // return max.unwrap() - min.unwrap();
    }
}

#[test]
fn row_checksum1() {
    assert_eq!(8, Row::new(vec![5, 1, 9, 5]).checksum1());
    assert_eq!(4, Row::new(vec![7, 5, 3]).checksum1());
    assert_eq!(6, Row::new(vec![2, 4, 6, 8]).checksum1());
}

#[test]
fn row_checksum2() {
    assert_eq!(Some(4), Row::new(vec![5, 9, 2, 8]).checksum2());
    assert_eq!(Some(3), Row::new(vec![9, 4, 7, 3]).checksum2());
    assert_eq!(Some(2), Row::new(vec![3, 8, 6, 5]).checksum2());
}

#[test]
fn sheet_from_str() {
    let sheet = Sheet::from_string("5 1 9 5\n7 5 3\n2 4 6 8".to_string());

    assert_eq!(
        sheet,
        Sheet {
            rows: vec![
                Row::new(vec![5, 1, 9, 5]),
                Row::new(vec![7, 5, 3]),
                Row::new(vec![2, 4, 6, 8]),
            ],
        }
    );
}

#[test]
fn sheet_checksum1() {
    let sheet = Sheet::from_string("5 1 9 5\n7 5 3\n2 4 6 8".to_string());

    assert_eq!(18, sheet.checksum1());
}

#[test]
fn sheet_checksum2() {
    let sheet = Sheet::from_string("5 9 2 8\n9 4 7 3\n3 8 6 5".to_string());

    assert_eq!(9, sheet.checksum2());
}

#[test]
fn permutate() {
    assert_eq!(vec![(1, 2), (2, 1)], permutations(&[1, 2]));
    assert_eq!(
        vec![(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)],
        permutations(&[1, 2, 3])
    );
}

#[bench]
fn bench_add_two(b: &mut Bencher) {
    let mut f = File::open(
        "/Users/jgeorge/Workspace/open-source/advent-of-code-2017/day2/benchmark.txt",
    ).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let sheet = Sheet::from_string(contents);

    b.iter(|| sheet.checksum2());
}
