use std::convert::TryInto;

fn has_char_in_str(ch: char, s: &str) -> bool {
    s.chars().any(|v| v == ch)
}

fn get_off_chars(s: &str, full: &str) -> Vec<char> {
    full.chars().filter(|v| !has_char_in_str(*v, s)).collect()
}

fn parse_digits<'a>(s: &'a str) -> [&'a str; 10] {
    let mut digits = [""; 10];
    let mut unknown = Vec::new();

    for digit in s.split_whitespace() {
        let known_by_length = match digit.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => {
                unknown.push(digit);
                None
            }
        };

        if let Some(idx) = known_by_length {
            digits[idx] = digit;
        }
    }

    while let Some(digit) = unknown.pop() {
        if digit.len() == 6 {
            let off = get_off_chars(digit, digits[8])[0];

            if has_char_in_str(off, digits[1]) {
                digits[6] = digit;
                continue;
            }

            if has_char_in_str(off, digits[4]) {
                digits[0] = digit;
                continue;
            }

            digits[9] = digit;
            continue;
        }

        let [a, b]: [char; 2] = get_off_chars(digit, digits[8]).try_into().unwrap();

        if !has_char_in_str(a, digits[1]) && !has_char_in_str(b, digits[1]) {
            digits[3] = digit;
            continue;
        }

        if has_char_in_str(a, digits[4]) && has_char_in_str(b, digits[4]) {
            digits[2] = digit;
        } else {
            digits[5] = digit;
        }
    }

    digits
}

fn get_digit(digit: &str, digits: &[&str]) -> u8 {
    for i in 0..10 {
        if digit.len() != digits[i].len() {
            continue;
        }

        if digit.chars().all(|v| has_char_in_str(v, digits[i])) {
            return i as u8;
        }
    }

    0
}

pub struct SevenSegmentSearch {
    data: Vec<(String, String)>,
}

impl crate::AdventOfCode for SevenSegmentSearch {
    fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|s| {
                let mut iter = s.split('|').map(|v| v.trim().to_owned());
                (iter.next().unwrap(), iter.next().unwrap())
            })
            .collect();

        Self { data }
    }

    fn part1(&self) -> u64 {
        self.data
            .iter()
            .map(|v| v.1.split_whitespace())
            .flatten()
            .filter(|v| matches!(v.len(), 2 | 3 | 4 | 7))
            .count() as u64
    }

    fn part2(&self) -> u64 {
        self.data
            .iter()
            .map(|(wired, display)| {
                let digits = parse_digits(wired);
                let mut res = 0;

                for digit in display.split_whitespace() {
                    res *= 10;
                    res += get_digit(digit, &digits) as u64;
                }

                res
            })
            .sum()
    }
}
