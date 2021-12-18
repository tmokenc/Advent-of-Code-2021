use std::iter;
use std::ops::Range;

struct Bullet {
    dx: i32,
    dy: i32,
    x: i32,
    y: i32,
}

impl Bullet {
    fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy, x: 0, y: 0 }
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dy -= 1;

        if self.dx > 0 {
            self.dx -= 1;
        } else {
            self.dx = 0;
        }
    }
}

pub struct TrickShot {
    x: Range<i32>,
    y: Range<i32>,
}

impl TrickShot {
    fn target_hit(&self, x: i32, y: i32) -> bool {
        let mut b = Bullet::new(x, y);

        while b.y >= self.y.start && b.x <= self.x.end {
            b.step();
            if self.x.contains(&b.x) && self.y.contains(&b.y) {
                return true;
            }
        }

        false
    }
}

impl crate::AdventOfCode for TrickShot {
    fn new(input: &str) -> Self {
        let mut iter = input
            .trim_start_matches("target area: ")
            .split(", ")
            .map(|v| v.split("=").nth(1).unwrap())
            .flat_map(|v| v.split(".."))
            .map(|v| v.parse::<i32>().unwrap());

        let x1 = iter.next().unwrap();
        let x2 = iter.next().unwrap() + 1;
        let y1 = iter.next().unwrap();
        let y2 = iter.next().unwrap() + 1;

        Self {
            x: x1..x2,
            y: y1..y2,
        }
    }

    fn part1(&self) -> u64 {
        let y_target = self.y.start.abs() - 1;
        let result = y_target * (y_target + 1) / 2;
        result as u64
    }

    // brute force
    fn part2(&self) -> u64 {
        (1..self.x.end)
            .flat_map(|x| iter::repeat(x).zip(self.y.start..self.y.start.abs()))
            .filter(|&(x, y)| self.target_hit(x, y))
            .count() as u64
    }
}
