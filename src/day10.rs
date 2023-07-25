use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut register = 1;
    let mut cycle_cnt = 1;
    let mut result = 0;

    fn next_cycle(cycle_cnt: &mut i32, register: i32) -> i32 {
        *cycle_cnt += 1;
        return if (*cycle_cnt - 20) % 40 == 0 {
            // println!("{}: {}", *cycle_cnt, register);
            *cycle_cnt * register
        } else {
            0
        };
    }

    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            match s.split_once(" ") {
                None => result += next_cycle(&mut cycle_cnt, register),
                Some((_, cnt)) => {
                    let cnt = cnt.parse::<i32>().unwrap();
                    result += next_cycle(&mut cycle_cnt, register);
                    register += cnt;
                    result += next_cycle(&mut cycle_cnt, register);
                }
            }
            // println!("tick: {}, register: {}, result: {}", cycle_cnt, register, result)
        }
    }

    println!("{}", result);
}

#[allow(dead_code)]
pub fn solve_part2() {
    let stdin = io::stdin();
    let mut register = 1;
    let mut cycle_cnt = 1;
    let mut screen = vec![vec![false; 40]; 6];

    fn next_cycle(cycle_cnt: &mut i32, register: i32, screen: &mut Vec<Vec<bool>>) {
        *cycle_cnt += 1;
        let row = (*cycle_cnt - 1) / 40;
        let col = (*cycle_cnt - 1) % 40;

        if (register - col).abs() <= 1 {
            screen[row as usize][col as usize] = true;
        }
    }

    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            match s.split_once(" ") {
                None => next_cycle(&mut cycle_cnt, register, &mut screen),
                Some((_, cnt)) => {
                    let cnt = cnt.parse::<i32>().unwrap();
                    next_cycle(&mut cycle_cnt, register, &mut screen);
                    register += cnt;
                    next_cycle(&mut cycle_cnt, register, &mut screen);
                }
            }
            // println!("tick: {}, register: {}, result: {}", cycle_cnt, register, result)
        }
    }

    println!(
        "{}",
        screen
            .iter()
            .map(|row| row
                .iter()
                .map(|v| if *v { "#" } else { "." })
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
