use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Board {
    // number -> (column, row)
    numbers: HashMap<usize, (usize, usize)>,
    // (column, row)
    hits: HashSet<(usize, usize)>,
}

impl Board {
    fn new(numbers: HashMap<usize, (usize, usize)>) -> Self {
        Self {
            numbers,
            hits: HashSet::new(),
        }
    }

    fn see(&mut self, number: usize) {
        if let Some(position) = self.numbers.get(&number) {
            self.hits.insert(*position);
        }
    }

    fn is_winner(&self) -> bool {
        self.is_column_winner() || self.is_row_winner()
    }

    fn is_column_winner(&self) -> bool {
        (0..5).any(|column| (0..5).all(|row| self.hits.contains(&(column, row))))
    }

    fn is_row_winner(&self) -> bool {
        (0..5).any(|row| (0..5).all(|column| self.hits.contains(&(column, row))))
    }

    fn score(&self, last_called: usize) -> usize {
        let unmarked_sum: usize = self
            .numbers
            .iter()
            .filter(|(_number, position)| !self.hits.contains(position))
            .map(|(number, _position)| number)
            .sum();
        last_called * unmarked_sum
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers: HashMap<usize, (usize, usize)> = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (column, word) in line.split_whitespace().enumerate() {
                let number: usize = word.parse()?;
                numbers.insert(number, (column, row));
            }
        }
        let board = Board::new(numbers);
        Ok(board)
    }
}

pub(crate) fn day04() {
    let input = std::fs::read_to_string("data/day04.txt").unwrap();
    let mut lines = input.lines();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|word| word.parse().unwrap())
        .collect();

    let boards: Vec<Board> = lines
        .chunks(6)
        .into_iter()
        .map(|mut chunk| {
            let _blank = chunk.next();
            chunk.join("\n").parse().unwrap()
        })
        .collect();

    let mut part_one_boards = boards.clone();
    'outer1: for number in &numbers {
        for board in &mut part_one_boards {
            board.see(*number);
            if board.is_winner() {
                println!("Part one answer is {}", board.score(*number));
                break 'outer1;
            }
        }
    }

    let mut winners: HashSet<usize> = HashSet::new();
    let mut part_two_boards = boards.clone();
    'outer2: for number in &numbers {
        for (index, board) in part_two_boards.iter_mut().enumerate() {
            if winners.contains(&index) {
                continue;
            }
            board.see(*number);
            if board.is_winner() {
                winners.insert(index);
                if winners.len() == boards.len() {
                    println!("Part two answer is {}", board.score(*number));
                    break 'outer2;
                }
            }
        }
    }
}
