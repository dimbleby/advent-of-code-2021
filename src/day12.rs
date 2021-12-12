use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;

struct Cave {
    neighbours: Vec<String>,
    little: bool,
}

impl Cave {
    fn new(id: String) -> Self {
        let neighbours = vec![];
        let little = id.chars().all(|c| c.is_lowercase());
        Self { neighbours, little }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Path<'a> {
    visited: BTreeSet<&'a str>,
    revisited: bool,
    current: &'a str,
}

impl<'a> Path<'a> {
    fn new(start: &'a str) -> Self {
        Self {
            visited: btreeset! { start },
            revisited: false,
            current: start,
        }
    }
}

#[derive(Default)]
struct CaveMap {
    caves: HashMap<String, Cave>,
}

impl FromStr for CaveMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave_map = CaveMap::default();
        for line in s.lines() {
            let mut caves = line.split('-');
            let cave1 = caves.next().ok_or(())?;
            let cave2 = caves.next().ok_or(())?;
            cave_map.add_tunnel(cave1, cave2);
        }
        Ok(cave_map)
    }
}

impl CaveMap {
    fn add_tunnel(&mut self, id1: &str, id2: &str) {
        let cave1 = self
            .caves
            .entry(id1.to_owned())
            .or_insert_with(|| Cave::new(id1.to_owned()));
        cave1.neighbours.push(id2.to_owned());

        let cave2 = self
            .caves
            .entry(id2.to_owned())
            .or_insert_with(|| Cave::new(id2.to_owned()));
        cave2.neighbours.push(id1.to_owned());
    }

    fn count_routes_inner<'a>(
        &'a self,
        path: Path<'a>,
        allow_revisit: bool,
        cache: &mut HashMap<Path<'a>, usize>,
    ) -> usize {
        if let Some(cached) = cache.get(&path) {
            return *cached;
        }

        if path.current == "end" {
            return 1;
        }

        let mut sum = 0;
        for neighbour in &self.caves[path.current].neighbours {
            if neighbour == "start" {
                continue;
            }

            let cave = &self.caves[neighbour];
            let visited = cave.little && path.visited.contains(&neighbour.as_str());
            if visited && (!allow_revisit || path.revisited) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.current = neighbour;
            if visited {
                new_path.revisited = true;
            } else if cave.little {
                new_path.visited.insert(neighbour);
            }

            sum += self.count_routes_inner(new_path, allow_revisit, cache);
        }

        cache.insert(path, sum);
        sum
    }

    fn count_routes(&self, allow_revisit: bool) -> usize {
        let mut cache: HashMap<Path, usize> = HashMap::new();
        let start = Path::new("start");
        self.count_routes_inner(start, allow_revisit, &mut cache)
    }
}

pub(crate) fn day12() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();
    let cave_map: CaveMap = input.parse().unwrap();

    println!("Part one answer is {}", cave_map.count_routes(false));
    println!("Part two answer is {}", cave_map.count_routes(true));
}
