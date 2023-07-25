use std::io;
use std::io::prelude::*;

fn ch2priority(ch: u8) -> u8 {
    if ch >= 'a' as u8 && ch <= 'z' as u8 {
        return ch - 'a' as u8 + 1;
    }
    return ch - 'A' as u8 + 'z' as u8 - 'a' as u8 + 2;
}

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut result = 0;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            let (l, r) = s.split_at(s.len() / 2);
            let mut letters =
                [false; ('z' as usize) - ('a' as usize) + ('Z' as usize) - ('A' as usize) + 2];
            for ch in l.bytes() {
                let priority = ch2priority(ch) as usize;
                letters[priority - 1] = true;
            }
            for ch in r.bytes() {
                let priority = ch2priority(ch) as usize;
                if letters[priority - 1] {
                    result += priority;
                    letters[priority - 1] = false;
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
    let mut letters = [0; ('z' as usize) - ('a' as usize) + ('Z' as usize) - ('A' as usize) + 2];

    for (i, line) in stdin.lock().lines().enumerate() {
        if let Ok(s) = line {
            if i % 3 == 0 {
                for i in 0..letters.len() {
                    letters[i] = 0;
                }
            }

            for ch in s.bytes() {
                let priority = ch2priority(ch) as usize;

                if letters[priority - 1] == i % 3 {
                    letters[priority - 1] += 1;
                }
            }

            if i % 3 == 2 {
                for range in ['a'..='z', 'A'..='Z'] {
                    for ch in range {
                        let priority = ch2priority(ch as u8) as usize;
                        if letters[priority - 1] == 3 {
                            result += priority;
                        }
                    }
                }
            }
        }
    }

    println!("{}", result);
}
