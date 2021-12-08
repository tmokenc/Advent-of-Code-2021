fn lanternfish(timers: &[u8], day: u64) -> u64 {
    let mut data = [0u64; 9];
    
    for timer in timers {
        data[*timer as usize] += 1;
    }
    
    for _ in 0..day {
        let new = data[0];
        
        for i in 1..=8 {
            data[i-1] = data[i];
        }
        
        data[6] += new;
        data[8] = new;
    }
    
    data.iter().sum()
}

#[allow(dead_code)]
fn dummy_lanternfish(init: &[u8], mut day: u64) -> u64 {
    let mut fish_timers = init.to_owned();
    
    while day > 0 {
        let mut zero_count = 0;
        
        for timer in &mut fish_timers {
            if *timer == 0 {
                zero_count += 1;
                *timer = 6
            } else {
                *timer -= 1;
            }
        }
        
        if zero_count > 0 {
            fish_timers.extend(vec![8; zero_count]);
        }
        
        day -= 1;
    }
    
    
    fish_timers.len() as u64
}

fn main() -> std::io::Result<()> {
    let data = std::fs::read_to_string("./input/day6.txt")?
        .split(",")
        .filter_map(|v| v.parse::<u8>().ok())
        .collect::<Vec<u8>>();
        
    let example_data = std::fs::read_to_string("./input/example/day6.txt")?
        .split(",")
        .filter_map(|v| v.parse::<u8>().ok())
        .collect::<Vec<u8>>();
        
    println!("Day6 Part 1 Example: {}", day, lanternfish(&example_data, 80));
    println!("Day6 Part 1: {}", day, lanternfish(&data, 80));
    
    println!("Day6 Part 2 Example: {}", day, lanternfish(&example_data, 256));
    println!("Day6 Part 2: {}", day, lanternfish(&data, 256));
    
    Ok(())
}
