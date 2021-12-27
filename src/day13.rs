use std::fmt;
use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Default)]
struct DotMap {
    dots: HashSet<(usize, usize)>,
    max_x: usize,
    max_y: usize,
}

fn fold(axis: usize, value: usize) -> usize {
    if value < axis {
        value
    } else {
        2 * axis - value
    }
}

impl DotMap {
    fn insert(&mut self, (x, y): (usize, usize)) {
        self.dots.insert((x, y));
        self.max_x = std::cmp::max(x, self.max_x);
        self.max_y = std::cmp::max(y, self.max_y);
    }

    fn folded(&self, instruction: &FoldInstruction) -> Self {
        let mut folded = Self::default();
        for (x, y) in &self.dots {
            let moved = match instruction.axis {
                Axis::X => (fold(instruction.value, *x), *y),
                Axis::Y => (*x, fold(instruction.value, *y)),
            };
            folded.insert(moved)
        }
        folded
    }
}

impl Display for DotMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.max_y {
            let row: String = (0..=self.max_x)
                .map(|x| {
                    if self.dots.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                })
                .collect();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

struct FoldInstruction {
    axis: Axis,
    value: usize,
}

impl FromStr for FoldInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split('=');
        let axis = match sections.next().ok_or(())?.chars().last() {
            Some('x') => Axis::X,
            Some('y') => Axis::Y,
            _ => return Err(()),
        };
        let value = sections.next().ok_or(())?.parse().map_err(|_| ())?;
        let instruction = Self { axis, value };
        Ok(instruction)
    }
}

pub(crate) fn day13() {
    let mut dot_map = DotMap::default();
    let input = std::fs::read_to_string("data/day13.txt").unwrap();
    let mut lines = input.lines();
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut numbers = line.split(',');
        let x: usize = numbers.next().unwrap().parse().unwrap();
        let y: usize = numbers.next().unwrap().parse().unwrap();
        dot_map.insert((x, y));
    }
    let instructions: Vec<FoldInstruction> = lines.map(|line| line.parse().unwrap()).collect();

    let part_one = dot_map.folded(&instructions[0]);
    println!("Part one answer is {}", part_one.dots.len());

    let part_two = instructions
        .iter()
        .fold(dot_map, |map, instruction| map.folded(instruction));
    println!("Part two answer is\n{}", part_two);
}
