use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::BufRead;
use std::rc::Rc;
use regex::Regex;

struct Node {
    name: String,
    children: RefCell<HashMap<String, (usize, Rc<Node>)>>,
    flow: RefCell<usize>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Node {{name: {}, flow: {}, children: {}}}",
                             self.name,
                             self.flow.borrow(),
                             self.children.borrow().values()
                                 .map(|(dist, ch)| format!("{{dist: {}, name: {}}}", dist, ch.name))
                                 .collect::<Vec<_>>().join(", ")
        ))
    }
}

impl Node {
    fn empty(name: String) -> Node {
        Node {
            name,
            children: RefCell::new(HashMap::new()),
            flow: RefCell::new(0),
        }
    }

    fn concentrate(&self) {
        println!("concentrate node: {}", self.name);

        let mut visited = HashSet::new();
        let mut next_iter = true;

        while next_iter {
            next_iter = false;

            let mut children_to_add: HashMap<String, (usize, Rc<Node>)> = HashMap::new();
            let mut children_to_remove = HashSet::new();

            for (ch_name, (dist, ch)) in self.children.borrow().iter() {
                if *ch.flow.borrow() == 0 {
                    for (_, (ch_dist, ch_ch)) in ch.children.borrow().iter() {
                        if ch_ch.name != self.name {
                            if let Some((stored_dist, _)) = children_to_add.get(&ch_ch.name) {
                                if *stored_dist > *ch_dist + *dist {
                                    children_to_add.insert(ch_ch.name.to_string(), (*ch_dist + *dist, ch_ch.clone()));
                                }
                            } else {
                                children_to_add.insert(ch_ch.name.to_string(), (*ch_dist + *dist, ch_ch.clone()));
                            }
                        }
                        children_to_remove.insert(ch_name.clone());
                    }
                }
            }

            for i in children_to_remove {
                self.children.borrow_mut().remove(&i);
            }
            for (name, (new_dist, ch)) in children_to_add {
                let need_to_insert =
                    if let Some((dist, _)) = self.children.borrow().get(&name) {
                        *dist > new_dist
                    } else {
                        true
                    };
                if need_to_insert && !visited.contains(&name) {
                    self.children.borrow_mut().insert(name.clone(), (new_dist, ch));
                    visited.insert(name);
                    next_iter = true;
                }
            }
        }
        // self.children.borrow_mut().append(&mut children_to_add.into_values().collect());

        // println!("After concentration: {}", self);
        // if self.children.borrow().values().find(|(_, ch)| ch.name != self.name && *ch.flow.borrow() == 0).is_some() {
        //     self.concentrate();
        // }
    }
}

#[allow(dead_code)]
pub fn solve_part1() {
    let stdin = io::stdin();

    let input_regex = Regex::new(r"Valve ([^ ]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let mut nodes: HashMap<String, Rc<Node>> = HashMap::new();
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            println!("{}", s);
            let groups = input_regex.captures(&s).unwrap();
            let name = &groups[1];
            let flow = groups[2].parse::<usize>().unwrap();
            let children = groups[3].split(", ").collect::<Vec<_>>();

            let children = children.iter()
                .map(|&nm| (nm.to_string(), (1 as usize, nodes.entry(nm.to_string()).or_insert(Rc::new(Node::empty(nm.to_string()))).clone())))
                .collect::<HashMap<_, _>>();

            let node = nodes.entry(name.to_string()).or_insert(Rc::new(Node {
                name: name.to_string(),
                children: RefCell::new(HashMap::new()),
                flow: RefCell::new(0),
            }));
            *node.children.borrow_mut() = children;
            *node.flow.borrow_mut() = flow;
        }
    }

    for node in nodes.values() {
        if *node.flow.borrow() != 0 || node.name == "AA" {
            node.concentrate();
        }
        println!("{}", node);
    }
}

#[allow(dead_code)]
pub fn solve_part2() {}
