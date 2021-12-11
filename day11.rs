pub fn parse_input(input: &str) -> (usize, Vec<u8>) {
    input.lines().fold((0, Vec::new()), |(len, mut arr), x| {
        let len = x.len();
        arr.extend(x.chars().map(|v| v.to_digit(10).unwrap()));
        (len, arr)
    })
}

mod part1 {
    pub fn dumbo_octopus(input: &str, step: u32) -> u64 {
        let mut flashes = 0;
        let (row_len, mut arr) = parse_input(input);
        
        for _ in 0..step {
            
            for (light, idx) in arr.iter_mut().zip(0isize..) {
                if light > 9 {
                    continue;
                }
                
                let indexes = [
                    idx - row_len - 1,
                    idx - row_len,
                    idx - row_len + 1,
                    idx - 1,
                    idx + 1,
                    idx + row_len - 1,
                    idx + row_len,
                    idx + row_len + 1,
                ];
                
                *light += indexes
                    .iter()
                    .filter_map(|v| usize::try_from(**v).ok())
                    .filter_map(|v| arr.get(*v))
                    .filter(|v| v < 10)
                    .count()
            }
            
            for light in &mut arr {
                if *light > 9 {
                    *light = 0;
                    flashes += 1;
                }
            }
        }
        
        flashes
    }
}

mod part1 {
    pub fn dumbo_octopus(input: &str) -> u64 {
        input.len() as u64
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./input/day11.txt")?;
    let example_input = std::fs::read_to_string("./input/example/day11.txt")?;

    println!("Day3 Part 1 Example: {}", part1::binary_diagnostic(&example_input, 100));
    println!("Day3 Part 1: {}", part1::binary_diagnostic(&input, 100));

    println!("Day3 Part 2 Example: {}", part2::binary_diagnostic(&example_input));
    println!("Day3 Part 2: {}", part2::binary_diagnostic(&input));

    Ok(())
}