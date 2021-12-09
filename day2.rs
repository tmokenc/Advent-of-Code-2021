mod part1 {
  pub fn dive(input: &str) -> u64 {
    0
  }
}

mod part2 {
  pub fn dive(input: &str) -> u64 {
    0
  }
}

fn main() -> std::io::Result<()>
    let input = std::fs::read_to_string("./input/day2.txt")?;
    let example_input = std::fs::read_to_string("./input/example/day2.txt")?;

    println!("Day8 Part 1 Example: {}", part1::dive(&example_input));
    println!("Day8 Part 1: {}", part1::dive(&input));

    println!("Day8 Part 2 Example: {}", part2::dive(&example_input));
    println!("Day8 Part 2: {}", part2::dive(&input));
    
    Ok(())
}