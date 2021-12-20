use std::collections::HashMap;

type Point = (isize, isize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pixel {
    Dark,
    Light,
}

impl Pixel {
    fn as_bit(&self) -> usize {
        match self {
            Self::Dark => 0,
            Self::Light => 1,
        }
    }
}

#[derive(Clone, Debug)]
struct Image {
    background: Pixel,
    pixels: HashMap<Point, Pixel>,

    // The extent of the image that is not simply background.
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Image {
    fn new(pixels: HashMap<Point, Pixel>) -> Self {
        let min_x = pixels.keys().map(|point| point.0).min().unwrap_or(0);
        let min_y = pixels.keys().map(|point| point.1).min().unwrap_or(0);
        let max_x = pixels.keys().map(|point| point.0).max().unwrap_or(0);
        let max_y = pixels.keys().map(|point| point.1).max().unwrap_or(0);
        let background = Pixel::Dark;
        Self {
            background,
            pixels,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn get_pixel(&self, point: &Point) -> Pixel {
        *self.pixels.get(point).unwrap_or(&self.background)
    }

    fn enhance(&mut self, key: &[Pixel]) {
        let mut pixels: HashMap<Point, Pixel> = HashMap::default();
        for (x, y) in itertools::iproduct!(
            self.min_x - 1..=self.max_x + 1,
            self.min_y - 1..=self.max_y + 1
        ) {
            let pixel = self.enhanced_at(&(x, y), key);
            pixels.insert((x, y), pixel);
        }

        let background = match self.background {
            Pixel::Dark => key[0],
            Pixel::Light => key[511],
        };

        self.pixels = pixels;
        self.min_x -= 1;
        self.max_x += 1;
        self.min_y -= 1;
        self.max_y += 1;
        self.background = background;
    }

    fn enhanced_at(&self, point: &Point, key: &[Pixel]) -> Pixel {
        let &(x, y) = point;
        let mut number = 0;
        for yi in [y - 1, y, y + 1] {
            for xi in [x - 1, x, x + 1] {
                let pixel = self.get_pixel(&(xi, yi));
                let bit = pixel.as_bit();
                number *= 2;
                number += bit;
            }
        }

        key[number]
    }

    fn count_lit(&self) -> usize {
        assert!(self.background == Pixel::Dark);
        self.pixels
            .values()
            .filter(|&&pixel| pixel == Pixel::Light)
            .count()
    }
}

pub(crate) fn day20() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();
    let mut lines = input.lines();

    let key_line = lines.next().unwrap();
    let key: Vec<Pixel> = key_line
        .chars()
        .map(|c| if c == '.' { Pixel::Dark } else { Pixel::Light })
        .collect();

    let _blank = lines.next().unwrap();

    let mut pixels: HashMap<Point, Pixel> = HashMap::default();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pixel = if c == '.' { Pixel::Dark } else { Pixel::Light };
            pixels.insert((x as isize, y as isize), pixel);
        }
    }
    let image = Image::new(pixels);

    let mut part_one = image.clone();
    for _ in 0..2 {
        part_one.enhance(&key);
    }
    println!("Part one answer is {}", part_one.count_lit());

    let mut part_two = image;
    for _ in 0..50 {
        part_two.enhance(&key);
    }
    println!("Part two answer is {}", part_two.count_lit());
}
