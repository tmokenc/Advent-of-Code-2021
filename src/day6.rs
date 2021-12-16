pub struct Lanternfish {
    timers: Vec<u8>,
}

impl Lanternfish {
    fn lanternfish(&self, day: u64) -> u64 {
        let mut data = [0u64; 9];

        for timer in &self.timers {
            data[*timer as usize] += 1;
        }

        for _ in 0..day {
            let new = data[0];

            for i in 1..=8 {
                data[i - 1] = data[i];
            }

            data[6] += new;
            data[8] = new;
        }

        data.iter().sum()
    }
}

impl crate::AdventOfCode for Lanternfish {
    fn new(input: &str) -> Self {
        Self {
            timers: input
                .split(",")
                .filter_map(|v| v.parse::<u8>().ok())
                .collect(),
        }
    }

    fn part1(&self) -> u64 {
        self.lanternfish(80)
    }

    fn part2(&self) -> u64 {
        self.lanternfish(256)
    }
}
