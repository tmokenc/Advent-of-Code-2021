mod part1 {
    pub fn sonar_sweep(data: &[i32]) -> usize {
        data.iter()
            .skip(1)
            .zip(data.iter())
            .filter(|(a, b)| a > b)
            .count()
    }
}

mod part2 {
    pub fn sonar_sweep(data: &[i32]) -> usize {
        data.windows(3)
            .skip(1)
            .zip(data.windows(3))
            .filter(|(a, b)| a.iter().sum::<i32>() > b.iter().sum())
            .count()
    }
}

fn main() -> std::io::Result<()> {
    let example_input = std::fs::read_to_string("./input/example/day1.txt")?
        .lines()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<_>>();

    let input = std::fs::read_to_string("./input/day1.txt")?
        .lines()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("Day1 Part 1 Example: {}", part1::sonar_sweep(&example_input));
    println!("Day1 Part 1: {}", part1::sonar_sweep(&input));

    println!("Day1 Part 2 Example: {}", part2::sonar_sweep(&example_input));
    println!("Day1 Part 2: {}", part2::sonar_sweep(&input));

    Ok(())
}
