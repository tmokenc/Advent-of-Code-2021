pub struct TheTreacheryOfWhales {
    data: Vec<i32>,
}

impl crate::AdventOfCode for TheTreacheryOfWhales {
    fn new(input: &str) -> Self {
        Self {
            data: input
                .split(",")
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<_>>(),
        }
    }

    fn part1(&self) -> u64 {
        if self.data.is_empty() {
            return 0;
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for i in &self.data {
            if *i < min {
                min = *i;
            } else if *i > max {
                max = *i;
            }
        }

        (min..=max)
            .map(|v| self.data.iter().copied().map(|x| (v - x).abs()).sum())
            .min()
            .unwrap_or(0i32) as u64
    }

    fn part2(&self) -> u64 {
        if self.data.is_empty() {
            return 0;
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for i in &self.data {
            if *i < min {
                min = *i;
            } else if *i > max {
                max = *i;
            }
        }

        (min..=max)
            .map(|v| {
                self.data
                    .iter()
                    .copied()
                    .map(|x| {
                        let steps = (v - x).abs();
                        (0..=steps).sum::<i32>()
                    })
                    .sum()
            })
            .min()
            .unwrap_or(0i32) as u64
    }
}
