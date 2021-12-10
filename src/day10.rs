use std::collections::HashMap;

lazy_static! {
    static ref PARTNERS: HashMap<char, char> = hashmap! {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
    };
    static ref SYNTAX_SCORES: HashMap<char, usize> = hashmap! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    };
    static ref COMPLETION_SCORES: HashMap<char, usize> = hashmap! {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
    };
}

fn syntax_score(line: &str) -> Option<usize> {
    let mut stack: Vec<char> = vec![];
    for bracket in line.chars() {
        match PARTNERS.get(&bracket) {
            Some(opener) => {
                if stack.pop() != Some(*opener) {
                    let score = SYNTAX_SCORES[&bracket];
                    return Some(score);
                }
            }
            None => stack.push(bracket),
        }
    }
    None
}

fn completion_score(line: &str) -> Option<usize> {
    let mut stack: Vec<char> = vec![];
    for bracket in line.chars() {
        match PARTNERS.get(&bracket) {
            Some(opener) => {
                if stack.pop() != Some(*opener) {
                    return None;
                }
            }
            None => stack.push(bracket),
        }
    }

    let mut score = 0;
    while let Some(opener) = stack.pop() {
        score *= 5;
        score += COMPLETION_SCORES[&opener]
    }
    Some(score)
}

pub(crate) fn day10() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let part_one: usize = lines.iter().filter_map(|line| syntax_score(line)).sum();
    println!("Part one answer is {}", part_one);

    let mut completion_scores: Vec<usize> = lines
        .iter()
        .filter_map(|line| completion_score(line))
        .collect();
    completion_scores.sort_unstable();
    let part_two = completion_scores[completion_scores.len() / 2];
    println!("Part two answer is {}", part_two);
}
