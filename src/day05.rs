struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(input: &str) -> Self {
        let mut iter = input.split(',').map(|v| v.parse::<usize>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        Self { x, y }
    }
}

struct Line {
    from: Coordinate,
    to: Coordinate,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}

fn count_overlaps<'a>(lines: impl Iterator<Item = &'a Line>) -> u64 {
    let mut map = [[0u8; 1000]; 1000];

    for Line { from, to } in lines {
        let Coordinate { mut x, mut y } = from;
        map[x][y] += 1;

        while x != to.x || y != to.y {
            if x < to.x {
                x += 1;
            } else if x > to.x {
                x -= 1;
            }

            if y < to.y {
                y += 1;
            } else if y > to.y {
                y -= 1;
            }

            map[x][y] += 1;
        }
    }

    map.into_iter().flatten().filter(|&v| v > 1).count() as u64
}

pub struct HydrothermalVenture {
    lines: Vec<Line>,
}

impl crate::AdventOfCode for HydrothermalVenture {
    fn new(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|v| {
                let mut iter = v.split(" -> ");
                let from = iter.next().unwrap();
                let to = iter.next().unwrap();

                Line {
                    from: Coordinate::new(from),
                    to: Coordinate::new(to),
                }
            })
            .collect::<Vec<_>>();

        Self { lines }
    }

    fn part1(&self) -> u64 {
        count_overlaps(self.lines.iter().filter(|line| line.is_straight()))
    }

    fn part2(&self) -> u64 {
        count_overlaps(self.lines.iter())
    }
}
