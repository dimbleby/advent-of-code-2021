use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Default, Clone)]
struct PolymerTemplate {
    pairs: HashMap<(char, char), usize>,
    start: char,
    end: char,
}

impl PolymerTemplate {
    fn new(pairs: HashMap<(char, char), usize>, start: char, end: char) -> Self {
        Self { pairs, start, end }
    }

    fn apply_rules(&self, rules: &HashMap<(char, char), char>) -> Self {
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        for (pair, instances) in &self.pairs {
            if let Some(&c3) = rules.get(pair) {
                let (c1, c2) = pair;
                let count = pairs.entry((*c1, c3)).or_insert(0);
                *count += instances;

                let count = pairs.entry((c3, *c2)).or_insert(0);
                *count += instances;
            } else {
                let count = pairs.entry(*pair).or_insert(0);
                *count += instances;
            }
        }
        Self::new(pairs, self.start, self.end)
    }

    fn count_chars(&self) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for ((c1, c2), instances) in &self.pairs {
            let count = counts.entry(*c1).or_insert(0);
            *count += instances;
            let count = counts.entry(*c2).or_insert(0);
            *count += instances;
        }

        // We've counted everything twice, except the start and the end.
        let count = counts.entry(self.start).or_insert(0);
        *count += 1;
        let count = counts.entry(self.end).or_insert(0);
        *count += 1;
        for count in counts.values_mut() {
            *count /= 2
        }
        counts
    }
}

impl FromStr for PolymerTemplate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s.chars().next().ok_or(())?;
        let end = s.chars().last().ok_or(())?;
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        for pair in s.chars().tuple_windows() {
            let count = pairs.entry(pair).or_insert(0);
            *count += 1;
        }
        let template = Self::new(pairs, start, end);
        Ok(template)
    }
}

pub(crate) fn day14() {
    let input = std::fs::read_to_string("data/day14.txt").unwrap();
    let mut lines = input.lines();
    let template: PolymerTemplate = lines.next().unwrap().parse().unwrap();

    let _blank = lines.next().unwrap();

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for line in lines {
        let mut words = line.split_whitespace();
        let pair = words
            .next()
            .unwrap()
            .chars()
            .tuple_windows()
            .next()
            .unwrap();
        let _arrow = words.next().unwrap();
        let out = words.next().unwrap().chars().next().unwrap();
        rules.insert(pair, out);
    }

    let part_one = (0..10).fold(template.clone(), |template, _| template.apply_rules(&rules));
    let counts = part_one.count_chars();
    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();
    println!("Part one answer is {}", max_count - min_count);

    let part_two = (0..40).fold(template, |template, _| template.apply_rules(&rules));
    let counts = part_two.count_chars();
    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();
    println!("Part two answer is {}", max_count - min_count);
}
