use std::collections::HashMap;
use std::fs;

fn solve_first(path: &str) -> u64 {
    let days = 80;
    let cycle = 7;
    let mut numbers: Vec<_> = fs::read_to_string(path).expect("File not found")
        .split(",").map(|n| n.parse::<i64>().unwrap()).collect();
    for _ in 0..days {
        let mut updated = Vec::with_capacity(numbers.len());
        let mut new = Vec::new();
        for fish in numbers.iter() {
            let updated_fish = fish - 1;
            if updated_fish < 0 {
                updated.push(updated_fish + cycle);
                new.push(cycle + 1);
            } else {
                updated.push(updated_fish);
            }
        }
        updated.append(&mut new);
        numbers = updated;
    }
    numbers.len() as u64
}

fn solve_second(path: &str) -> u64 {
    let days = 256;
    let cycle = 7;
    let numbers: Vec<_> = fs::read_to_string(path).expect("File not found")
        .split(",").map(|n| n.parse::<i64>().unwrap()).collect();
    let mut current_gen: HashMap<i64, u64> = HashMap::new();
    numbers.iter().for_each(|&n| *current_gen.entry(n).or_insert(0) += 1);
    for _ in 0..days {
        let mut next_gen = HashMap::with_capacity(current_gen.len());
        for (&gen, &count) in current_gen.iter() {
            let update_gen = gen - 1;
            if update_gen < 0 {
                *next_gen.entry(update_gen + cycle).or_insert(0) += count;
                *next_gen.entry(cycle + 1).or_insert(0) += count;
            } else {
                *next_gen.entry(update_gen).or_insert(0) += count;
            }
        }
        current_gen = next_gen;
    }
    current_gen.values().sum::<u64>()
}

fn main() {
    println!("{:?}", solve_first("d06/input/data.txt"));
    println!("{:?}", solve_second("d06/input/data.txt"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("test/test.txt"), 5934);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second("test/test.txt"), 26984457539);
    }
}
