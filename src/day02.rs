use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => return Err(ParseError),
        };
        Ok(direction)
    }
}

#[derive(Debug)]
struct Instruction(Direction, usize);

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let direction: Direction = words.next().ok_or(ParseError).and_then(|w| w.parse())?;

        let distance: usize = words
            .next()
            .and_then(|w| w.parse().ok())
            .ok_or(ParseError)?;

        Ok(Instruction(direction, distance))
    }
}

// (position, depth)
fn move_submarine_part_one(from: &(usize, usize), instruction: &Instruction) -> (usize, usize) {
    match instruction {
        Instruction(Direction::Forward, distance) => (from.0 + distance, from.1),
        Instruction(Direction::Down, distance) => (from.0, from.1 + distance),
        Instruction(Direction::Up, distance) => (from.0, from.1 - distance),
    }
}

// (position, depth, aim)
fn move_submarine_part_two(
    from: &(usize, usize, usize),
    instruction: &Instruction,
) -> (usize, usize, usize) {
    match instruction {
        Instruction(Direction::Forward, distance) => {
            (from.0 + distance, from.1 + (distance * from.2), from.2)
        }
        Instruction(Direction::Down, distance) => (from.0, from.1, from.2 + distance),
        Instruction(Direction::Up, distance) => (from.0, from.1, from.2 - distance),
    }
}

pub(crate) fn day02() {
    let input = std::fs::read_to_string("data/day02.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let finish = instructions.iter().fold((0, 0), |position, instruction| {
        move_submarine_part_one(&position, instruction)
    });
    println!("Part one answer is {}", finish.0 * finish.1);

    let finish = instructions
        .iter()
        .fold((0, 0, 0), |position, instruction| {
            move_submarine_part_two(&position, instruction)
        });
    println!("Part two answer is {}", finish.0 * finish.1);
}
