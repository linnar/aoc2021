use std::collections::HashMap;
use std::fs;

fn linear_cost(starting: &Vec<i64>, target: i64) -> i64 {
    starting.iter().map(|n| (n - target).abs()).sum()
}

fn cumulative_cost(starting: &Vec<i64>, target: i64) -> i64 {
    starting.iter().map(|n| {
        let dist = (n - target).abs();
        (dist * (dist + 1)) / 2
    }).sum()
}

fn solve_first(path: &str) -> i64 {
    let mut numbers: Vec<_> = fs::read_to_string(path).expect("File not found")
        .trim().split(",").map(|n| n.parse::<i64>().unwrap()).collect();
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    (min..=max).map(|t| linear_cost(&numbers, t)).min().unwrap()
}

fn solve_second(path: &str) -> i64 {
    let mut numbers: Vec<_> = fs::read_to_string(path).expect("File not found")
        .trim().split(",").map(|n| n.parse::<i64>().unwrap()).collect();
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    let sum = numbers.iter().sum::<i64>();
    let mean = sum as f64 / (numbers.len() as f64);
    let mut floor_pivot = mean.floor() as i64;
    let mut ceil_pivot = mean.ceil() as i64;
    if floor_pivot == ceil_pivot {
        ceil_pivot += 1;
    }
    let floor_pivot_cost = cumulative_cost(&numbers, floor_pivot);
    let ceil_pivot_cost = cumulative_cost(&numbers, ceil_pivot);
    let mut pivot = floor_pivot;
    let mut best_cost = floor_pivot_cost;
    if floor_pivot_cost < ceil_pivot_cost || ceil_pivot > max {
        while pivot - 1 >= min {
            let cost = cumulative_cost(&numbers, pivot - 1);
            if cost > best_cost {
                break
            } else {
                pivot -= 1;
                best_cost = cost;
            }
        }
    } else {
        while pivot + 1 <= max {
            let cost = cumulative_cost(&numbers, pivot + 1);
            if cost > best_cost {
                break
            } else {
                pivot += 1;
                best_cost = cost;
            }
        }
    };
    best_cost
}

fn main() {
    println!("{:?}", solve_first("d07/input/data.txt"));
    println!("{:?}", solve_second("d07/input/data.txt"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("test/test.txt"), 37);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second("test/test.txt"), 168);
    }
}
