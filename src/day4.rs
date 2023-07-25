use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut result = 0;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let [l, r] = s.split(',').collect::<Vec<_>>()[..2] {
                if let [l1, r1] = l
                    .split('-')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()[..2]
                {
                    if let [l2, r2] = r
                        .split('-')
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()[..2]
                    {
                        if (l1 <= l2 && l2 <= r1) || (l2 <= l1 && l1 <= r2) {
                            result += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", result);
}

#[allow(dead_code)]
pub fn solve_part2() {
    let stdin = io::stdin();
    let mut result = 0;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let [l, r] = s.split(',').collect::<Vec<_>>()[..2] {
                if let [l1, r1] = l
                    .split('-')
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()[..2]
                {
                    if let [l2, r2] = r
                        .split('-')
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()[..2]
                    {
                        if (l1 <= l2 && l2 <= r1)
                            || (l1 <= r2 && r2 <= r1)
                            || (l2 <= r1 && r1 <= r2)
                            || (l2 <= l1 && l1 <= r2)
                        {
                            result += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", result);
}
