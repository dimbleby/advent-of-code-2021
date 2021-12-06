use std::collections::HashMap;
use std::str::FromStr;

struct Stepper {
    next: isize,
    step: isize,
}

impl Stepper {
    fn new(start: isize, step: isize) -> Self {
        Self { next: start, step }
    }
}

impl Iterator for Stepper {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.next;
        self.next += self.step;
        Some(value)
    }
}

#[derive(Debug)]
struct ParseError;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Point(isize, isize);

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(',');
        let x = words
            .next()
            .and_then(|w| w.parse().ok())
            .ok_or(ParseError)?;
        let y = words
            .next()
            .and_then(|w| w.parse().ok())
            .ok_or(ParseError)?;
        let point = Point(x, y);
        Ok(point)
    }
}

#[derive(Debug)]
struct Line {
    end1: Point,
    end2: Point,
}

impl Line {
    fn new(end1: Point, end2: Point) -> Self {
        Self { end1, end2 }
    }

    fn is_vertical(&self) -> bool {
        self.end1.0 == self.end2.0
    }

    fn is_horizontal(&self) -> bool {
        self.end1.1 == self.end2.1
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let xs = Stepper::new(self.end1.0, (self.end2.0 - self.end1.0).signum());
        let ys = Stepper::new(self.end1.1, (self.end2.1 - self.end1.1).signum());
        xs.zip(ys)
            .map(|(x, y)| Point(x, y))
            .take_while(|p| p != &self.end2)
            .chain(std::iter::once(self.end2))
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let end1: Point = words.next().ok_or(ParseError).and_then(|w| w.parse())?;
        let _arrow = words.next();
        let end2: Point = words.next().ok_or(ParseError).and_then(|w| w.parse())?;
        let line = Line::new(end1, end2);
        Ok(line)
    }
}

pub(crate) fn day05() {
    let input = std::fs::read_to_string("data/day05.txt").unwrap();
    let (rectilinear, diagonals): (Vec<Line>, Vec<Line>) = input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .partition(|line| line.is_vertical() || line.is_horizontal());

    let mut grid: HashMap<Point, usize> = HashMap::new();
    for line in &rectilinear {
        for point in line.points() {
            let counter = grid.entry(point).or_insert(0);
            *counter += 1;
        }
    }
    let count = grid.values().filter(|&&count| count > 1).count();
    println!("Part one answer is {}", count);

    for line in &diagonals {
        for point in line.points() {
            let counter = grid.entry(point).or_insert(0);
            *counter += 1;
        }
    }
    let count = grid.values().filter(|&&count| count > 1).count();
    println!("Part two answer is {}", count);
}
