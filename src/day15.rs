use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use rayon::prelude::*;

fn dist((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    return (x1 - x2).abs() + (y1 - y2).abs();
}

fn parse_input() -> Vec<((i64, i64), (i64, i64))> {
    let stdin = io::stdin();

    let mut beacons = vec![];
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            let s = s + ".";
            let parts = s.split(" ").collect::<Vec<_>>();

            fn extract_num(part: &str) -> i64 {
                let tmp = part.split_once("=").unwrap().1;
                tmp[..tmp.len() - 1].parse::<i64>().unwrap()
            }

            let sensor_pos = (extract_num(parts[2]), extract_num(parts[3]));
            let beacon_pos = (extract_num(parts[8]), extract_num(parts[9]));
            beacons.push((beacon_pos, sensor_pos));
        }
    }
    beacons
}

#[allow(dead_code)]
pub fn solve_part1() {
    const LINE_NUM: i64 = 2_000_000;

    let beacons = parse_input();

    let mut things_in_row_cnt = 0;
    beacons.iter().map(|i| i.1).for_each(|(_, sy)| if sy == LINE_NUM { things_in_row_cnt += 1 });
    beacons.iter().map(|i| i.0).collect::<HashSet<_>>().iter().for_each(|&(_, by)| if by == LINE_NUM { things_in_row_cnt += 1 });

    let mut row = vec![false; 10_000_000 as usize];
    row = row.par_iter().enumerate().map(|(i, _)| {
        for &(b, s) in &beacons {
            if dist(s, b) >= dist((i as i64 - 5_000_000, LINE_NUM), s) {
                return true;
            }
        }
        false
    }).collect::<Vec<_>>();

    println!("{}", row.iter().map(|b| *b as usize).sum::<usize>() - things_in_row_cnt)
}

fn outer_border((x, y): (i64, i64), radius: i64) -> Vec<(i64, i64)> {
    let radius = radius + 1;
    let mut results = vec![];
    for i in 1..radius {
        results.push((x - radius + i, y - i));
        results.push((x + radius - i, y - i));
        results.push((x - radius + i, y + i));
        results.push((x + radius - i, y + i));
    }
    results.push((x + radius, y));
    results.push((x - radius, y));
    results.push((x, y + radius));
    results.push((x, y - radius));
    return results;
}

#[allow(dead_code)]
pub fn solve_part2() {
    const MAX_SEARCH_RANGE: usize = 4_000_001;
    let beacons = parse_input();

    let mut candidates = vec![];

    for &(s, b) in &beacons {
        candidates.append(&mut outer_border(b, dist(s, b)));
    }

    let result = candidates.par_iter().map(|&(x, y)| {
        if x < MAX_SEARCH_RANGE as i64 && x >= 0 && y >= 0 && y <= MAX_SEARCH_RANGE as i64 {
            let mut all_far = true;
            for &(b, s) in &beacons {
                if dist(s, b) >= dist((x, y), s) {
                    all_far = false;
                    break;
                }
            }

            if all_far {
                return Some((x, y));
            }
        }
        None
    }).find_any(|x| x.is_some())
        .map(|x| x.unwrap());

    if let Some((x, y)) = result {
        println!("{}, {}", x, y);
        println!("{}", 4_000_000 * x + y);
    }
}
