pub struct SonarSweep {
    data: Vec<i32>,
}

impl crate::AdventOfCode for SonarSweep {
    fn new(input: &str) -> Self {
        Self {
            data: input.lines().filter_map(|v| v.parse().ok()).collect(),
        }
    }

    fn part1(&self) -> u64 {
        self.data
            .iter()
            .skip(1)
            .zip(self.data.iter())
            .filter(|(a, b)| a > b)
            .count() as u64
    }

    fn part2(&self) -> u64 {
        self.data
            .windows(3)
            .skip(1)
            .zip(self.data.windows(3))
            .filter(|(a, b)| a.iter().sum::<i32>() > b.iter().sum())
            .count() as u64
    }
}
