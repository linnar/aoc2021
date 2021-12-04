use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Edge {
    from: Point,
    to: Point,
}

fn solve_first(path: &str) -> (i64, i64) {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);

    let edges: Vec<_> = reader.lines().map(|line| line.unwrap()).map(|line| {
        let (from_str, to_str) = line.split_once(" -> ").unwrap();
        let (fx, fy) = from_str.split_once(",").unwrap();
        let (tx, ty) = to_str.split_once(",").unwrap();
        Edge {
            from: Point { x: fx.parse().unwrap(), y: fy.parse().unwrap() },
            to: Point { x: tx.parse().unwrap(), y: ty.parse().unwrap() },
        }
    }).collect();
    let mut max_x = 0;
    let mut max_y = 0;
    for edge in edges.iter() {
        if edge.from.x > max_x {
            max_x = edge.from.x
        }
        if edge.to.x > max_x {
            max_x = edge.to.x
        }
        if edge.from.y > max_y {
            max_y = edge.from.y
        }
        if edge.to.y > max_y {
            max_y = edge.to.y
        }
    }
    let mut first_map = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    let mut second_map = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    for edge in edges.iter() {
        if edge.from.x == edge.to.x {
            let start = min(edge.from.y, edge.to.y);
            let end = max(edge.from.y, edge.to.y);
            for y in start..=end {
                first_map[y as usize][edge.from.x as usize] += 1;
                second_map[y as usize][edge.from.x as usize] += 1;
            }
        } else if edge.from.y == edge.to.y {
            let start = min(edge.from.x, edge.to.x);
            let end = max(edge.from.x, edge.to.x);
            for x in start..=end {
                first_map[edge.from.y as usize][x as usize] += 1;
                second_map[edge.from.y as usize][x as usize] += 1;
            }
        } else {
            let y_rev = edge.from.y > edge.to.y;
            let x_rev = edge.from.x > edge.to.x;
            if !x_rev && !y_rev {
                (edge.from.y..=edge.to.y).zip(edge.from.x..=edge.to.x).for_each(|(y, x)| {
                    second_map[y as usize][x as usize] += 1;
                });
            } else if !x_rev && y_rev {
                (edge.to.y..=edge.from.y).rev().zip(edge.from.x..=edge.to.x).for_each(|(y, x)| {
                    second_map[y as usize][x as usize] += 1;
                });
            } else if x_rev && !y_rev {
                (edge.from.y..=edge.to.y).zip((edge.to.x..=edge.from.x).rev()).for_each(|(y, x)| {
                    second_map[y as usize][x as usize] += 1;
                });
            } else {
                (edge.to.y..=edge.from.y).rev().zip((edge.to.x..=edge.from.x).rev()).for_each(|(y, x)| {
                    second_map[y as usize][x as usize] += 1;
                });
            };
        }
    }
    let first = first_map.iter().map(|v| v.iter().filter(|&&n| n > 1).count() as i64).sum();
    let second = second_map.iter().map(|v| v.iter().filter(|&&n| n > 1).count() as i64).sum();
    (first, second)
}

fn main() {
    println!("{:?}", solve_first("d05/input/data.txt"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("test/test.txt").0, 5);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_first("test/test.txt").1, 12);
    }
}
