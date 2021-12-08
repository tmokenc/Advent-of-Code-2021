fn the_treachery_of_whales(input: &[i32]) -> u64 {
    if input.is_empty() {
        return 0;
    }
    
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    
    for i in input {
        if *i < min {
            min = *i;
        } else if *i > max {
            max = *i;
        }
    }
    
    (min..=max)
        .map(|v| {
            input.iter().copied().map(|x| (v - x).abs()).sum()
        })
        .min()
        .unwrap_or(0i32) as u64
}

fn the_treachery_of_whales_part2(input: &[i32]) -> u64 {
    if input.is_empty() {
        return 0;
    }
    
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    
    for i in input {
        if *i < min {
            min = *i;
        } else if *i > max {
            max = *i;
        }
    }
    
    (min..=max)
        .map(|v| {
            input.iter().copied().map(|x| {
                let steps = (v - x).abs();
                (0..=steps).sum::<i32>()
            }).sum()
        })
        .min()
        .unwrap_or(0i32) as u64
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./input/day7.txt")?
        .split(",")
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<_>>();
    
    let example_input = std::fs::read_to_string("./input/example/day7.txt")?
        .split(",")
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<_>>();
    
    println!("Day7 Part 1 Example: {}", the_treachery_of_whales(&example_input));
    println!("Day7 Part 1: {}", the_treachery_of_whales(&input));
    
    println!("Day7 Part 2 Example: {}", the_treachery_of_whales_part2(&example_input));
    println!("Day7 Part 2: {}", the_treachery_of_whales_part2(&input));
    
    Ok(())
} 
