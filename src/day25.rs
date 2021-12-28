#[derive(PartialEq, Clone, Copy)]
enum LocationEntry {
    East, // >
    South, // v
    Empty, // .
}

impl LocationEntry {
    fn parse(ch: char) -> Self {
        match ch {
            '>' => Self::East,
            'v' => Self::South,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct SeaCucumber {
    map: Vec<Vec<LocationEntry>>,
}

impl SeaCucumber {
    // return true if any of the sea cucumbers move
    fn step(&mut self) -> bool {
        let old_map = self.map.clone();

        let width = self.map[0].len();
        let height = self.map.len();

        type Step = ((usize, usize), (usize, usize));

        let (east, south): (Vec<Step>, Vec<Step>) = (0..height)
            .flat_map(|x| std::iter::repeat(x).zip(0..width))
            .filter_map(|from| {
                let mut to = from.to_owned();

                match self.map[from.0][from.1] {
                    LocationEntry::Empty => return None,

                    LocationEntry::East => {
                        to.1 += 1;

                        if to.1 >= width {
                            to.1 = 0;
                        }
                    }

                    LocationEntry::South => {
                        to.0 += 1;

                        if to.0 >= height {
                            to.0 = 0;
                        }
                    }
                }

                Some((from, to))
            })
            .partition(|(from, to)| to.0 == from.0);

        for mut coords in [east, south] {
            coords.retain(|(_, to)| self.map[to.0][to.1] == LocationEntry::Empty);

            for (from, to) in coords {
                self.map[to.0][to.1] = self.map[from.0][from.1];
                self.map[from.0][from.1] = LocationEntry::Empty;
            }

        }

        old_map != self.map
    }
}

impl crate::AdventOfCode for SeaCucumber {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|v| v.chars().map(LocationEntry::parse).collect())
            .collect();

        Self { map }
    }

    fn part1(&self) -> u64 {
        let mut map = self.clone();
        let mut count = 1;

        while map.step() {
            count += 1;
        }

        count
    }

    fn part2(&self) -> u64 {
        0
    }
}