use std::collections::BTreeSet;

pub struct Chiton {
    map: Vec<Vec<u8>>,
}

impl Chiton {
    fn low_risk_path(&self) -> Vec<(usize, usize)> {
        // let mut queue = BTreeSet::new();
        Vec::new()
    }
}

impl crate::AdventOfCode for Chiton {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|v| v.bytes().map(|x| x - 48).collect())
            .collect();

        Self { map }
    }

    fn part1(&self) -> u64 {
        self.low_risk_path()
            .into_iter()
            .map(|(y, x)| self.map[y][x] as u64)
            .sum()
    }

    fn part2(&self) -> u64 {
        0
    }
}
