use itertools::Either;

#[derive(Debug)]
struct BlackBox {
    pop: bool,
    b: u32,
    c: u32,
}

impl BlackBox {
    fn new(pop: bool, b: u32, c: u32) -> Self {
        Self { pop, b, c }
    }

    // Simplified instructions, which we interpret as operating on a stack of digits (mod 26).
    //
    // Each box maybe pops and maybe pushes a digit.  Boxes that don't pop are always set up so
    // that they are guaranteed to push.
    fn run(&self, w: u32, mut z: u32) -> u32 {
        let x = z % 26;

        if self.pop {
            z /= 26;
        }

        if !self.pop || x != w + self.b {
            z *= 26;
            z += w + self.c;
        }

        z
    }
}

fn solve(boxes: &[BlackBox], min: bool, ws: u64, z: u32) -> Option<u64> {
    if boxes.is_empty() {
        return Some(ws);
    }

    // Count either up or down.
    let iter = if min {
        Either::Left(1..=9)
    } else {
        Either::Right((1..=9).rev())
    };

    // Since we have seven boxes that will definitely push to the stack, we'd better arrange that
    // the seven boxes that pop from it do nothing else.
    let next_box = &boxes[0];
    for w in iter {
        let new_z = next_box.run(w, z);
        if next_box.pop && new_z != z / 26 {
            continue;
        }
        let new_ws = 10 * ws + w as u64;
        if let Some(solution) = solve(&boxes[1..], min, new_ws, new_z) {
            return Some(solution);
        }
    }

    None
}

pub(crate) fn day24() {
    let boxes = [
        BlackBox::new(false, 11, 5),
        BlackBox::new(false, 13, 5),
        BlackBox::new(false, 12, 1),
        BlackBox::new(false, 15, 15),
        BlackBox::new(false, 10, 2),
        BlackBox::new(true, 1, 2),
        BlackBox::new(false, 14, 5),
        BlackBox::new(true, 8, 8),
        BlackBox::new(true, 7, 14),
        BlackBox::new(true, 8, 12),
        BlackBox::new(false, 11, 7),
        BlackBox::new(true, 2, 14),
        BlackBox::new(true, 2, 13),
        BlackBox::new(true, 13, 6),
    ];

    let part_one = solve(&boxes, false, 0, 0).unwrap();
    println!("Part one answer is {}", part_one);

    let part_two = solve(&boxes, true, 0, 0).unwrap();
    println!("Part two answer is {}", part_two);
}
