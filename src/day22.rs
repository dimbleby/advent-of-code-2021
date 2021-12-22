use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Action {
    On,
    Off,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "on" => Action::On,
            "off" => Action::Off,
            _ => return Err(()),
        };
        Ok(action)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Cuboid {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Cuboid {
    fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64, min_z: i64, max_z: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    fn size(&self) -> i64 {
        (self.max_x + 1 - self.min_x)
            * (self.max_y + 1 - self.min_y)
            * (self.max_z + 1 - self.min_z)
    }

    fn size_part_one(&self) -> i64 {
        let max_x = self.max_x.min(50);
        let max_y = self.max_y.min(50);
        let max_z = self.max_z.min(50);
        let min_x = self.min_x.max(-50);
        let min_y = self.min_y.max(-50);
        let min_z = self.min_z.max(-50);
        (max_x + 1 - min_x).max(0) * (max_y + 1 - min_y).max(0) * (max_z + 1 - min_z).max(0)
    }

    fn intersect(&self, other: &Cuboid) -> Option<Self> {
        if self.max_x < other.min_x
            || other.max_x < self.min_x
            || self.max_y < other.min_y
            || other.max_y < self.min_y
            || self.max_z < other.min_z
            || other.max_z < self.min_z
        {
            return None;
        }

        let intersection = Self::new(
            self.min_x.max(other.min_x),
            self.max_x.min(other.max_x),
            self.min_y.max(other.min_y),
            self.max_y.min(other.max_y),
            self.min_z.max(other.min_z),
            self.max_z.min(other.max_z),
        );
        Some(intersection)
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // yuk.
        let mut ranges = s.split(',');
        let mut xrange = ranges.next().ok_or(())?[2..].split("..");
        let min_x = xrange.next().ok_or(())?.parse().map_err(|_| ())?;
        let max_x = xrange.next().ok_or(())?.parse().map_err(|_| ())?;
        let mut yrange = ranges.next().ok_or(())?[2..].split("..");
        let min_y = yrange.next().ok_or(())?.parse().map_err(|_| ())?;
        let max_y = yrange.next().ok_or(())?.parse().map_err(|_| ())?;
        let mut zrange = ranges.next().ok_or(())?[2..].split("..");
        let min_z = zrange.next().ok_or(())?.parse().map_err(|_| ())?;
        let max_z = zrange.next().ok_or(())?.parse().map_err(|_| ())?;

        let cuboid = Cuboid::new(min_x, max_x, min_y, max_y, min_z, max_z);
        Ok(cuboid)
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    cuboid: Cuboid,
}

impl Instruction {
    fn new(action: Action, cuboid: Cuboid) -> Self {
        Self { action, cuboid }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let action = words.next().ok_or(())?.parse()?;
        let cuboid = words.next().ok_or(())?.parse()?;

        let instruction = Instruction::new(action, cuboid);
        Ok(instruction)
    }
}

pub(crate) fn day22() {
    let input = std::fs::read_to_string("data/day22.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut contributions: HashMap<Cuboid, i64> = HashMap::default();
    for instruction in &instructions {
        let mut intersections: HashMap<Cuboid, i64> = HashMap::default();
        for (cuboid, value) in &contributions {
            if let Some(intersection) = cuboid.intersect(&instruction.cuboid) {
                // We're either undoing or double-counting an existing contribution.
                *intersections.entry(intersection).or_default() -= value;
            }
        }
        for (intersection, value) in intersections {
            *contributions.entry(intersection).or_default() += value;
        }
        if instruction.action == Action::On {
            *contributions.entry(instruction.cuboid).or_default() += 1;
        }
        contributions.retain(|_, v| *v != 0);
    }

    let part_one: i64 = contributions
        .iter()
        .map(|(cuboid, value)| cuboid.size_part_one() * value)
        .sum();
    println!("Part two answer is {}", part_one);

    let part_two: i64 = contributions
        .iter()
        .map(|(cuboid, value)| cuboid.size() * value)
        .sum();
    println!("Part two answer is {}", part_two);
}
