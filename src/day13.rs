use crate::day13::Either::{Left, Right};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::Read;
use std::iter::once;
use std::str::FromStr;

#[derive(Debug)]
enum Either<A, B> {
    Left(A),
    Right(B),
}

#[derive(Debug)]
struct List<T> {
    elements: Vec<Either<List<T>, T>>,
}

impl FromStr for List<i64> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_list(s).0)
    }
}

impl<T> Display for List<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, el) in self.elements.iter().enumerate() {
            match el {
                Left(list) => list.fmt(f),
                Right(value) => value.fmt(f),
            }?;
            if i != self.elements.len() - 1 {
                f.write_str(",")?;
            }
        }
        f.write_str("]")
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { elements: vec![] }
    }

    fn singleton(val: T) -> List<T> {
        List {
            elements: vec![Right(val)],
        }
    }

    fn push(&mut self, el: Either<List<T>, T>) {
        self.elements.push(el)
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T: Ord + Clone> Eq for List<T> {}

impl<T: Ord + Clone> PartialEq<Self> for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl<T: Ord + Clone> PartialOrd<Self> for List<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Clone> Ord for List<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (el1, el2) in self.elements.iter().zip(other.elements.iter()) {
            let ordering = match (el1, el2) {
                (Left(l1), Left(l2)) => l1.cmp(l2),
                (Left(l1), Right(v2)) => l1.cmp(&List::singleton(v2.clone())),
                (Right(v1), Left(l2)) => List::singleton(v1.clone()).cmp(l2),
                (Right(v1), Right(v2)) => v1.cmp(v2),
            };

            if ordering != Equal {
                return ordering;
            }
        }

        return if self.len() < other.len() {
            Less
        } else if self.len() == other.len() {
            Equal
        } else {
            Greater
        };
    }
}

fn parse_list(s: &str) -> (List<i64>, usize) {
    let s = s.strip_prefix("[").unwrap();
    let mut pos = 0;
    let mut result = List::new();
    let mut current_number: Option<i64> = None;

    while pos < s.as_bytes().len() {
        let ch = s.as_bytes()[pos] as char;
        match ch {
            '[' => {
                let (res, end_pos) = parse_list(&s[pos..]);
                result.push(Left(res));
                pos += end_pos + 1;
            }
            ']' => {
                if let Some(num) = current_number {
                    result.push(Right(num));
                }

                return (result, pos + 1);
            }
            ch if ch as u8 >= '0' as u8 && ch as u8 <= '9' as u8 => {
                let current_diggit = ch.to_digit(10).unwrap() as i64;
                if let Some(num) = current_number {
                    current_number = Some(num * 10 + current_diggit);
                } else {
                    current_number = Some(current_diggit);
                }
                pos += 1;
            }
            ',' => {
                if let Some(num) = current_number {
                    result.push(Right(num));
                    current_number = None;
                }

                pos += 1;
            }
            ' ' => pos += 1,
            _ => panic!(),
        }
    }

    (result, s.as_bytes().len() - 1)
}

fn parse_input() -> Vec<(List<i64>, List<i64>)> {
    let stdin = io::stdin();

    let mut input: String = String::new();
    stdin
        .lock()
        .read_to_string(&mut input)
        .expect("Can't read input");
    input
        .split("\n\n")
        .map(|chunk| chunk.split_once("\n").unwrap())
        .map(|(s1, s2)| (parse_list(s1).0, parse_list(s2).0))
        .collect()
}

#[allow(dead_code)]
pub fn solve_part1() {
    let input = parse_input();

    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .map(|(i, (l1, l2))| if l1 < l2 { i + 1 } else { 0 })
            .sum::<usize>()
    );
}

#[allow(dead_code)]
pub fn solve_part2() {
    let input = parse_input();
    let mut lists: Vec<_> = input.iter()
        .flat_map(|(a, b)| once(a).chain(once(b)))
        .collect();

    let start_marker = "[[2]]".parse().unwrap();
    let end_marker = "[[6]]".parse().unwrap();

    lists.push(&start_marker);
    lists.push(&end_marker);

    lists.sort();

    let start_pos = lists.iter().position(|x| **x == start_marker).unwrap() + 1;
    let end_pos = lists.iter().position(|x| **x == end_marker).unwrap() + 1;

    println!("{}", start_pos * end_pos);
}

#[cfg(test)]
mod tests {
    use crate::day13::{parse_list, List};
    use std::cmp::Ordering::Less;

    #[test]
    fn parse_test() {
        let result = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<List<i64>>().unwrap();
        assert_eq!(result.to_string(), "[1,[2,[3,[4,[5,6,7]]]],8,9]");
    }

    #[test]
    fn parse_test1() {
        let str = "[[[6,[9,5,0,7,9],[3,6,7,7,3],6,[5,8,5,8]],5,6,10,[[9,1,6,1],[2,0,9,2,3],8,5,[4]]],[2,[[1],2,0,[7,4,9,9,9]],[5,2,0]],[[[],[4,7,1,7,3],1,10]],[[4,[9,4,3,8,5],2],2,1,4],[7,7,5,8]]";
        let result = str.parse::<List<i64>>().unwrap();
        assert_eq!(result.to_string(), str);
    }

    fn is_less(l1: &str, l2: &str) {
        let left = l1.parse::<List<i64>>().unwrap();
        let right = l2.parse::<List<i64>>().unwrap();
        assert_eq!(left < right, true);
    }

    #[test]
    fn test8() {
        is_less("[1,[2,[3,[4,[5,6,0]]]],8,9]", "[1,[2,[3,[4,[5,6,7]]]],8,9]");
    }

    #[test]
    fn test7() {
        is_less("[[]]", "[[[]]]");
    }

    #[test]
    fn test4() {
        is_less("[[4,4],4,4]", "[[4,4],4,4,4]");
    }

    #[test]
    fn test2() {
        is_less("[[1],[2,3,4]]", "[[1],4]");
    }
}