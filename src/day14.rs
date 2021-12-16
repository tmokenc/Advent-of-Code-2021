use std::collections::HashMap;
use std::mem;

type Pair = u16;
type Rules = HashMap<Pair, (Pair, Pair)>;

fn chars_to_pair(lhs: char, rhs: char) -> u16 {
    (lhs as u16) << 8 | rhs as u16
}

fn pair_to_chars(pair: Pair) -> (char, char) {
    let lhs = (pair >> 8) as u8 as char;
    let rhs = (pair & 0x00ff) as u8 as char;
    (lhs, rhs)
}

struct Pairs {
    pair_count: HashMap<Pair, u64>,
}

impl Pairs {
    fn from_template(s: &str) -> Self {
        let mut pair_count = HashMap::new();
        let iter = s.chars().zip(s.chars().skip(1));

        for (lhs, rhs) in iter {
            let key = chars_to_pair(lhs, rhs);
            *pair_count.entry(key).or_insert(0) += 1;
        }

        Self { pair_count }
    }

    fn step_one(&mut self, rules: &Rules) {
        for (pair, count) in mem::take(&mut self.pair_count) {
            match rules.get(&pair) {
                None => *self.pair_count.entry(pair).or_insert(0) += count,
                Some((lhs, rhs)) => {
                    *self.pair_count.entry(*lhs).or_insert(0) += count;
                    *self.pair_count.entry(*rhs).or_insert(0) += count;
                }
            }
        }
    }

    fn step(&mut self, rules: &Rules, times: u8) {
        for _ in 0..times {
            self.step_one(rules);
        }
    }

    fn min_max(&self) -> (u64, u64) {
        let mut counter = HashMap::new();
        let mut min = u64::MAX;
        let mut max = 0;

        for (pair, count) in self.pair_count.iter() {
            let (lhs, rhs) = pair_to_chars(*pair);
            *counter.entry(lhs).or_insert(0) += count;
            *counter.entry(rhs).or_insert(0) += count;
        }

        for all_count in counter.into_values() {
            let count = all_count / 2 + (all_count & 1);

            if count < min {
                min = count;
            }

            if count > max {
                max = count;
            }
        }

        (min, max)
    }
}

pub struct ExtendedPolymerization {
    template: String,
    rules: Rules,
}

impl crate::AdventOfCode for ExtendedPolymerization {
    fn new(input: &str) -> Self {
        let mut iter = input.lines();
        let template = iter.next().unwrap().to_owned();
        let rules: Rules = iter
            .skip(1)
            .map(|line| {
                let mut iter = line.chars();
                let lhs = iter.next().unwrap();
                let rhs = iter.next().unwrap();
                let middle = iter.nth(4).unwrap();

                let from = chars_to_pair(lhs, rhs);
                let to = (chars_to_pair(lhs, middle), chars_to_pair(middle, rhs));

                (from, to)
            })
            .collect();

        Self { template, rules }
    }

    fn part1(&self) -> u64 {
        let mut pairs = Pairs::from_template(&self.template);
        pairs.step(&self.rules, 10);
        let (min, max) = pairs.min_max();
        max - min
    }

    fn part2(&self) -> u64 {
        let mut pairs = Pairs::from_template(&self.template);
        pairs.step(&self.rules, 40);
        let (min, max) = pairs.min_max();
        max - min
    }
}
