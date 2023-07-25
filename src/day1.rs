use std::cmp::max;
use std::collections::BTreeSet;
use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut sum = 0;
    let mut max_sum = -1;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            match s.as_str() {
                "" => {
                    max_sum = max(max_sum, sum);
                    sum = 0;
                }
                cnt => sum += cnt.parse::<i32>().unwrap(),
            }
        }
    }

    max_sum = max(max_sum, sum);

    println!("{}", max_sum);
}

#[allow(dead_code)]
pub fn solve_part2() {
    let stdin = io::stdin();
    let mut sum = 0;
    let mut top3 = BTreeSet::new();
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            match s.as_str() {
                "" => {
                    top3.insert(sum);
                    if top3.len() > 3 {
                        top3.pop_first();
                    }
                    sum = 0;
                }
                cnt => sum += cnt.parse::<i32>().unwrap(),
            }
        }
    }

    top3.insert(sum);
    if top3.len() > 3 {
        top3.pop_first();
    }

    println!("{}", top3.iter().sum::<i32>());
}
