use itertools::Itertools;

#[derive(Clone, Debug)]
enum SnailfishNumber {
    Regular(u32),
    Pair(Box<(SnailfishNumber, SnailfishNumber)>),
}

enum ExplosionResult {
    NoChange,
    Exploded(Option<u32>, Option<u32>),
}

impl SnailfishNumber {
    fn add(self, other: Self) -> Self {
        let mut sum = Self::Pair(Box::new((self, other)));
        sum.reduce();
        sum
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Regular(v) => *v,
            Self::Pair(pair) => (3 * pair.0.magnitude()) + (2 * pair.1.magnitude()),
        }
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        !matches!(self._explode(0), ExplosionResult::NoChange)
    }

    fn _explode(&mut self, depth: usize) -> ExplosionResult {
        match self {
            Self::Regular(_) => ExplosionResult::NoChange,

            Self::Pair(pair) => match &mut **pair {
                (Self::Regular(left_carry), Self::Regular(right_carry)) if depth >= 4 => {
                    let result = ExplosionResult::Exploded(Some(*left_carry), Some(*right_carry));
                    *self = Self::Regular(0);
                    result
                }

                (left, right) => {
                    let mut left_result = left._explode(depth + 1);
                    if let ExplosionResult::Exploded(_, right_carry) = &mut left_result {
                        if let Some(carry) = right_carry.take() {
                            right.carry_right(carry);
                        }
                        left_result
                    } else {
                        let mut right_result = right._explode(depth + 1);
                        if let ExplosionResult::Exploded(left_carry, _) = &mut right_result {
                            if let Some(carry) = left_carry.take() {
                                left.carry_left(carry);
                            }
                        }
                        right_result
                    }
                }
            },
        }
    }

    fn carry_left(&mut self, value: u32) {
        match self {
            Self::Regular(v) => *v += value,
            Self::Pair(pair) => pair.1.carry_left(value),
        }
    }

    fn carry_right(&mut self, value: u32) {
        match self {
            Self::Regular(v) => *v += value,
            Self::Pair(pair) => pair.0.carry_right(value),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Regular(v) if *v >= 10 => {
                let (v1, v2) = (*v / 2, (*v + 1) / 2);
                *self = Self::Pair(Box::new((Self::Regular(v1), Self::Regular(v2))));
                true
            }
            Self::Regular(_) => false,
            Self::Pair(pair) => pair.0.split() || pair.1.split(),
        }
    }
}

fn read_snailfish<T: Iterator<Item = char>>(chars: &mut T) -> SnailfishNumber {
    let c = chars.next().unwrap();
    if let Some(d) = c.to_digit(10) {
        return SnailfishNumber::Regular(d);
    }

    let left = read_snailfish(chars);
    let _comma = chars.next();
    let right = read_snailfish(chars);
    let _close = chars.next();

    SnailfishNumber::Pair(Box::new((left, right)))
}

pub(crate) fn day18() {
    let input = std::fs::read_to_string("data/day18.txt").unwrap();
    let numbers: Vec<SnailfishNumber> = input
        .lines()
        .map(|line| read_snailfish(&mut line.chars()))
        .collect();

    let total = numbers
        .clone()
        .into_iter()
        .reduce(|total, next| total.add(next))
        .unwrap();

    println!("Part one answer is {}", total.magnitude());

    let max = numbers
        .into_iter()
        .permutations(2)
        .map(|mut pair| {
            let first = pair.pop().unwrap();
            let second = pair.pop().unwrap();
            first.add(second).magnitude()
        })
        .max()
        .unwrap();

    println!("Part two answer is {}", max);
}
