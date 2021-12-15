enum Direction {
    Forward,
    Up,
    Down,
}

struct Action {
    value: u64,
    direction: Direction,
}

pub struct Dive {
    actions: Vec<Action>,
}

impl crate::AdventOfCode for Dive {
    fn new(input: &str) -> Self {
        let actions = input.lines().map(|line| {
            let mut iter = line.split_whitespace();
            let direction = match iter.next().unwrap_or_default() {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => unreachable!(),
            };
            let value = iter.next().and_then(|v| v.parse::<u64>().ok()).unwrap_or_default();
            
            Action { value, direction }
        });
            
        Self { 
            actions: actions.collect()
        }
    }
    
    fn part1(&self) -> u64 {
        let mut position = 0u64;
        let mut depth = 0u64;
        
        for action in &self.actions {
            match action.direction {
                Direction::Forward => position += action.value,
                Direction::Up => depth -= action.value,
                Direction::Down => depth += action.value,
            }
        }
        
        position * depth
    }
    
    fn part2(&self) -> u64 {
        let mut position = 0u64;
        let mut depth = 0u64;
        let mut aim = 0u64;
        
        for action in &self.actions {
            match action.direction {
                Direction::Forward => {
                    position += action.value;
                    depth += aim * action.value;
                }
                Direction::Up => aim -= action.value,
                Direction::Down => aim += action.value,
            }
        }
        
        position * depth
    }
}