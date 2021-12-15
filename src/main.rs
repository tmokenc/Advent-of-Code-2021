mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use std::time::{Duration, Instant};
use std::path::Path;
use humantime::format_duration;

fn time<T>(f: impl Fn() -> T) -> (T, Duration) {
    let start = Instant::now();
    let res = f();
    let time = start.elapsed();
    
    (res, time)
}

struct Solution {
    app: Box<dyn AdventOfCode>,
    parse_time: Duration,
}

impl Solution {
    fn new<AoC: AdventOfCode + 'static>(input: &str) -> Self {
        let (app, parse_time) = time(|| AoC::new(input));
        
        Self {
            app: Box::new(app) as Box<_>,
            parse_time,
        }
    }
}

impl Solution {
    fn run(&self) {
        println!("Input parsed in {}", format_duration(self.parse_time));
        
        let (res, timed) = time(|| self.app.part1());
        println!("Part 1: {} ({})", res, format_duration(timed));
        
        let (res, timed) = time(|| self.app.part2());
        println!("Part 2: {} ({})", res, format_duration(timed));
        
        println!("");
    }
}

struct Solutions {
    example: Solution,
    solution: Solution,
    day: u8,
}

impl Solutions {
    fn new<AoC: AdventOfCode + 'static>(day: u8) -> Option<Self> {
        let input_name = format!("day{}.txt", day);
        let example_path = Path::new("./input/example/").join(&input_name);
        let input_path = Path::new("./input/").join(&input_name);
        
        let example_input = std::fs::read_to_string(example_path).ok()?;
        let input = std::fs::read_to_string(input_path).ok()?;
        
        let example = Solution::new::<AoC>(&example_input);
        let solution = Solution::new::<AoC>(&input);
        
        Some(Self {
            example, solution, day
        })
    }
    
    fn get(day: u8) -> Option<Self> {
        match day {
            1 => Self::new::<day1::SonarSweep>(1),
            2 => Self::new::<day2::Dive>(2),
            3 => Self::new::<day3::BinaryDiagnostic>(3),
            4 => Self::new::<day4::GiantSquid>(4),
            5 => Self::new::<day5::HydrothermalVenture>(5),
            6 => Self::new::<day6::Lanternfish>(6),
            7 => Self::new::<day7::TheTreacheryOfWhales>(7),
            8 => Self::new::<day8::SevenSegmentSearch>(8),
            9 => Self::new::<day9::SmokeBasin>(9),
            10 => Self::new::<day10::SyntaxScoring>(10),
            11 => Self::new::<day11::DumboOctopus>(11),
            _ => None,
        }
    }
    
    fn run(&self) {
        println!("DAY {}", self.day);
        
        println!("Example");
        self.example.run();
        
        println!("Solution");
        self.solution.run();
    }
}

pub trait AdventOfCode {
    fn new(input: &str) -> Self where Self: Sized;
    fn part1(&self) -> u64;
    fn part2(&self) -> u64;
}

fn main() -> Result<(), String> {
    let day = std::env::args().nth(1).and_then(|v| v.parse::<u8>().ok());
    
    match day {
        Some(d) => Solutions::get(d)
            .ok_or(format!("Cannot get any solution for day {}", d))?
            .run(),
        None => (1..=25)
            .filter_map(|d| Solutions::get(d))
            .for_each(|v| v.run()),
    }
    
    Ok(())
}
