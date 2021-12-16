use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Clone)]
struct RiskMap {
    levels: Vec<Vec<u32>>,
    rows: usize,
    columns: usize,
}

impl FromStr for RiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels = vec![];
        for line in s.lines() {
            let row: Vec<u32> = line
                .chars()
                .map(|c| c.to_digit(10).ok_or(()))
                .collect::<Result<_, _>>()?;
            levels.push(row);
        }

        let risk_map = Self::new(levels);
        Ok(risk_map)
    }
}

#[derive(PartialEq, Eq)]
struct SearchState {
    position: (usize, usize),
    total_risk: u32,
}

impl SearchState {
    fn new(position: (usize, usize), risk: u32) -> Self {
        Self {
            position,
            total_risk: risk,
        }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_risk.cmp(&self.total_risk)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RiskMap {
    fn new(levels: Vec<Vec<u32>>) -> Self {
        let rows = levels.len();
        let columns = levels[0].len();
        Self {
            levels,
            rows,
            columns,
        }
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .iter()
            .filter_map(move |(dx, dy)| {
                let neighbour = (x as isize + dx, y as isize + dy);
                if neighbour.0 < 0 || neighbour.1 < 0 {
                    return None;
                }
                let neighbour = (neighbour.0 as usize, neighbour.1 as usize);
                if neighbour.0 >= self.rows || neighbour.1 >= self.columns {
                    return None;
                }
                Some(neighbour)
            })
    }

    fn minimum_risk(&self) -> u32 {
        let start = (0, 0);
        let goal = (self.columns - 1, self.rows - 1);

        let mut risks = vec![vec![std::u32::MAX; self.columns]; self.rows];
        risks[start.1][start.0] = 0;

        let start_state = SearchState::new(start, 0);
        let mut queue = BinaryHeap::new();
        queue.push(start_state);

        while let Some(state) = queue.pop() {
            if state.position == goal {
                break;
            }

            for neighbour in self.neighbours(state.position) {
                let step_risk = self.levels[neighbour.1][neighbour.0];
                let total_risk = state.total_risk + step_risk;
                let best_risk = &mut risks[neighbour.1][neighbour.0];
                if total_risk >= *best_risk {
                    continue;
                }
                *best_risk = total_risk;
                let new_state = SearchState::new(neighbour, total_risk);
                queue.push(new_state);
            }
        }

        risks[goal.1][goal.0]
    }

    fn extended(&self) -> Self {
        let mut extended_levels = vec![Vec::with_capacity(5 * self.columns); 5 * self.rows];
        for (y, extended_row) in extended_levels.iter_mut().enumerate() {
            for x in 0..5 * self.rows {
                let orig_x = x % self.columns;
                let orig_y = y % self.rows;
                let increment = (x / self.columns) + (y / self.rows);
                let mut new_risk = self.levels[orig_y][orig_x] + increment as u32;
                if new_risk > 9 {
                    new_risk %= 9;
                }
                extended_row.push(new_risk);
            }
        }
        Self::new(extended_levels)
    }
}

pub(crate) fn day15() {
    let input = std::fs::read_to_string("data/day15.txt").unwrap();
    let risk_map: RiskMap = input.parse().unwrap();

    let part_one = risk_map.minimum_risk();
    println!("Part one answer is {}", part_one);

    let risk_map = risk_map.extended();
    let part_two = risk_map.minimum_risk();
    println!("Part two answer is {}", part_two);
}
