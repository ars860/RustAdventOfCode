use std::collections::BTreeSet;
use std::io;

type Pos = (usize, usize);

fn neighbours(pos: Pos, height_map: &Vec<Vec<i64>>) -> Vec<Pos> {
    let (x, y) = pos;
    let mut result: Vec<Pos> = vec![];
    if x > 0 && height_map[x - 1][y] - height_map[x][y] <= 1 {
        result.push((x - 1, y));
    }
    if x < height_map.len() - 1 && height_map[x + 1][y] - height_map[x][y] <= 1 {
        result.push((x + 1, y));
    }
    if y > 0 && height_map[x][y - 1] - height_map[x][y] <= 1 {
        result.push((x, y - 1));
    }
    if y < height_map[0].len() - 1 && height_map[x][y + 1] - height_map[x][y] <= 1 {
        result.push((x, y + 1));
    }
    result
}

fn dijkstra_heightmap(start_pos: Pos, height_map: &Vec<Vec<i64>>) -> Vec<Vec<usize>> {
    let mut dists = vec![vec![usize::MAX; height_map[0].len()]; height_map.len()];
    let mut queue: BTreeSet<(usize, Pos)> = BTreeSet::new();
    dists[start_pos.0][start_pos.1] = 0;
    queue.insert((0, start_pos));

    while let Some((dist, (x, y))) = queue.pop_first() {
        for (nx, ny) in neighbours((x, y), &height_map) {
            if dist + 1 < dists[nx][ny] {
                queue.remove(&(dists[nx][ny], (nx, ny)));
                queue.insert((dist + 1, (nx, ny)));
                dists[nx][ny] = dist + 1;
            }
        }
    }

    return dists;
}

fn parse_board() -> (Pos, Pos, Vec<Vec<i64>>) {
    let stdin = io::stdin();
    let mut height_map: Vec<Vec<i64>> = vec![];
    let mut start_pos: Option<Pos> = None;
    let mut end_pos: Option<Pos> = None;

    for (x, line) in stdin.lines().enumerate() {
        if let Ok(s) = line {
            height_map.push(
                s.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(y, v)| {
                        return match *v as char {
                            'S' => {
                                start_pos = Some((x, y));
                                'a'
                            }
                            'E' => {
                                end_pos = Some((x, y));
                                'z'
                            }
                            _ => *v as char,
                        } as i64;
                    })
                    .collect(),
            );
        }
    }

    return (start_pos.unwrap(), end_pos.unwrap(), height_map);
}

#[allow(dead_code)]
pub fn solve_part1() {
    let (start_pos, (end_x, end_y), height_map) = parse_board();

    println!(
        "{}",
        dijkstra_heightmap(start_pos, &height_map)[end_x][end_y]
    );
}

#[allow(dead_code)]
pub fn solve_part2() {
    let (_, end_pos, mut height_map) = parse_board();
    height_map
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|v| *v = -*v));

    let dists = dijkstra_heightmap(end_pos, &height_map);
    let mut result = usize::MAX;
    for (x, row) in dists.iter().enumerate() {
        for (y, v) in row.iter().enumerate() {
            if height_map[x][y] == -('a' as i64) {
                result = std::cmp::min(result, *v);
            }
        }
    }

    println!("{}", result);
}
