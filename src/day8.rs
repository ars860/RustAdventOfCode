use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<i8>> = Vec::new();
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            grid.push(
                s.chars()
                    .map(|ch| char::to_digit(ch, 10).unwrap() as i8)
                    .collect(),
            );
        }
    }

    let mut visible = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        let mut max = -1;
        for j in 0..grid[i].len() {
            if grid[i][j] > max {
                visible[i][j] = true;
            }
            max = std::cmp::max(max, grid[i][j]);
        }

        let mut max = -1;
        for j in (0..grid[i].len()).rev() {
            if grid[i][j] > max {
                visible[i][j] = true;
            }
            max = std::cmp::max(max, grid[i][j]);
        }
    }

    for i in 0..grid[0].len() {
        let mut max = -1;
        for j in 0..grid.len() {
            if grid[j][i] > max {
                visible[j][i] = true;
            }
            max = std::cmp::max(max, grid[j][i]);
        }

        let mut max = -1;
        for j in (0..grid.len()).rev() {
            if grid[j][i] > max {
                visible[j][i] = true;
            }
            max = std::cmp::max(max, grid[j][i]);
        }
    }

    println!(
        "{}",
        visible
            .iter()
            .map(|row| row.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>()
    );
}

#[allow(dead_code)]
pub fn solve_part2() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<i8>> = Vec::new();
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            grid.push(
                s.chars()
                    .map(|ch| char::to_digit(ch, 10).unwrap() as i8)
                    .collect(),
            );
        }
    }

    let mut max_score = -1;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut pos: i32 = j as i32 - 1;
            while pos > -1 && grid[i][pos as usize] < grid[i][j] {
                pos -= 1;
            }
            let mut left = j as i32 - pos;
            if pos == -1 {
                left -= 1;
            }

            let mut pos = j as i32 + 1;
            while pos < grid[i].len() as i32 && grid[i][pos as usize] < grid[i][j] {
                pos += 1;
            }
            let mut right = pos - j as i32;
            if pos == grid[i].len() as i32 {
                right -= 1;
            }

            let mut pos = i as i32 - 1;
            while pos > -1 && grid[pos as usize][j] < grid[i][j] {
                pos -= 1;
            }
            let mut top = i as i32 - pos;
            if pos == -1 as i32 {
                top -= 1;
            }

            let mut pos = i as i32 + 1;
            while pos < grid.len() as i32 && grid[pos as usize][j] < grid[i][j] {
                pos += 1;
            }
            let mut bottom = pos - i as i32;
            if pos == grid.len() as i32 {
                bottom -= 1;
            }

            let score = left * right * top * bottom;
            // println!("{}:{} -> {}", i, j, score);
            max_score = std::cmp::max(max_score, score);
        }
    }

    println!("{}", max_score);
}
