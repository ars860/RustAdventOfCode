



use std::io;
use std::io::{BufRead};
use crate::day17::Cell::{BLOCKED, EMPTY};
use crate::day17::Direction::{DOWN, LEFT, RIGHT, UP};
use std::cmp::max;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
enum Cell {
    EMPTY,
    BLOCKED,
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<Cell>>,
    floor: i64,
    width: usize,
}

impl Board {
    fn new(width: usize) -> Board {
        Board {
            board: vec![vec![BLOCKED; width]; 1],
            floor: 1,
            width,
        }
    }

    fn grow(&mut self, figure: &Figure) {
        if self.board.len() < self.floor as usize + 3 + figure.body.len() {
            self.board.append(&mut vec![vec![EMPTY; self.width]; self.floor as usize + 3 + figure.body.len() - self.board.len()])
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Figure {
    body: Vec<Vec<Cell>>,
    x: i64,
    y: i64,
}

impl Figure {
    fn is_collision(&self, board: &Board) -> bool {
        for y in self.y as usize..self.y as usize + self.body.len() {
            for x in self.x as usize..self.x as usize + self.body[0].len() {
                if self.body[y - self.y as usize][x - self.x as usize] == BLOCKED && board.board[y][x] == BLOCKED {
                    return true;
                }
            }
        }
        false
    }

    fn push(&mut self, board: &Board, dir: Direction) {
        let (old_x, old_y) = (self.x, self.y);
        match dir {
            UP => { self.y += 1 }
            DOWN => { self.y -= 1 }
            LEFT => { self.x -= 1 }
            RIGHT => { self.x += 1 }
        }


        if self.y < 0 || self.x < 0 || self.x as usize + self.body[0].len() > board.width || self.is_collision(board) {
            (self.x, self.y) = (old_x, old_y);
        }
        // else {
        //     board.floor = board.board.len() as i64 - self.y - 1;
        // }
        // dbg!(&self);
    }

    fn stamp(&self, board: &mut Board) {
        for y in self.y as usize..self.y as usize + self.body.len() {
            for x in self.x as usize..self.x as usize + self.body[0].len() {
                if self.body[y - self.y as usize][x - self.x as usize] == BLOCKED {
                    board.board[y][x] = BLOCKED;
                }
            }
        }
    }
}

fn print_board(board: &Board, figure: &Figure) {
    let mut result = board.clone();
    figure.stamp(&mut result);

    for row in result.board.iter().rev() {
        println!("{}", row.iter().map(|&c| if c == EMPTY { "." } else { "#" }).collect::<Vec<_>>().join(""))
    }
}

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();

    let mut wind = String::new();
    stdin.lock().read_line(&mut wind).expect("can't read");
    let wind = wind.chars()
        .filter(|ch| *ch == '<' || *ch == '>')
        .map(|ch| match ch {
            '<' => LEFT,
            '>' => RIGHT,
            ch => {
                dbg!(ch);
                panic!();
            }
        }).collect::<Vec<_>>();

    let mut board = Board::new(7);
    let bodies = vec![
        vec![
            vec![BLOCKED, BLOCKED, BLOCKED, BLOCKED]
        ],
        vec![
            vec![EMPTY, BLOCKED, EMPTY],
            vec![BLOCKED, BLOCKED, BLOCKED],
            vec![EMPTY, BLOCKED, EMPTY],
        ],
        vec![
            vec![BLOCKED, BLOCKED, BLOCKED],
            vec![EMPTY, EMPTY, BLOCKED],
            vec![EMPTY, EMPTY, BLOCKED],
        ],
        vec![
            vec![BLOCKED],
            vec![BLOCKED],
            vec![BLOCKED],
            vec![BLOCKED],
        ],
        vec![
            vec![BLOCKED, BLOCKED],
            vec![BLOCKED, BLOCKED],
        ],
    ];

    let mut iter = 0;
    let mut fall_iter = 0;
    loop {
        let body = bodies[iter % bodies.len()].clone();
        let mut figure = Figure {
            y: -1,
            body,
            x: 2,
        };
        // println!("floor: {}", board.floor);
        board.grow(&figure);
        figure.y = board.floor + 3;
        // print_board(&board, &figure);
        // println!("-------------------------------------");


        loop {
            // println!("{:?}\n{:?}", board, figure);
            figure.push(&mut board, wind[fall_iter % wind.len()]);
            // println!("{:?}", wind[fall_iter % wind.len()]);
            // print_board(&board, &figure);
            // println!("-------------------------------------");
            fall_iter += 1;

            let (x, y) = (figure.x, figure.y);
            figure.push(&mut board, DOWN);
            // print_board(&board, &figure);
            // println!("-------------------------------------");

            if (x, y) == (figure.x, figure.y) {
                figure.stamp(&mut board);
                board.floor = max(board.floor, figure.y + figure.body.len() as i64);
                break;
            }
        }

        iter += 1;

        if iter % 1_000_000 == 0 {
            println!("{}", iter);
        }

        if iter > 1000000000000 {
            // print_board(&board, &figure);
            break;
        }
    }

    println!("{}", board.floor - 1);
}

#[allow(dead_code)]
pub fn solve_part2() {}
