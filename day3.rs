mod part1 {
    pub fn append_binary(num: u64, b: bool) -> u64 {
        (num << 1) | b as u64
    }

    pub fn binary_diagnostic(input: &str) -> u64 {
        let mut lines = input.lines();
        let mut data = lines
            .next()
            .unwrap()
            .bytes()
            .map(|v| v as i32 - 48)
            .collect::<Vec<i32>>();

        lines
            .flat_map(|v| v.bytes().enumerate())
            .for_each(|(i, v)| {
                if v == 48 {
                    data[i] -= 1;
                } else if v == 49 {
                    data[i] += 1;
                }
            });

        let gamma = data.iter().map(|&v| v > 0).fold(0, append_binary);
        let elipsion = data.iter().map(|&v| v < 1).fold(0, append_binary);

        gamma * elipsion
    }
}

mod part2 {
    fn binary_slice_to_num(s: &[u8]) -> u64 {
        s.iter().fold(0, |v, x| v << 1 | *x as u64)
    }

    pub fn binary_diagnostic(input: &str) -> u64 {
        let lines = input
            .lines()
            .map(|v| v.bytes().map(|v| v - 48).collect::<Vec<u8>>())
            .collect::<Vec<_>>();

        let bit_criteria = |greater: bool| {
            let mut current_pos = 0;
            let mut indexes = (0..lines.len()).collect::<Vec<usize>>();

            while indexes.len() > 1 && current_pos < lines[0].len() {
                let score = indexes
                    .iter()
                    .map(|v| lines[*v][current_pos])
                    .fold(0, |v, x| v + if x == 1 { 1 } else { -1 });

                let match_bit = if score == 0 {
                    greater as u8
                } else {
                    score.clamp(0, 1) as u8 ^ !greater as u8
                };

                indexes.retain(|v| lines[*v][current_pos] == match_bit);
                current_pos += 1;
            }

            binary_slice_to_num(&lines[indexes[0]])
        };

        let oxygen = bit_criteria(true);
        let co2 = bit_criteria(false);

        oxygen * co2
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./input/day3.txt")?;
    let example_input = std::fs::read_to_string("./input/example/day3.txt")?;

    println!("Day3 Part 1 Example: {}", part1::binary_diagnostic(&example_input));
    println!("Day3 Part 1: {}", part1::binary_diagnostic(&input));

    println!("Day3 Part 2 Example: {}", part2::binary_diagnostic(&example_input));
    println!("Day3 Part 2: {}", part2::binary_diagnostic(&input));

    Ok(())
}
