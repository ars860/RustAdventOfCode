use std::collections::BTreeSet;
use std::io;

#[allow(dead_code)]
pub fn solve(n: usize) {
    let stdin = io::stdin();
    let mut result = 0;
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    assert!(line.as_bytes().len() > n);
    for i in 0..line.as_bytes().len() - n {
        if BTreeSet::from_iter(line.as_bytes()[i..i + n].iter()).len() == n {
            result = i;
            break;
        }
    }

    println!("{}", result + n);
}

#[allow(dead_code)]
pub fn solve_part1() {
    solve(4);
}

#[allow(dead_code)]
pub fn solve_part2() {
    solve(14);
}
