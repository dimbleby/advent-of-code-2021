use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Sub;
use std::str::FromStr;

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
    rotations: Vec<HashSet<Vec3D>>,
}

impl Scanner {
    fn new(readings: HashSet<Vec3D>) -> Self {
        let mut rotations = vec![HashSet::default(); 24];
        for reading in &readings {
            for (point, set) in reading.rotations().into_iter().zip(rotations.iter_mut()) {
                set.insert(point);
            }
        }

        Self {
            readings,
            rotations,
        }
    }

    fn find_match(&self, other: &Scanner) -> Option<(Vec3D, HashSet<Vec3D>)> {
        for readings in &other.rotations {
            for (p1, p2) in itertools::iproduct!(self.readings.iter(), readings.iter()) {
                let offset = *p2 - *p1;
                let realigned = readings.iter().map(|reading| *reading - offset).collect();
                let hits = self.readings.intersection(&realigned).count();
                if hits >= 12 {
                    return Some((offset, realigned));
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

    while let Some(mut unfixed) = unmapped.pop_front() {
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
                unfixed.readings = alignment;
                mapped.insert(position, unfixed);
            }
        }
    }

    println!("Part one answer is {}", full_map.len());

    let part_two = itertools::iproduct!(mapped.keys(), mapped.keys())
        .map(|(point1, point2)| point1.distance(point2))
        .max()
        .unwrap();

    println!("Part two answer is {}", part_two);
}
