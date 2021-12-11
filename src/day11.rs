use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
struct EnergyMap {
    levels: HashMap<(isize, isize), u32>,
}

impl FromStr for EnergyMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels: HashMap<(isize, isize), u32> = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, number) in line.chars().enumerate() {
                let energy = number.to_digit(10).ok_or(())?;
                levels.insert((x as isize, y as isize), energy);
            }
        }

        let energy_map = Self::new(levels);
        Ok(energy_map)
    }
}

fn neighbours((x, y): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    itertools::iproduct!((-1..=1), (-1..=1))
        .filter(|shift| shift != &(0, 0))
        .map(move |(dx, dy)| (x + dx, y + dy))
}

impl EnergyMap {
    fn new(levels: HashMap<(isize, isize), u32>) -> Self {
        Self { levels }
    }

    fn step(&mut self) -> usize {
        let mut flashers: Vec<(isize, isize)> = vec![];
        let mut pending: Vec<(isize, isize)> = vec![];

        for (position, level) in self.levels.iter_mut() {
            *level += 1;
            if *level == 10 {
                pending.push(*position)
            }
        }

        while let Some(flasher) = pending.pop() {
            flashers.push(flasher);

            for neighbour in neighbours(flasher) {
                if let Some(neighbour_level) = self.levels.get_mut(&neighbour) {
                    *neighbour_level += 1;
                    if *neighbour_level == 10 {
                        pending.push(neighbour);
                    }
                }
            }
        }

        let count = flashers.len();
        self.levels
            .extend(flashers.into_iter().zip(std::iter::repeat(0)));
        count
    }
}

pub(crate) fn day11() {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();
    let energy_map: EnergyMap = input.parse().unwrap();

    let mut flashers = 0;
    let mut part_one = energy_map.clone();
    for _ in 0..100 {
        flashers += part_one.step();
    }
    println!("Part one answer is {}", flashers);

    let mut part_two = energy_map;
    for step in 1.. {
        if part_two.step() == 100 {
            println!("Part two answer is {}", step);
            break;
        }
    }
}
