mod part1 {
    pub fn dive(input: &str) -> i64 {
        let mut position = 0i64;
        let mut depth = 0i64;
        
        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let direction = iter.next().unwrap_or_default();
            let num = iter.next().and_then(|v| v.parse::<i64>().ok()).unwrap_or_default();
            
            match direction {
                "forward" => position += num,
                "up" => depth -= num,
                "down" => depth += num,
                _ => ()
            }
        }
        
        position * depth
    }
}

mod part2 {
    pub fn dive(input: &str) -> i64 {
        let mut position = 0i64;
        let mut depth = 0i64;
        let mut aim = 0i64;
        
        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let direction = iter.next().unwrap_or_default();
            let num = iter.next().and_then(|v| v.parse::<i64>().ok()).unwrap_or_default();
            
            match direction {
                "forward" => {
                    position += num;
                    depth += aim * num;
                }
                "up" => aim -= num,
                "down" => aim += num,
                _ => ()
            }
        }
        
        position * depth
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./input/day2.txt")?;
    let example_input = std::fs::read_to_string("./input/example/day2.txt")?;

    println!("Day8 Part 1 Example: {}", part1::dive(&example_input));
    println!("Day8 Part 1: {}", part1::dive(&input));

    println!("Day8 Part 2 Example: {}", part2::dive(&example_input));
    println!("Day8 Part 2: {}", part2::dive(&input));
    
    Ok(())
}