pub fn parse_char_to_input(ch: char) -> (usize, bool) {
    match ch {
        '(' => (0, false),
        '[' => (1, false),
        '{' => (2, false),
        '<' => (3, false),

        ')' => (0, true),
        ']' => (1, true),
        '}' => (2, true),
        '>' => (3, true),

        _ => unreachable!(),
    }
}

mod part1 {
    pub fn syntax_scoring(input: &str) -> u64 {
        const MULTIPLYER: [u64; 4] = [3, 57, 1197, 25137];
        let mut score = [0u64; 4];

        for line in input.lines() {
            let mut states = Vec::new();
            for ch in line.chars() {
                let (idx, closing) = super::parse_char_to_input(ch);

                if closing {
                    match states.pop() {
                        Some(i) if i == idx => continue,
                        _ => {
                            score[idx] += 1;
                            break;
                        }
                    }
                } else {
                    states.push(idx);
                }
            }
        }

        score.iter().zip(MULTIPLYER).map(|(s, m)| s * m).sum()
    }
}

mod part2 {
    pub fn syntax_scoring(input: &str) -> u64 {
        let mut scores = Vec::new();

        'line: for line in input.lines() {
            let mut states = Vec::new();

            for ch in line.chars() {
                let (idx, closing) = super::parse_char_to_input(ch);

                if closing {
                    match states.pop() {
                        Some(i) if i == idx => continue,
                        _ => continue 'line,
                    }
                } else {
                    states.push(idx);
                }
            }

            let mut score = 0;

            while let Some(i) = states.pop() {
                score *= 5;
                score += i as u64 + 1;
            }

            scores.push(score);
        }

        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./input/day10.txt")?;
    let example_input = std::fs::read_to_string("./input/example/day10.txt")?;

    println!("Day10 Part 1 Example: {}", part1::syntax_scoring(&example_input));
    println!("Day10 Part 1: {}", part1::syntax_scoring(&input));

    println!("Day10 Part 2 Example: {}", part2::syntax_scoring(&example_input));
    println!("Day10 Part 2: {}", part2::syntax_scoring(&input));

    Ok(())
}
