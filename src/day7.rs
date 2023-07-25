use crate::parser::{combine, end, ident, ignore, or, space, string};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum Command {
    Cd { dir: String },
    Ls,
}

fn parse_command(s: String) -> Option<Command> {
    let parser = combine(
        Box::new(ignore(string("$".to_string()))),
        combine(
            combine(
                ignore(Box::new(space)),
                or(
                    Box::new(string("ls".to_string())),
                    combine(
                        Box::new(string("cd".to_string())),
                        combine(ignore(Box::new(space)), Box::new(ident)),
                    ),
                ),
            ),
            Box::new(end),
        ),
    );

    return if let (Some(vals), _) = parser(s) {
        match &vals[..] {
            [x, dir] if x.to_string() == "cd" => Some(Command::Cd {
                dir: dir.to_string(),
            }),
            [x] if x.to_string() == "ls" => Some(Command::Ls),
            _ => None,
        }
    } else {
        None
    };
}

struct Node {
    name: String,
    children: RefCell<HashMap<String, Rc<Node>>>,
    parent: Weak<Node>,
    size: RefCell<usize>,
}

fn to_string_indent(node: &Node, indent: usize) -> String {
    return "  ".repeat(indent).to_owned()
        + "- "
        + &node.name
        + " "
        + (if node.children.borrow().is_empty() {
            "(file)"
        } else {
            "(dir)"
        })
        + " "
        + "(size="
        + &node.size.borrow().to_string()
        + ")"
        + "\n"
        + &node
            .children
            .borrow()
            .iter()
            .map(|(_, ch)| to_string_indent(ch, indent + 1))
            .fold(String::new(), |a, b| a + &b);
}

// impl ToString for Node {
//     fn to_string(&self) -> String {
//         return to_string_indent(self, 0);
//     }
// }

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*to_string_indent(self, 0))
    }
}

// enum NodeKind {
//     Dir { size: usize },
//     File { size: usize },
// }

fn cd(node: &Rc<Node>, name: String, size: usize) -> Rc<Node> {
    if name == ".." {
        return node.parent.upgrade().unwrap();
    }

    return node
        .children
        .borrow_mut()
        .entry(name.clone())
        .or_insert_with(|| {
            Rc::new(Node {
                name,
                children: RefCell::new(HashMap::new()),
                parent: Rc::downgrade(node),
                size: RefCell::new(size),
            })
        })
        .to_owned();
}

fn propagate_size(node: Rc<Node>) {
    let mut cur = node.parent.upgrade();
    while let Some(p) = cur {
        *p.size.borrow_mut() += *node.size.borrow();
        cur = p.parent.upgrade();
    }
}

fn find_needed(cur: &Rc<Node>) -> usize {
    let mut cur_size = *cur.size.borrow();
    // println!("Node: {} size: {}", cur.name, cur_size);
    if cur_size >= 100_000 || cur.children.borrow().is_empty() {
        cur_size = 0;
    }
    return cur_size
        + cur
            .children
            .borrow()
            .iter()
            .map(|(_, ch)| find_needed(ch))
            .sum::<usize>();
}

fn find_needed2(cur: &Rc<Node>, cmp: usize) -> usize {
    let mut cur_size = *cur.size.borrow();
    // println!("Node: {} size: {}", cur.name, cur_size);
    if cur_size < cmp || cur.children.borrow().is_empty() {
        cur_size = usize::MAX;
    }
    return min(
        cur_size,
        cur.children
            .borrow()
            .iter()
            .map(|(_, ch)| find_needed2(ch, cmp))
            .min()
            .unwrap_or(usize::MAX),
    );
}

fn process_cmd(
    cmd: Command,
    cmd_lines: &Vec<String>,
    current_node: Rc<Node>,
    root_node: &Rc<Node>,
) -> Rc<Node> {
    return match cmd {
        Command::Cd { dir } => {
            if dir == "/" {
                root_node.clone()
            } else {
                cd(&current_node, dir, 0)
            }
        }
        Command::Ls => {
            for cmd_line in cmd_lines {
                match cmd_line.split_once(" ") {
                    Some(("dir", dir_name)) => {
                        cd(&current_node, dir_name.to_string(), 0);
                    }
                    Some((file_size, file_name)) => {
                        let file_node = cd(
                            &current_node,
                            file_name.to_string(),
                            file_size.parse().unwrap(),
                        );
                        propagate_size(file_node);
                    }
                    None => panic!(),
                }
            }
            current_node
        }
    };
}

fn build_tree() -> Rc<Node> {
    let stdin = io::stdin();
    let mut prev_cmd: Option<Command> = None;
    let mut cmd_lines: Vec<String> = vec![];
    let root = Rc::new(Node {
        name: "/".to_string(),
        children: RefCell::new(HashMap::new()),
        parent: Weak::new(),
        size: RefCell::new(0),
    });
    let mut current_node = root.clone();

    for line in stdin.lines() {
        if let Ok(s) = line {
            if let Some(cmd) = parse_command(s.clone()) {
                if let Some(c) = prev_cmd {
                    current_node = process_cmd(c, &cmd_lines, current_node, &root);
                    cmd_lines.clear();
                }
                prev_cmd = Some(cmd);
            } else {
                // println!("{}", s);
                cmd_lines.push(s);
            }
        }
    }
    process_cmd(prev_cmd.unwrap(), &cmd_lines, current_node, &root);

    println!("{}", root);
    root
}

#[allow(dead_code)]
pub fn solve_part1() {
    let root = build_tree();
    println!("{}", find_needed(&root));
}

#[allow(dead_code)]
pub fn solve_part2() {
    let root = build_tree();
    println!(
        "{}",
        find_needed2(&root, 30_000_000 - (70_000_000 - *root.size.borrow()))
    );
}
