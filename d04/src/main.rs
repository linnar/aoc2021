use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn board_value(board: &Vec<Vec<i64>>) -> i64 {
    board.iter().map(|row| row.iter().sum::<i64>()).sum()
}

fn is_board_complete(board: &Vec<Vec<i64>>) -> bool {
    if board.iter().any(|row| row.iter().all(|&number| number == 0)) {
        return true
    }
    let len = board.first().unwrap().len();
    for i in 0..len {
        if board
            .iter()
            .map(|x| x[i])
            .all(|number| number == 0) {
            return true
        }
    }
    false
}

fn solve_first(path: &str) -> (i64, i64) {
    let file = File::open(path).expect("File not found");
    let mut reader = BufReader::new(file);

    let mut number_input= String::new();
    reader.read_line(&mut number_input).unwrap();
    let numbers: Vec<i64> = number_input.trim().split(",").map(|n| n.parse::<i64>().unwrap()).collect();

    let mut boards = Vec::<Vec<Vec<i64>>>::new();
    reader.lines().map(|line| line.unwrap()).for_each(|line| {
       if line.is_empty() {
           boards.push(Vec::new());
       } else {
           boards.last_mut().unwrap().push(line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect())
       }
    });

    let mut results: HashMap<usize, i64> = HashMap::new();
    let mut first_win = None;
    let mut last_win = None;

    for number in numbers {
        for (board_num, board) in boards.iter_mut().enumerate() {
            if results.contains_key(&board_num) {
                continue
            }
            let mut changed = None;
            for x in board.iter_mut() {
                if let Some(n) = x.iter_mut().find(|&&mut n| n == number) {
                    *n = 0_i64;
                    changed = Some(number);
                    break;
                }
            }
            if let Some(n) = changed {
                if is_board_complete(board) {
                    let result = board_value(board) * n;
                    results.insert(board_num, result);
                    if first_win == None {
                        first_win = Some(result);
                    }
                    last_win = Some(result);
                }
            }
        }
    }
    (first_win.unwrap(), last_win.unwrap())
}

fn main() {
    println!("{:?}", solve_first("d04/input/data.txt"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("test/test.txt").0, 4512);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_first("test/test.txt").1, 1924);
    }

}
