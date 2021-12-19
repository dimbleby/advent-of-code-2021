use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Sub;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Vec3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3D {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn rotations(&self) -> Vec<Self> {
        let (x, y, z) = (self.x, self.y, self.z);
        vec![
            Self::new(x, y, z),
            Self::new(-x, -y, z),
            Self::new(-x, y, -z),
            Self::new(x, -y, -z),
            Self::new(y, z, x),
            Self::new(-y, -z, x),
            Self::new(-y, z, -x),
            Self::new(y, -z, -x),
            Self::new(z, x, y),
            Self::new(-z, -x, y),
            Self::new(-z, x, -y),
            Self::new(z, -x, -y),
            Self::new(-x, -z, -y),
            Self::new(x, z, -y),
            Self::new(x, -z, y),
            Self::new(-x, z, y),
            Self::new(-y, -x, -z),
            Self::new(y, x, -z),
            Self::new(y, -x, z),
            Self::new(-y, x, z),
            Self::new(-z, -y, -x),
            Self::new(z, y, -x),
            Self::new(z, -y, x),
            Self::new(-z, y, x),
        ]
    }

    fn distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn normalized(&self) -> Self {
        let mut coords = vec![self.x.abs(), self.y.abs(), self.z.abs()];
        coords.sort_unstable();
        Self::new(coords[0], coords[1], coords[2])
    }
}

impl Sub for Vec3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl FromStr for Vec3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',');
        let x = coords.next().ok_or(())?.parse().map_err(|_| ())?;
        let y = coords.next().ok_or(())?.parse().map_err(|_| ())?;
        let z = coords.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Self::new(x, y, z))
    }
}

#[derive(Clone, Debug)]
struct Scanner {
    readings: HashSet<Vec3D>,
    fingerprint: HashMap<Vec3D, Vec<Vec3D>>,
}

impl Scanner {
    fn new(readings: HashSet<Vec3D>) -> Self {
        let mut fingerprint: HashMap<Vec3D, Vec<Vec3D>> = HashMap::default();
        for pair in readings.iter().cloned().combinations(2) {
            let diff = (pair[0] - pair[1]).normalized();
            let entry = fingerprint.entry(diff).or_insert_with(Vec::new);
            entry.extend(pair.into_iter());
        }

        Self {
            readings,
            fingerprint,
        }
    }

    fn rotations(&self) -> Vec<HashSet<Vec3D>> {
        let mut result = vec![HashSet::default(); 24];
        for reading in &self.readings {
            for (point, set) in reading.rotations().into_iter().zip(result.iter_mut()) {
                set.insert(point);
            }
        }
        result
    }

    fn find_match(&self, other: &Scanner) -> Option<(Vec3D, HashSet<Vec3D>)> {
        let matching_prints: HashSet<_> = self
            .fingerprint
            .keys()
            .filter(|key| other.fingerprint.contains_key(key))
            .collect();

        if matching_prints.len() < 50 {
            return None;
        }

        for print in matching_prints {
            for (p1, p2) in itertools::iproduct!(
                self.fingerprint[print].iter(),
                other.fingerprint[print].iter()
            ) {
                let p2_rotations = p2.rotations();
                let offsets = p2_rotations.iter().map(|point| *point - *p1);
                for (offset, readings) in offsets.zip(other.rotations()) {
                    let realigned = readings.iter().map(|reading| *reading - offset).collect();
                    let hits = self.readings.intersection(&realigned).count();
                    if hits >= 12 {
                        return Some((offset, realigned));
                    }
                }
            }
        }

        None
    }
}

fn read_scanner<'a, T: Iterator<Item = &'a str>>(lines: &mut T) -> Option<Scanner> {
    lines.next()?;

    let readings: HashSet<Vec3D> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    let scanner = Scanner::new(readings);

    Some(scanner)
}

pub(crate) fn day19() {
    let input = std::fs::read_to_string("data/day19.txt").unwrap();
    let mut lines = input.lines();
    let mut scanners: Vec<Scanner> = vec![];
    while let Some(scanner) = read_scanner(&mut lines) {
        scanners.push(scanner);
    }

    let mut unmapped: VecDeque<Scanner> = scanners.iter().cloned().collect();
    let start = unmapped.pop_front().unwrap();
    let mut full_map: HashSet<Vec3D> = start.readings.clone();
    let mut mapped: HashMap<Vec3D, Scanner> = hashmap! { Vec3D::default() => start };

    while let Some(unfixed) = unmapped.pop_front() {
        let mut realigned = None;
        for fixed in mapped.values() {
            realigned = fixed.find_match(&unfixed);
            if realigned.is_some() {
                break;
            }
        }

        match realigned {
            None => {
                unmapped.push_back(unfixed);
            }
            Some((position, alignment)) => {
                full_map.extend(alignment.iter());
                mapped.insert(position, Scanner::new(alignment));
            }
        }
    }

    println!("Part one answer is {}", full_map.len());

    let part_two = mapped
        .keys()
        .combinations(2)
        .map(|points| points[0].distance(points[1]))
        .max()
        .unwrap();

    println!("Part two answer is {}", part_two);
}
