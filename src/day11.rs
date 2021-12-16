use std::collections::HashSet;

#[derive(Clone)]
pub struct DumboOctopus {
    lights: Vec<u8>,
    width: usize,
}

impl crate::AdventOfCode for DumboOctopus {
    fn new(input: &str) -> Self {
        let mut iter = input.lines().map(|v| v.bytes().map(|v| v - 48));

        let mut lights = iter.next().unwrap().collect::<Vec<u8>>();
        let width = lights.len();
        lights.extend(iter.flatten());

        Self { lights, width }
    }

    fn part1(&self) -> u64 {
        let mut map = self.clone();
        (0..100).map(|_| map.step()).sum()
    }

    fn part2(&self) -> u64 {
        let mut map = self.clone();
        let mut count = 0;

        while map.lights.iter().any(|&v| v != 0) {
            count += 1;
            map.step();
        }

        count
    }
}

impl DumboOctopus {
    fn step(&mut self) -> u64 {
        let mut bloomed = HashSet::new();

        for idx in 0..self.lights.len() {
            if bloomed.contains(&idx) {
                continue;
            }

            if !self.bloom_at_idx(idx) {
                continue;
            }

            //bloom
            bloomed.insert(idx);
            let mut indexes = self.affect_indexes(idx, &bloomed);

            while let Some(idx) = indexes.pop() {
                if bloomed.contains(&idx) {
                    continue;
                }

                if self.bloom_at_idx(idx) {
                    bloomed.insert(idx);
                    indexes.extend(self.affect_indexes(idx, &bloomed));
                }
            }
        }

        bloomed.len() as u64
    }

    fn bloom_at_idx(&mut self, idx: usize) -> bool {
        if self.lights[idx] < 9 {
            self.lights[idx] += 1;
            false
        } else {
            self.lights[idx] = 0;
            true
        }
    }

    fn affect_indexes(&self, idx: usize, bloomed: &HashSet<usize>) -> Vec<usize> {
        let mut indexes = Vec::new();

        let x = idx % self.width;
        let y = idx / self.width;
        let combine_idx = |x, y| y * self.width + x;

        if x != 0 {
            indexes.push(combine_idx(x - 1, y));
        }

        if x < self.width - 1 {
            indexes.push(combine_idx(x + 1, y));
        }

        if y != 0 {
            indexes.push(combine_idx(x, y - 1));

            if x != 0 {
                indexes.push(combine_idx(x - 1, y - 1));
            }

            if x < self.width - 1 {
                indexes.push(combine_idx(x + 1, y - 1));
            }
        }

        if y < (self.lights.len() / self.width) - 1 {
            indexes.push(combine_idx(x, y + 1));

            if x != 0 {
                indexes.push(combine_idx(x - 1, y + 1));
            }

            if x < self.width - 1 {
                indexes.push(combine_idx(x + 1, y + 1));
            }
        }

        indexes.retain(|v| !bloomed.contains(v));
        indexes
    }
}
