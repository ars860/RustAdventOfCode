use enum_ordinalize::Ordinalize;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Eq, PartialEq, Ordinalize, Debug, Clone, Copy)]
enum Turn {
    A,
    B,
    C,
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Turn::A),
            "B" => Ok(Turn::B),
            "C" => Ok(Turn::C),
            _ => Err(()),
        }
    }
}

fn eval_turn_score(t1: &Turn, t2: &Turn) -> i32 {
    let initial_score = (t1.ordinal() + 1) as i32;

    let result = initial_score
        + match (t1, t2) {
            (Turn::A, Turn::A) => 3,
            (Turn::A, Turn::B) => 0,
            (Turn::A, Turn::C) => 6,
            (Turn::B, Turn::A) => 6,
            (Turn::B, Turn::B) => 3,
            (Turn::B, Turn::C) => 0,
            (Turn::C, Turn::A) => 0,
            (Turn::C, Turn::B) => 6,
            (Turn::C, Turn::C) => 3,
        };

    // println!("turn: ({:?} {:?}), result: {}", t1, t2, result);
    return result;
}

#[derive(Eq, PartialEq, Ordinalize)]
enum MyTurn {
    X,
    Y,
    Z,
}

impl FromStr for MyTurn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MyTurn::X),
            "Y" => Ok(MyTurn::Y),
            "Z" => Ok(MyTurn::Z),
            _ => Err(()),
        }
    }
}

impl MyTurn {
    fn guess(&self, i: i8) -> Turn {
        Turn::from_ordinal((self.ordinal() + i) % 3).unwrap()
    }
}

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut strat_results = [0, 0, 0];
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let [t1, t2] = s.split(" ").collect::<Vec<&str>>()[..2] {
                let (t1, t2) = (Turn::from_str(t1).unwrap(), MyTurn::from_str(t2).unwrap());
                for i in 0..3 {
                    strat_results[i] += eval_turn_score(&t2.guess(i as i8), &t1)
                }
            }
        }
    }

    println!("{}", strat_results.iter().max().unwrap())
}

#[allow(dead_code)]
pub fn solve_part2() {
    let stdin = io::stdin();
    let mut result: i32 = 0;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let [t1, t2] = s.split(" ").collect::<Vec<&str>>()[..2] {
                let (t1, t2) = (Turn::from_str(t1).unwrap(), MyTurn::from_str(t2).unwrap());
                let t2 = match t2 {
                    MyTurn::X => match t1 {
                        Turn::A => Turn::C,
                        Turn::B => Turn::A,
                        Turn::C => Turn::B,
                    },
                    MyTurn::Z => match t1 {
                        Turn::A => Turn::B,
                        Turn::B => Turn::C,
                        Turn::C => Turn::A,
                    },
                    MyTurn::Y => t1,
                };

                result += eval_turn_score(&t2, &t1);
            }
        }
    }

    println!("{}", result)
}
