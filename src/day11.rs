use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::prelude::*;
use std::ops::{Add, DivAssign, Mul, Rem};

#[derive(Clone)]
struct Remainders {
    remainders: HashMap<usize, usize>,
}

impl Remainders {
    fn new(value: usize, dividers: &Vec<usize>) -> Remainders {
        let mut remainders = HashMap::new();
        for div in dividers {
            remainders.insert(*div, value % div);
        }
        return Remainders { remainders };
    }

    fn with_same_dividers(&self, value: usize) -> Remainders {
        let mut remainders = HashMap::new();
        for div in self.remainders.keys() {
            remainders.insert(*div, value % div);
        }
        return Remainders { remainders };
    }
}

impl Display for Remainders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{remainders: {:?}}}", self.remainders))
    }
}

impl Add<usize> for Remainders {
    type Output = Remainders;

    fn add(self, rhs: usize) -> Self::Output {
        let mut remainders = HashMap::new();

        for (div, rem) in self.remainders {
            remainders.insert(div, (rem + rhs) % div);
        }

        return Remainders { remainders };
    }
}

impl Add for Remainders {
    type Output = Remainders;

    fn add(self, rhs: Remainders) -> Self::Output {
        let mut remainders = HashMap::new();

        for (div, rem) in self.remainders {
            remainders.insert(div, (rem + rhs.remainders.get(&div).unwrap()) % div);
        }

        return Remainders { remainders };
    }
}

impl Mul<usize> for Remainders {
    type Output = Remainders;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut remainders = HashMap::new();

        for (div, rem) in self.remainders {
            remainders.insert(div, (rem * rhs) % div);
        }

        return Remainders { remainders };
    }
}

impl Mul for Remainders {
    type Output = Remainders;

    fn mul(self, rhs: Remainders) -> Self::Output {
        let mut remainders = HashMap::new();

        for (div, rem) in self.remainders {
            remainders.insert(div, (rem * rhs.remainders.get(&div).unwrap()) % div);
        }

        return Remainders { remainders };
    }
}

impl DivAssign<usize> for Remainders {
    fn div_assign(&mut self, rhs: usize) {
        // self.remainders.
        self.remainders = self
            .remainders
            .iter()
            .map(|(div, rem)| (*div, (rem / rhs) % div))
            .collect::<HashMap<_, _>>();
        // for (div, rem) in &self.remainders {
        //     self.remainders.insert(*div, (rem / rhs) % div);
        // }
    }
}

impl Rem<usize> for Remainders {
    type Output = usize;

    fn rem(self, rhs: usize) -> Self::Output {
        return *self.remainders.get(&rhs).unwrap();
    }
}

struct Monkey {
    id: usize,
    items: RefCell<Vec<Remainders>>,
    operation: Box<dyn Fn(Remainders) -> Remainders>,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspections_cnt: RefCell<usize>,
}

impl Monkey {
    fn turn(&self, monkeys: &Vec<Monkey>, divide_by_three: bool) {
        for item in self.items.borrow().iter() {
            let mut new_item = (self.operation)(item.clone());
            if divide_by_three {
                new_item /= 3 as usize;
            }

            if new_item.clone() % self.test == 0 {
                monkeys[self.if_true]
                    .items
                    .borrow_mut()
                    .push(new_item.clone());
            } else {
                monkeys[self.if_false]
                    .items
                    .borrow_mut()
                    .push(new_item.clone());
            }
            *self.inspections_cnt.borrow_mut() += 1;
        }
        self.items.borrow_mut().clear();
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{{id: {}, items: {}, test: {}, if_true: {}, if_false: {}, inspections:cnt: {}}}",
            self.id,
            self.items
                .borrow()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.test,
            self.if_true,
            self.if_false,
            self.inspections_cnt.borrow()
        ))
    }
}

fn parse_monkey(s: &str, dividers: &Vec<usize>) -> Monkey {
    fn helper(s: &str, dividers: &Vec<usize>) -> Option<Monkey> {
        let lines = s.split("\n").collect::<Vec<_>>();
        let id = lines[0]
            .split_once(" ")?
            .1
            .replace(":", "")
            .parse::<usize>()
            .ok()?;
        let items = lines[1]
            .split_once(": ")?
            .1
            .split(", ")
            .map(|x| x.parse::<usize>().ok())
            .collect::<Option<Vec<_>>>()?;
        let items = items
            .iter()
            .map(|i| Remainders::new(*i, dividers))
            .collect::<Vec<_>>();
        let op_string = lines[2].split_once(" = ")?.1.split(" ").collect::<Vec<_>>();
        let [left, op, right] = op_string[..] else { todo!() };
        let (left, op, right) = (left.to_string(), op.to_string(), right.to_string());
        let operation = move |x: Remainders| {
            let left = if left == "old" {
                x.clone()
            } else {
                x.with_same_dividers(left.parse::<usize>().unwrap())
            };

            let right = if right == "old" {
                x.clone()
            } else {
                x.with_same_dividers(right.parse::<usize>().unwrap())
            };

            return if op == "+" {
                left + right
            } else {
                left * right
            };
        };
        let test = lines[3].split(" ").last()?.parse::<usize>().ok()?;
        let if_true = lines[4].split(" ").last()?.parse::<usize>().ok()?;
        let if_false = lines[5].split(" ").last()?.parse::<usize>().ok()?;
        return Some(Monkey {
            id,
            items: RefCell::new(items),
            operation: Box::new(operation),
            test,
            if_true,
            if_false,
            inspections_cnt: RefCell::new(0),
        });
    }

    return helper(s, dividers).unwrap();
}

fn parse_dividers(monkeys: &Vec<&str>) -> Vec<usize> {
    fn parse_divider(s: &str) -> Option<usize> {
        let lines = s.split("\n").collect::<Vec<_>>();
        Some(lines[3].split(" ").last()?.parse::<usize>().ok()?)
    }

    return monkeys
        .iter()
        .map(|m| parse_divider(m).unwrap())
        .collect::<Vec<_>>();
}

pub fn solve(round_cnt: usize, divide_by_three: bool) {
    let stdin = io::stdin();
    let mut buf = vec![];
    stdin.lock().read_to_end(&mut buf).unwrap();
    let input = String::from_utf8(buf).unwrap();
    let monkeys = input.split("\n\n").collect::<Vec<_>>();
    let dividers = parse_dividers(&monkeys);
    let monkeys = monkeys
        .iter()
        .map(|x| parse_monkey(x, &dividers))
        .collect::<Vec<_>>();

    for j in 0..round_cnt {
        println!("{}", j);
        for i in 0..monkeys.len() {
            let cur = &monkeys[i];
            cur.turn(&monkeys, divide_by_three);
        }
    }

    println!(
        "{}",
        monkeys
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!(
        "{}",
        monkeys
            .iter()
            .map(|m| *m.inspections_cnt.borrow())
            .collect::<BTreeSet<_>>()
            .iter()
            .rev()
            .take(2)
            .fold(1, |x, y| x * *y)
    )
}

// part1 solution is kinda ruined by part2 solution
// replace Remainders type with usize to get a working part1 solution
#[allow(dead_code)]
pub fn solve_part1() {
    solve(20, true);
}

#[allow(dead_code)]
pub fn solve_part2() {
    solve(10_000, false);
}
