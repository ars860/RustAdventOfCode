use crate::parser::*;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;

// impl CrateParser {
fn parse_crate(row: String) -> (Option<Vec<String>>, String) {
    let bytes = row.as_bytes();
    if bytes.len() >= 3 && bytes[0] as char == '[' && bytes[2] as char == ']' {
        // println!("crate");
        return (
            Some(vec![row[1..2].to_string()]),
            if row.len() > 3 {
                row[3..].to_string()
            } else {
                "".to_string()
            },
        );
    }
    return (None, row);
}

fn parse_triple_space(row: String) -> (Option<Vec<String>>, String) {
    let bytes = row.as_bytes();
    if bytes.len() >= 3
        && bytes[0] as char == ' '
        && bytes[1] as char == ' '
        && bytes[2] as char == ' '
    {
        return (Some(vec!["   ".to_string()]), row[3..].to_string());
    }
    return (None, row);
}

fn parse_row(row: String) -> Vec<String> {
    let test = or(
        combine(Box::new(parse_crate), Box::new(space)),
        combine(Box::new(parse_triple_space), Box::new(space)),
    );
    let (result, _) = many(Rc::from(test))(row);
    if let Some(result) = result {
        return result;
    } else {
        panic!("Can't parse row");
    }
}
// }

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();
    let mut crates: Vec<Vec<char>> = vec![];
    let mut read_commands = false;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if s.starts_with(" 1 ") {
                read_commands = true;
                crates.iter_mut().for_each(|c| c.reverse());
                // dbg!(&crates);
            }

            if !read_commands {
                let row = parse_row(s + " ");
                let row = row.iter().filter(move |c| *c != " ").collect::<Vec<_>>();
                crates.resize(row.len(), vec![]);
                for (i, c) in row.iter().enumerate() {
                    if c.len() == 1 {
                        crates[i].push(c.as_bytes()[0] as char)
                    }
                }
            } else {
                if s.is_empty() {
                    continue;
                }

                if let [_, cnt, _, from, _, to] = s.split(" ").collect::<Vec<_>>()[..] {
                    let cnt = cnt.parse::<usize>().unwrap();
                    let from = from.parse::<usize>().unwrap() - 1;
                    let to = to.parse::<usize>().unwrap() - 1;

                    for _ in 0..cnt {
                        let ch = crates[from].pop().unwrap();
                        crates[to].push(ch);
                    }
                    // println!("moved {} from {} to {}", cnt, from, to);
                    // dbg!(&crates);
                }
            }
        }
    }

    println!(
        "{}",
        crates.iter().map(|c| c.last().unwrap()).collect::<String>()
    );
}

#[allow(dead_code)]
pub fn solve_part2() {
    let stdin = io::stdin();
    let mut crates: Vec<Vec<char>> = vec![];
    let mut read_commands = false;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if s.starts_with(" 1 ") {
                read_commands = true;
                crates.iter_mut().for_each(|c| c.reverse());
                // dbg!(&crates);
            }

            if !read_commands {
                let row = parse_row(s + " ");
                let row = row.iter().filter(move |c| *c != " ").collect::<Vec<_>>();
                crates.resize(row.len(), vec![]);
                for (i, c) in row.iter().enumerate() {
                    if c.len() == 1 {
                        crates[i].push(c.as_bytes()[0] as char)
                    }
                }
            } else {
                if s.is_empty() {
                    continue;
                }

                if let [_, cnt, _, from, _, to] = s.split(" ").collect::<Vec<_>>()[..] {
                    let cnt = cnt.parse::<usize>().unwrap();
                    let from = from.parse::<usize>().unwrap() - 1;
                    let to = to.parse::<usize>().unwrap() - 1;

                    let mut itermediate = vec![];

                    for _ in 0..cnt {
                        let ch = crates[from].pop().unwrap();
                        itermediate.push(ch);
                    }

                    itermediate.reverse();
                    crates[to].append(&mut itermediate);
                    // println!("moved {} from {} to {}", cnt, from, to);
                    // dbg!(&crates);
                }
            }
        }
    }

    println!(
        "{}",
        crates.iter().map(|c| c.last().unwrap()).collect::<String>()
    );
}
