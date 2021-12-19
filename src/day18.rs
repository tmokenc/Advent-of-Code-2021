use std::iter;
use std::ops::AddAssign;

#[derive(Debug, Clone)]
enum SnailfishNumber {
    Number(u8),
    Pair(Box<Self>, Box<Self>),
}

impl SnailfishNumber {
    fn parse(s: &str) -> Self {
        if let Ok(num) = s.parse::<u8>() {
            return Self::Number(num);
        }

        let mut pos = 1;

        let lhs = {
            let mut iter = s[pos..].chars();
            let mut ch = iter.next().unwrap();
            let mut open_brackets = 0u32;

            loop {
                match ch {
                    '[' => open_brackets += 1,
                    ']' => open_brackets -= 1,
                    ',' if open_brackets == 0 => {
                        break Self::parse(&s[1..pos]);
                    }
                    _ => (),
                }

                pos += 1;
                ch = iter.next().unwrap();
            }
        };

        pos += 1;
        let end = s.len() - 1;

        let rhs = Self::parse(&s[pos..end]);

        Self::Pair(Box::new(lhs), Box::new(rhs))
    }

    fn add_number(&mut self, value: u8, is_left: bool) {
        if value == 0 {
            return;
        }

        match self {
            Self::Number(n) => *n += value,
            Self::Pair(lhs, rhs) => {
                if is_left {
                    lhs.add_number(value, is_left);
                } else {
                    rhs.add_number(value, is_left)
                }
            }
        }
    }

    fn reduce(&mut self) {
        while self.explode(0).is_some() || self.split() {}
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Pair(lhs, rhs) => lhs.split() || rhs.split(),
            Self::Number(0..=9) => false,
            Self::Number(n) => {
                let lhs = *n / 2;
                let rhs = lhs + (*n & 1);

                *self = Self::Pair(Box::new(Self::Number(lhs)), Box::new(Self::Number(rhs)));

                true
            }
        }
    }

    fn explode(&mut self, depth: u8) -> Option<(u8, u8)> {
        if let Self::Pair(lhs, rhs) = self {
            if depth == 4 {
                let l = match **lhs {
                    Self::Number(n) => n,
                    _ => unreachable!(),
                };

                let r = match **rhs {
                    Self::Number(n) => n,
                    _ => unreachable!(),
                };

                *self = Self::Number(0);
                return Some((l, r));
            }

            if let Some((l, r)) = lhs.explode(depth + 1) {
                rhs.add_number(r, true);
                return Some((l, 0));
            }

            if let Some((l, r)) = rhs.explode(depth + 1) {
                lhs.add_number(l, false);
                return Some((0, r));
            }
        }

        None
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Number(n) => *n as u64,
            Self::Pair(lhs, rhs) => (lhs.magnitude() * 3) + (rhs.magnitude() * 2),
        }
    }
}

impl AddAssign for SnailfishNumber {
    fn add_assign(&mut self, other: Self) {
        *self = Self::Pair(Box::new(self.clone()), Box::new(other));

        self.reduce();
    }
}

pub struct Snailfish {
    nums: Vec<SnailfishNumber>,
}

impl crate::AdventOfCode for Snailfish {
    fn new(input: &str) -> Self {
        let nums = input.lines().map(SnailfishNumber::parse).collect();

        Self { nums }
    }

    fn part1(&self) -> u64 {
        let mut res = self.nums[0].to_owned();

        for num in &self.nums[1..] {
            res += num.to_owned();
        }

        res.magnitude()
    }

    fn part2(&self) -> u64 {
        let range = 0..self.nums.len();

        range
            .clone()
            .flat_map(|x| iter::repeat(x).zip(range.clone()))
            .filter(|(x, y)| x != y)
            .map(|(x, y)| {
                let mut x = self.nums[x].to_owned();
                x += self.nums[y].to_owned();
                x.magnitude()
            })
            .max()
            .unwrap()
    }
}
