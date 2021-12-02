use std::fs;

fn solve_first(path: &str) -> i64 {
    let mut x = 0;
    let mut depth = 0;

    fs::read_to_string(path)
    .expect("File not found")
    .lines()
    .map(|line| {
        let mut parts = line.split(" ");
        (parts.next().unwrap(), parts.next().unwrap().parse::<i64>().expect("Not a i64 number"))
    }).for_each(|(command, value)| {
            match command {
                "forward" => x += value,
                "down" => depth += value,
                "up" => depth -= value,
                _ => panic!("Unexpected input"),
            }
        });
    x * depth
}

fn solve_second(path: &str) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    fs::read_to_string(path)
    .expect("File not found")
    .lines()
    .map(|line| {
        let mut parts = line.split(" ");
        (parts.next().unwrap(), parts.next().unwrap().parse::<i64>().expect("Not a i64 number"))
    }).for_each(|(command, value)| {
            match command {
                "forward" => {
                    horizontal += value;
                    depth += aim * value;
                },
                "down" => aim += value,
                "up" => aim -= value,
                _ => panic!("Unexpected input"),
            }
        });
    horizontal * depth
}

fn main() {
    println!("{}", solve_first("d02/input/data.txt"));
    println!("{}", solve_second("d02/input/data.txt"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("test/test.txt"), 150);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second("test/test.txt"), 900);
    }

}
