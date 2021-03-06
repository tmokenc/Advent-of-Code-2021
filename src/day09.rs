use std::collections::HashSet;

pub struct SmokeBasin {
    map: Vec<Vec<u8>>,
}

impl crate::AdventOfCode for SmokeBasin {
    fn new(input: &str) -> Self {
        let mut map = Vec::new();

        for line in input.lines() {
            let y = line
                .chars()
                .map(|v| v.to_digit(10).unwrap() as u8)
                .collect();
            map.push(y);
        }

        Self { map }
    }

    fn part1(&self) -> u64 {
        let mut risk = 0;

        if self.map.is_empty() || self.map[0].is_empty() {
            return 0;
        }

        for (y, row) in self.map.iter().enumerate() {
            for (x, value) in row.iter().copied().enumerate() {
                if value > 8 {
                    continue;
                }

                let mut to_check = Vec::new();

                if x != 0 {
                    to_check.push(row[x - 1]);
                }

                if y != 0 {
                    to_check.push(self.map[y - 1][x]);
                }

                if let Some(t) = row.get(x + 1) {
                    to_check.push(*t);
                }

                if let Some(row) = self.map.get(y + 1) {
                    to_check.push(row[x]);
                }

                if to_check.into_iter().all(|v| value < v) {
                    risk += value as u64 + 1;
                }
            }
        }

        risk
    }

    fn part2(&self) -> u64 {
        let mut visited = HashSet::new();
        let mut largest_3 = [1u64; 3];

        if self.map.get(0).filter(|v| !v.is_empty()).is_none() {
            return 0;
        }

        let x_len = self.map[0].len();
        let y_len = self.map.len();

        let next_moves = |x, y| {
            let mut moves = Vec::new();

            if x != 0 {
                moves.push((x - 1, y));
            }

            if y != 0 {
                moves.push((x, y - 1));
            }

            if x != x_len - 1 {
                moves.push((x + 1, y));
            }

            if y != y_len - 1 {
                moves.push((x, y + 1));
            }

            moves
        };

        let iter = (0..x_len)
            .map(|x| std::iter::repeat(x).zip(0..y_len))
            .flatten()
            .filter(|(x, y)| self.map[*y][*x] < 9);

        for (x, y) in iter {
            if !visited.insert((x, y)) {
                continue;
            }

            let mut result = 1;
            let mut possible_moves = next_moves(x, y);

            while let Some((x, y)) = possible_moves.pop() {
                if !visited.insert((x, y)) || self.map[y][x] == 9 {
                    continue;
                }

                result += 1;
                possible_moves.extend(next_moves(x, y));
            }

            if result > largest_3[0] {
                largest_3[0] = result;
                largest_3.sort_unstable();
            }
        }

        largest_3.iter().product()
    }
}
