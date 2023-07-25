use std::io;
use std::io::BufRead;
use crate::day14::Cell::{Empty, Sand, Wall};
use std::cmp::{min, max};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Sand,
}

fn drop_sand(pos: (usize, usize), board: &mut Vec<Vec<Cell>>) -> bool {
    let mut pos = pos;
    while pos.1 < board.len() - 1 {
        let (x, y) = pos;

        pos = match (board[x - 1][y + 1], board[x][y + 1], board[x + 1][y + 1]) {
            (_, Empty, _) => (x, y + 1),
            (Empty, _, _) => (x - 1, y + 1),
            (_, _, Empty) => (x + 1, y + 1),
            _ => {
                board[x][y] = Sand;
                return true;
            }
        };
    }
    false
}

const BOARD_SIZE: usize = 1000;
const START_POS: (usize, usize) = (500, 0);

fn parse_input() -> (Vec<Vec<Cell>>, usize) {
    let stdin = io::stdin();
    let mut board = vec![vec![Empty; BOARD_SIZE]; BOARD_SIZE];

    let mut max_y = 0;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            let walls = s.split(" -> ")
                .map(|s| s.split_once(",").unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect::<Vec<_>>();

            for &(_, y) in &walls {
                max_y = max(y, max_y);
            }

            for it in walls.windows(2) {
                if let &[(x1, y1), (x2, y2)] = it {
                    if x1 == x2 {
                        for y in min(y1, y2)..max(y1, y2) + 1 {
                            board[x1][y] = Wall;
                        }
                    } else {
                        for x in min(x1, x2)..max(x1, x2) + 1 {
                            board[x][y1] = Wall;
                        }
                    }
                }
            }
        }
    }
    (board, max_y)
}

#[allow(dead_code)]
pub fn solve_part1() {
    let (mut board, _) = parse_input();

    let mut cnt = 0;
    while drop_sand(START_POS, &mut board) == true {
        cnt += 1;
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
pub fn solve_part2() {
    let (mut board, max_y) = parse_input();

    for x in 0..board[0].len() {
        board[x][max_y + 2] = Wall;
    }

    let mut cnt = 0;
    while board[START_POS.0][START_POS.1] == Empty {
        drop_sand(START_POS, &mut board);
        cnt += 1;
    }

    println!("{}", cnt);
}
