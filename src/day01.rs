pub(crate) fn day01() {
    let input = std::fs::read_to_string("data/day01.txt").unwrap();
    let readings: Vec<usize> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let increases = count_increases(&readings, 1);
    println!("Part one answer is {}", increases);

    let increases = count_increases(&readings, 3);
    println!("Part two answer is {}", increases);
}

fn count_increases(input: &[usize], step: usize) -> usize {
    input
        .iter()
        .zip(input[step..].iter())
        .filter(|(prev, next)| prev < next)
        .count()
}
