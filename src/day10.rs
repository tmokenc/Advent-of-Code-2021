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

pub struct SyntaxScoring {
    data: Vec<Vec<char>>,
}

impl crate::AdventOfCode for SyntaxScoring {
    fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|v| v.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
            
        Self { data }
    }
    
    fn part1(&self) -> u64 {
        const MULTIPLIER: [u64; 4] = [3, 57, 1197, 25137];
        let mut score = [0u64; 4];

        for line in &self.data {
            let mut states = Vec::new();
            for ch in line {
                let (idx, closing) = parse_char_to_input(*ch);

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

        score.iter().zip(MULTIPLIER).map(|(s, m)| s * m).sum()
    }
    
    fn part2(&self) -> u64 {
        let mut scores = Vec::new();

        'line: for line in &self.data {
            let mut states = Vec::new();

            for ch in line {
                let (idx, closing) = parse_char_to_input(*ch);

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
