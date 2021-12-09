pub fn parse_height_map(s: &str) -> Vec<Vec<u8>> {
    let mut res = Vec::new();
    
    for line in s.lines() {
        let y = line.chars().map(|v| v.to_digit(10).unwrap() as u8).collect();
        res.push(y);
    }
    
    res
}

mod part1 {
    pub fn smoke_basin(input: &str) -> u64 {
        let map = super::parse_height_map(input);
        let mut risk = 0;
        
        if map.is_empty() || map[0].is_empty() {
            return 0;
        }
        
        for (y, row) in map.iter().enumerate() {
            for (x, value) in row.iter().copied().enumerate() {
                if value > 8 {
                    continue
                }
                
                let mut to_check = Vec::new();
                
                if x != 0 { 
                    to_check.push(row[x-1]);
                }
                
                if y != 0 {
                    to_check.push(map[y-1][x]);
                }
                
                if let Some(t) = row.get(x+1) {
                    to_check.push(*t);
                }
                
                if let Some(row) = map.get(y+1) {
                    to_check.push(row[x]);
                }
                
                if to_check.into_iter().all(|v| value < v) {
                    risk += value as u64 + 1;
                }
            }
        }
        
        risk
    }
}

mod part2 {
    use std::collections::HashSet;
    
    pub fn smoke_basin(input: &str) -> u64 {
        let map = super::parse_height_map(input);
        
        let mut visited = HashSet::new();
        let mut largest_3 = [1u64; 3];
        
        if map.get(0).filter(|v| !v.is_empty()).is_none() {
            return 0;
        }
        
        let x_len = map[0].len();
        let y_len = map.len();
        
        let next_moves = |x, y| {
            let mut moves = Vec::new();
            
            if x != 0 { 
                moves.push((x-1, y));
            }
            
            if y != 0 {
                moves.push((x, y-1));
            }
            
            if x != x_len - 1 {
                moves.push((x+1, y));
            }
            
            if y != y_len - 1 {
                moves.push((x, y+1));
            }
            
            moves
        };
            
        let iter = (0..x_len)
            .map(|x| std::iter::repeat(x).zip(0..y_len))
            .flatten()
            .filter(|(x, y)| map[*y][*x] < 9);
        
        for (x, y) in iter {
            if !visited.insert((x, y)) {
                continue;
            }
            
            let mut result = 1;
            let mut possible_moves = next_moves(x, y);
            
            while let Some((x, y)) = possible_moves.pop() {
                if !visited.insert((x, y)) || map[y][x] == 9 {
                    continue;
                }
                
                result += 1;
                possible_moves.extend(next_moves(x, y));
            }
            
            if result > largest_3[0] {
                largest_3[0] = result;
                largest_3.sort_unstable();
            }
        }
        
        largest_3.iter().product()
    }
}


fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./input/day9.txt")?;
    let example_input = std::fs::read_to_string("./input/example/day9.txt")?;

    println!("Day9 Part 1 Example: {}", part1::smoke_basin(&example_input));
    println!("Day9 Part 1: {}", part1::smoke_basin(&input));

    println!("Day9 Part 2 Example: {}", part2::smoke_basin(&example_input));
    println!("Day9 Part 2: {}", part2::smoke_basin(&input));

    Ok(())
}