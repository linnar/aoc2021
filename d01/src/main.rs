use std::fs;
use std::collections::VecDeque;

fn solve_each(path: &str) -> i32 {
    let mut previous = i64::MAX;
    let mut increases = 0;
    fs::read_to_string(path)
        .expect("File not found")
        .lines()
        .map(|x| x.parse::<i64>().expect("Not a i64 number"))
        .for_each(|number| {
            if number > previous {
                increases += 1
            }
            previous = number
        });
    increases
}

fn solve_window(path: &str) -> i32 {
    let mut q = VecDeque::new();
    q.reserve(4);
    let mut increases = 0;
    fs::read_to_string(path)
        .expect("File not found")
        .lines()
        .map(|x| x.parse::<i64>().expect("Not a i64 number"))
        .for_each(|number| {
            q.push_back(number);
            if q.len() > 3 {
                let removed = q.pop_front().unwrap();
                if number > removed {
                    increases += 1
                }
            }
        });
    increases
}

fn main() {
    println!("{}", solve_each("d01/input/data.txt"));
    println!("{}", solve_window("d01/input/data.txt"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_each() {
        assert_eq!(solve_each("test/test.txt"), 7);
    }

    
    #[test]
    fn test_window() {
        assert_eq!(solve_window("test/test.txt"), 5);
    }
}
