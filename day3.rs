pub fn append_binary(num: u64, b: bool) -> u64 {
    (num << 1) | if b { 1 } else { 0 }
}

mod part1 {
    pub fn binary_diagnostic(input: &str) -> u64 {
        let mut lines = input.lines();
        let mut data = lines
            .next()
            .unwrap()
            .bytes()
            .map(|v| v as i32 - 48)
            .collect::<Vec<i32>>();
        
        lines.flat_map(|v| v.bytes().enumerate()).for_each(|(i, v)| {
            if v == 48 {
                data[i] -= 1;
            } else if v == 49 {
                data[i] += 1;
            }
        });
            
        let gamma = data.iter().map(|&v| v > 0).fold(0, super::append_binary);
        let elipsion = data.iter().map(|&v| v < 1).fold(0, super::append_binary);
        
        gamma * elipsion
    }
}

mod part2 {
    pub fn binary_diagnostic(input: &str) -> u64 {
        0
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