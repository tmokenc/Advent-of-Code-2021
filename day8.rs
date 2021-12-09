mod part1 {
    pub fn seven_segment_search(input: &str) -> u64 {
        let mut count = 0;

        for line in input.lines() {
            let mut split = line.split('|').map(|v| v.trim());
            let _wired = split.next().unwrap();
            let display = split.next().unwrap();

            for digit in display.split_whitespace() {
                if matches!(digit.len(), 2 | 3 | 4 | 7) {
                    count += 1;
                }
            }
        }

        count
    }
}

mod part2 {
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

    pub fn seven_segment_search(input: &str) -> u64 {
        let mut total = 0;

        for line in input.lines() {
            let mut split = line.split('|').map(|v| v.trim());
            let wired = split.next().unwrap();
            let display = split.next().unwrap();
            let digits = parse_digits(wired);
            let mut res = 0;

            for digit in display.split_whitespace() {
                res *= 10;
                res += get_digit(digit, &digits) as u64;
            }

            total += res;
        }

        total
    }
}
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./input/day8.txt")?;
    let example_input = std::fs::read_to_string("./input/example/day8.txt")?;

    println!("Day8 Part 1 Example: {}", part1::seven_segment_search(&example_input));
    println!("Day8 Part 1: {}", part1::seven_segment_search(&input));

    println!("Day8 Part 2 Example: {}", part2::seven_segment_search(&example_input));
    println!("Day8 Part 2: {}", part2::seven_segment_search(&input));

    Ok(())
}
