fn count_ones(numbers: &[usize], bit: usize) -> usize {
    numbers
        .iter()
        .filter(|&number| number & (1 << bit) != 0)
        .count()
}

fn apply_criteria(mut numbers: Vec<usize>, bit: usize, oxygen: bool) -> Vec<usize> {
    let length = numbers.len();
    if numbers.len() == 1 {
        return numbers;
    }

    let ones = count_ones(&numbers, bit);

    let keep_ones = if oxygen {
        2 * ones >= length
    } else {
        2 * ones < length
    };

    numbers.retain(|number| (number & (1 << bit) != 0) == keep_ones);
    numbers
}

pub(crate) fn day03() {
    let input = std::fs::read_to_string("data/day03.txt").unwrap();
    let numbers: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();
    let length = numbers.len();

    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..12 {
        let ones = count_ones(&numbers, bit);
        if 2 * ones > length {
            gamma |= 1 << bit
        } else {
            epsilon |= 1 << bit
        }
    }
    println!("Part one answer is {}", gamma * epsilon);

    let mut oxygen_set = numbers.clone();
    let mut co2_set = numbers;
    for bit in (0..12).rev() {
        oxygen_set = apply_criteria(oxygen_set, bit, true);
        co2_set = apply_criteria(co2_set, bit, false);
    }
    println!("Part two answer is {}", oxygen_set[0] * co2_set[0]);
}
