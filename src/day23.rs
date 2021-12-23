use std::cmp::Ordering;
use std::collections::BinaryHeap;

const CORRIDOR_X: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOM_X: [usize; 4] = [2, 4, 6, 8];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn home_room(&self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }
    fn step_cost(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Room {
    amphipods: Vec<Option<Amphipod>>,
}

impl Room {
    fn new(amphipods: Vec<Amphipod>) -> Self {
        let amphipods = amphipods.into_iter().map(Some).collect();
        Self { amphipods }
    }

    fn is_full(&self, room_size: usize, amphipod: Amphipod) -> bool {
        self.amphipods
            .iter()
            .take(room_size)
            .all(|a| a == &Some(amphipod))
    }

    fn has_other(&self, amphipod: Amphipod) -> bool {
        self.amphipods
            .iter()
            .any(|amp| amp.map_or(false, |other| other != amphipod))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Map {
    rooms: [Room; 4],
    corridor: [Option<Amphipod>; 7],
}

impl Map {
    fn new(rooms: [Room; 4]) -> Self {
        let corridor = [None; 7];
        Self { rooms, corridor }
    }

    fn finished(&self, room_size: usize) -> bool {
        self.rooms
            .iter()
            .zip([
                Amphipod::Amber,
                Amphipod::Bronze,
                Amphipod::Copper,
                Amphipod::Desert,
            ])
            .all(|(room, amphipod)| room.is_full(room_size, amphipod))
    }

    fn corridor_occupied(&self, lo_x: usize, hi_x: usize) -> bool {
        self.corridor
            .iter()
            .zip(CORRIDOR_X)
            .any(|(amphipod, cx)| amphipod.is_some() && lo_x <= cx && cx <= hi_x)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    map: Map,
    cost: usize,
}

fn diff(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl State {
    fn new(map: Map, cost: usize) -> Self {
        Self { map, cost }
    }

    fn finished(&self, room_size: usize) -> bool {
        self.map.finished(room_size)
    }

    fn neighbours(&self, room_size: usize) -> Vec<Self> {
        // Consider putting a corridor amphipod in its home room.
        for (ci, amphipod) in self.map.corridor.iter().enumerate() {
            if let Some(amphipod) = amphipod {
                // Never block in something that's not at home.
                let home_room = amphipod.home_room();
                if self.map.rooms[home_room].has_other(*amphipod) {
                    continue;
                }

                // Don't pass through other amphipods.
                let rx = ROOM_X[home_room];
                let cx = CORRIDOR_X[ci];
                let (lo, hi) = if rx < cx { (rx, cx - 1) } else { (cx + 1, rx) };
                if self.map.corridor_occupied(lo, hi) {
                    continue;
                }

                // Go as far to the back of the room as we can.
                let occupied = self.map.rooms[home_room]
                    .amphipods
                    .iter()
                    .position(|a| a.is_some())
                    .unwrap_or(room_size);
                let back = occupied - 1;

                let distance = back + 1 + diff(rx, cx);
                let mut new_state = self.clone();
                new_state.map.rooms[home_room].amphipods[back] = new_state.map.corridor[ci].take();
                new_state.cost = self.cost + amphipod.step_cost() * distance;

                // If we can do this, it's definitely a good move; no need to consider further
                // neighbours.
                return vec![new_state];
            }
        }

        // Consider leaving a room.
        let mut neighbours = vec![];
        for (ri, room) in self.map.rooms.iter().enumerate() {
            if let Some(front) = room.amphipods.iter().position(|a| a.is_some()) {
                let amphipod = room.amphipods[front].unwrap();

                // Don't move if we're already at home, and not blocking anything.
                let home_room = amphipod.home_room();
                if ri == home_room && !room.has_other(amphipod) {
                    continue;
                }

                // Consider the possible places to enter the corridor.
                let rx = ROOM_X[ri];
                for (ci, &cx) in CORRIDOR_X.iter().enumerate() {
                    // Don't pass through other amphipods.
                    let (lo, hi) = if rx < cx { (rx, cx) } else { (cx, rx) };
                    if self.map.corridor_occupied(lo, hi) {
                        continue;
                    }

                    let distance = 1 + front + diff(rx, cx);
                    let mut new_state = self.clone();
                    new_state.map.corridor[ci] = new_state.map.rooms[ri].amphipods[front].take();
                    new_state.cost = self.cost + amphipod.step_cost() * distance;
                    neighbours.push(new_state);
                }
            }
        }

        neighbours
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(map: Map, room_size: usize) -> usize {
    let start = State::new(map, 0);
    let mut costs = hashmap! { start.map.clone() => 0 };
    let mut queue = BinaryHeap::new();
    queue.push(start);

    while let Some(state) = queue.pop() {
        if state.finished(room_size) {
            return state.cost;
        }

        for neighbour in state.neighbours(room_size) {
            let current_cost = costs.entry(neighbour.map.clone()).or_insert(usize::MAX);
            if neighbour.cost < *current_cost {
                *current_cost = neighbour.cost;
                queue.push(neighbour);
            }
        }
    }
    usize::MAX
}

pub(crate) fn day23() {
    // #############
    // #...........#
    // ###D#C#D#B###
    //   #B#A#A#C#
    //   #########
    let amber = Room::new(vec![Amphipod::Desert, Amphipod::Bronze]);
    let bronze = Room::new(vec![Amphipod::Copper, Amphipod::Amber]);
    let copper = Room::new(vec![Amphipod::Desert, Amphipod::Amber]);
    let desert = Room::new(vec![Amphipod::Bronze, Amphipod::Copper]);
    let map = Map::new([amber, bronze, copper, desert]);

    let cost = solve(map, 2);
    println!("Part one answer is {}", cost);

    let amber = Room::new(vec![
        Amphipod::Desert,
        Amphipod::Desert,
        Amphipod::Desert,
        Amphipod::Bronze,
    ]);
    let bronze = Room::new(vec![
        Amphipod::Copper,
        Amphipod::Copper,
        Amphipod::Bronze,
        Amphipod::Amber,
    ]);
    let copper = Room::new(vec![
        Amphipod::Desert,
        Amphipod::Bronze,
        Amphipod::Amber,
        Amphipod::Amber,
    ]);
    let desert = Room::new(vec![
        Amphipod::Bronze,
        Amphipod::Amber,
        Amphipod::Copper,
        Amphipod::Copper,
    ]);
    let map = Map::new([amber, bronze, copper, desert]);

    let cost = solve(map, 4);
    println!("Part two answer is {}", cost);
}
