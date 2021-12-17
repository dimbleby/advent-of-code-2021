#[derive(Debug)]
struct Launch {
    dx: isize,
    dy: isize,
}

impl Launch {
    fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }

    fn trajectory(&self) -> Trajectory {
        Trajectory::new(0, 0, self.dx, self.dy)
    }

    fn hits(&self, x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> bool {
        for (x, y) in self.trajectory() {
            if y < y_min || x > x_max {
                return false;
            }
            if x_min <= x && y <= y_max {
                return true;
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct Trajectory {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Trajectory {
    fn new(x: isize, y: isize, dx: isize, dy: isize) -> Self {
        Self { x, y, dx, dy }
    }
}

impl Iterator for Trajectory {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let value = (self.x, self.y);
        self.x += self.dx;
        self.y += self.dy;
        self.dx -= self.dx.signum();
        self.dy -= 1;
        Some(value)
    }
}

pub(crate) fn day17() {
    let x_min = 185;
    let x_max = 221;
    let y_min = -122;
    let y_max = -74;

    // With initial dy > 0: as we hit the ground, dy will again have the initial magnitude but be
    // negative. We mustn't overshoot in the next step.
    println!("Part one answer is {}", (121 * 122) / 2);

    let hits = itertools::iproduct!((1..=221), (-122..=121))
        .filter(|(dx, dy)| Launch::new(*dx, *dy).hits(x_min, x_max, y_min, y_max))
        .count();
    println!("Part two answer is {}", hits);
}
