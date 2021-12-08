use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Note {
    // NB we alphabetize the patterns and output during parsing.
    patterns: [String; 10],
    output: [String; 4],
}

impl Note {
    fn new(patterns: [String; 10], output: [String; 4]) -> Self {
        Self { patterns, output }
    }

    fn solve(&self) -> usize {
        let mut numbers: HashMap<&str, usize> = HashMap::new();

        // Number of segments used for each digit are: 6, 2, 5, 5, 4, 5, 6, 3, 7, 6.
        //
        // 1, 4, 7, 8 are easy (lengths 2, 4, 3, 7)
        //
        // So we're going to need to disambiguate:
        // - 0, 6, 9 (length 6)
        // - 2, 3, 5 (length 5)
        //
        // - 9 contains 4, 0 contains 1, 6 doesn't.
        // - 3 contains 1, 2 has two segments in common with 4, 5 doesn't.
        //
        // First pass: pick off the easy numbers (and remember about 1 and 4).
        let mut one: HashSet<char> = HashSet::new();
        let mut four: HashSet<char> = HashSet::new();
        for pattern in &self.patterns {
            if pattern.len() == 2 {
                one = pattern.chars().collect();
                numbers.insert(pattern, 1);
            } else if pattern.len() == 4 {
                four = pattern.chars().collect();
                numbers.insert(pattern, 4);
            } else if pattern.len() == 3 {
                numbers.insert(pattern, 7);
            } else if pattern.len() == 7 {
                numbers.insert(pattern, 8);
            }
        }

        // Second pass: use the easy numbers to solve the tricky numbers.
        for pattern in &self.patterns {
            if pattern.len() == 6 {
                let chars: HashSet<_> = pattern.chars().collect();
                if four.is_subset(&chars) {
                    numbers.insert(pattern, 9);
                } else if one.is_subset(&chars) {
                    numbers.insert(pattern, 0);
                } else {
                    numbers.insert(pattern, 6);
                }
            } else if pattern.len() == 5 {
                let chars: HashSet<_> = pattern.chars().collect();
                if one.is_subset(&chars) {
                    numbers.insert(pattern, 3);
                } else if four.intersection(&chars).count() == 2 {
                    numbers.insert(pattern, 2);
                } else {
                    numbers.insert(pattern, 5);
                }
            }
        }

        let mut solution = 0;
        for digit in &self.output {
            solution *= 10;
            solution += numbers[digit.as_str()];
        }
        solution
    }
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Note {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns: [String; 10] = Default::default();
        let mut output: [String; 4] = Default::default();

        let mut sections = s.split('|');
        let inputs = sections.next().ok_or(ParseError)?;
        for (index, word) in inputs.split_whitespace().enumerate() {
            let mut letters = word.chars().collect::<Vec<_>>();
            letters.sort_unstable();
            patterns[index] = letters.into_iter().collect();
        }
        let outputs = sections.next().ok_or(ParseError)?;
        for (index, word) in outputs.split_whitespace().enumerate() {
            let mut letters = word.chars().collect::<Vec<_>>();
            letters.sort_unstable();
            output[index] = letters.into_iter().collect();
        }

        let note = Note::new(patterns, output);
        Ok(note)
    }
}

pub(crate) fn day08() {
    let notes: Vec<Note> = std::fs::read_to_string("data/day08.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let easy: usize = notes
        .iter()
        .map(|note| {
            note.output
                .iter()
                .filter(|&digit| {
                    let segments = digit.len();
                    segments == 2 || segments == 3 || segments == 4 || segments == 7
                })
                .count()
        })
        .sum();
    println!("Part one answer is {}", easy);

    let total: usize = notes.iter().map(|note| note.solve()).sum();
    println!("Part two answer is {}", total);
}
