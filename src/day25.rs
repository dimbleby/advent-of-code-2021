use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum SeaCucumber {
    Eastbound,
    Southbound,
}

impl TryFrom<char> for SeaCucumber {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let sea_cucumber = match c {
            '>' => SeaCucumber::Eastbound,
            'v' => SeaCucumber::Southbound,
            _ => return Err(()),
        };
        Ok(sea_cucumber)
    }
}

struct SeaBed {
    grid: Vec<Vec<Option<SeaCucumber>>>,
    max_x: usize,
    max_y: usize,
}

impl FromStr for SeaBed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        for line in s.lines() {
            let row: Vec<Option<SeaCucumber>> = line.chars().map(|c| c.try_into().ok()).collect();
            grid.push(row);
        }

        let sea_bed = Self::new(grid);
        Ok(sea_bed)
    }
}

impl SeaBed {
    fn new(grid: Vec<Vec<Option<SeaCucumber>>>) -> Self {
        let max_y = grid.len();
        let max_x = grid[0].len();
        Self { grid, max_x, max_y }
    }

    fn step(&mut self) -> bool {
        self.step_east() | self.step_south()
    }

    fn step_east(&mut self) -> bool {
        let mut moved = false;
        for row in &mut self.grid {
            let moves: Vec<usize> = (0..self.max_x)
                .filter(|&x| {
                    row[x] == Some(SeaCucumber::Eastbound) && row[(x + 1) % self.max_x].is_none()
                })
                .collect();

            for &m in &moves {
                row[(m + 1) % self.max_x] = row[m].take();
            }
            moved |= !moves.is_empty();
        }

        moved
    }

    fn step_south(&mut self) -> bool {
        let mut moved = false;
        for x in 0..self.max_x {
            let moves: Vec<usize> = (0..self.max_y)
                .filter(|&y| {
                    self.grid[y][x] == Some(SeaCucumber::Southbound)
                        && self.grid[(y + 1) % self.max_y][x].is_none()
                })
                .collect();

            for &m in &moves {
                self.grid[(m + 1) % self.max_y][x] = self.grid[m][x].take();
            }
            moved |= !moves.is_empty();
        }

        moved
    }
}

pub(crate) fn day25() {
    let input = std::fs::read_to_string("data/day25.txt").unwrap();
    let mut sea_bed: SeaBed = input.parse().unwrap();
    let mut steps = 0;
    while sea_bed.step() {
        steps += 1;
    }
    println!("Part one answer is {}", steps + 1);
}
