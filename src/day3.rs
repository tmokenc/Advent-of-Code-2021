fn append_binary(num: u64, b: bool) -> u64 {
    (num << 1) | b as u64
}

fn binary_slice_to_num(s: &[u8]) -> u64 {
    s.iter().fold(0, |v, x| v << 1 | *x as u64)
}

pub struct BinaryDiagnostic {
    data: Vec<Vec<u8>>,
}

impl crate::AdventOfCode for BinaryDiagnostic {
    fn new(input: &str) -> Self {
        Self {
            data: input
                .lines()
                .map(|v| v.bytes().map(|x| x - 48).collect())
                .collect(),
        }
    }

    fn part1(&self) -> u64 {
        let mut iter = self.data.iter();
        let mut data = iter
            .next()
            .unwrap()
            .iter()
            .map(|v| *v as i32)
            .collect::<Vec<i32>>();

        iter.flat_map(|v| v.iter().enumerate()).for_each(|(i, v)| {
            if *v == 0 {
                data[i] -= 1;
            } else if *v == 1 {
                data[i] += 1;
            }
        });

        let gamma = data.iter().map(|&v| v > 0).fold(0, append_binary);
        let elipsion = data.iter().map(|&v| v < 1).fold(0, append_binary);

        gamma * elipsion
    }

    fn part2(&self) -> u64 {
        let bit_criteria = |greater: bool| {
            let mut current_pos = 0;
            let mut indexes = (0..self.data.len()).collect::<Vec<usize>>();

            while indexes.len() > 1 && current_pos < self.data[0].len() {
                let score = indexes
                    .iter()
                    .map(|v| self.data[*v][current_pos])
                    .fold(0, |v, x| v + if x == 1 { 1 } else { -1 });

                let match_bit = if score == 0 {
                    greater as u8
                } else {
                    score.clamp(0, 1) as u8 ^ !greater as u8
                };

                indexes.retain(|v| self.data[*v][current_pos] == match_bit);
                current_pos += 1;
            }

            binary_slice_to_num(&self.data[indexes[0]])
        };

        let oxygen = bit_criteria(true);
        let co2 = bit_criteria(false);

        oxygen * co2
    }
}
