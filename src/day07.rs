fn part_one(positions: &[isize]) -> isize {
    // Any median will do.
    let mut sorted = positions.to_vec();
    sorted.sort();
    let length = positions.len();
    let middle = length / 2;
    let median = sorted[middle];
    positions
        .iter()
        .map(|position| (position - median).abs())
        .sum()
}

fn part_two(positions: &[isize]) -> isize {
    // Solution is near the mean.
    let length = positions.len();
    let sum: isize = positions.iter().sum();
    let mean = sum / (length as isize);
    (mean..=mean + 1)
        .map(|m| {
            positions
                .iter()
                .map(|position| {
                    let distance = (position - m).abs();
                    (distance * (distance + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

pub(crate) fn day07() {
    let input = std::fs::read_to_string("data/day07.txt").unwrap();
    let positions: Vec<isize> = input
        .trim()
        .split(',')
        .map(|word| word.parse().unwrap())
        .collect();

    let cost = part_one(&positions);
    println!("Part one answer is {}", cost);

    let cost = part_two(&positions);
    println!("Part two answer is {}", cost);
}
