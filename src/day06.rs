use std::collections::HashMap;

fn update_population(population: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new: HashMap<usize, usize> = HashMap::new();
    let spawners = population.get(&0).unwrap_or(&0);
    new.insert(8, *spawners);

    for (clock, count) in &population {
        let new_clock = if *clock == 0 { 6 } else { clock - 1 };
        let counter = new.entry(new_clock).or_insert(0);
        *counter += *count;
    }
    new
}

pub(crate) fn day06() {
    let input = std::fs::read_to_string("data/day06.txt").unwrap();
    let fish: Vec<usize> = input
        .trim()
        .split(',')
        .map(|word| word.parse().unwrap())
        .collect();

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for f in fish {
        let counter = counts.entry(f).or_insert(0);
        *counter += 1;
    }

    let part_one = (0..80).fold(counts.clone(), |population, _day| {
        update_population(population)
    });
    let total: usize = part_one.values().sum();
    println!("Part one answer is {}", total);

    let part_two = (0..256).fold(counts.clone(), |population, _day| {
        update_population(population)
    });
    let total: usize = part_two.values().sum();
    println!("Part two answer is {}", total);
}
