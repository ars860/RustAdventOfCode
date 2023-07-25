use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
enum Directon {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

type Pos = (i32, i32);

impl Directon {
    fn coords(&self) -> Pos {
        match self {
            Directon::UP => (-1, 0),
            Directon::DOWN => (1, 0),
            Directon::LEFT => (0, -1),
            Directon::RIGHT => (0, 1),
        }
    }
}

impl FromStr for Directon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Directon::RIGHT),
            "U" => Ok(Directon::UP),
            "D" => Ok(Directon::DOWN),
            _ => Ok(Directon::LEFT),
        }
    }
}

fn make_move(head_pos: Pos, tail_pos: Pos, direction: &Directon) -> (Pos, Pos) {
    let (head_x, head_y) = head_pos;
    let (move_x, move_y) = direction.coords();
    let head_pos = (head_x + move_x, head_y + move_y);

    return (head_pos, follow_head(head_pos, tail_pos));
}

fn follow_head(head_pos: Pos, tail_pos: Pos) -> Pos {
    let (head_x, head_y) = head_pos;
    // let (mut tail_x, mut tail_y) = tail_pos;
    let (tail_x, tail_y) = tail_pos;
    let is_move_needed = (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1;
    let x_move = if is_move_needed {
        (head_x - tail_x).signum()
    } else {
        0
    };
    let y_move = if is_move_needed {
        (head_y - tail_y).signum()
    } else {
        0
    };

    // if head_x - tail_x > 1 {
    //     tail_x += 1;
    //     // tail_y = head_y;
    // }
    // if tail_x - head_x > 1 {
    //     tail_x -= 1;
    //     // tail_y = head_y;
    // }
    // if head_y - tail_y > 1 {
    //     tail_y += 1;
    //     // tail_x = head_x;
    // }
    // if tail_y - head_y > 1 {
    //     tail_y -= 1;
    //     // tail_x = head_x;
    // }
    return (tail_x + x_move, tail_y + y_move);
}

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    visited[500][500] = true;
    let mut head_pos = (500, 500);
    let mut tail_pos = (500, 500);
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let Some((direction, cnt)) = s.split_once(" ") {
                let cnt = cnt.parse::<i32>().unwrap();
                let direction = Directon::from_str(direction).unwrap();

                for _ in 0..cnt {
                    (head_pos, tail_pos) = make_move(head_pos, tail_pos, &direction);
                    let (tail_x, tail_y) = tail_pos;
                    visited[tail_x as usize][tail_y as usize] = true;
                }
            }
        }
    }

    println!(
        "{}",
        visited
            .iter()
            .map(|row| row.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>()
    );
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<bool>>) {
    for row in board {
        println!(
            "{}",
            row.iter()
                .map(|b| if *b { "#" } else { "." })
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

#[allow(dead_code)]
fn print_snake(board: &Vec<Vec<bool>>, snake: &Vec<(i32, i32)>) {
    let map = snake
        .iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    for (i, row) in board.iter().enumerate() {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(j, _)| if map.contains_key(&(i as i32, j as i32)) {
                    map.get(&(i as i32, j as i32)).unwrap().to_string()
                } else {
                    ".".to_string()
                })
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

#[allow(dead_code)]
pub fn solve_part2() {
    const BOARD_SIZE: usize = 1000;
    const START_POS: i32 = BOARD_SIZE as i32 / 2;

    let stdin = io::stdin();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; BOARD_SIZE]; BOARD_SIZE];
    visited[START_POS as usize][START_POS as usize] = true;
    let mut snake = vec![(START_POS, START_POS); 10];
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let Some((direction, cnt)) = s.split_once(" ") {
                let cnt = cnt.parse::<i32>().unwrap();
                let direction = Directon::from_str(direction).unwrap();

                // dbg!((&direction, &cnt));
                for _ in 0..cnt {
                    for i in 0..snake.len() - 1 {
                        let head_pos = snake[i];
                        let tail_pos = snake[i + 1];

                        if i == 0 {
                            (snake[i], snake[i + 1]) = make_move(head_pos, tail_pos, &direction);
                        } else {
                            snake[i + 1] = follow_head(head_pos, tail_pos);
                        }
                    }

                    let (tail_x, tail_y) = snake[snake.len() - 1];
                    visited[tail_x as usize][tail_y as usize] = true;
                    // print_snake(&visited, &snake);
                }
            }
        }
    }

    // print_board(&visited);

    println!(
        "{}",
        visited
            .iter()
            .map(|row| row.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>())
            .sum::<usize>()
    );
}
