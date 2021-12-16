mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

use humantime::format_duration;
use std::path::Path;
use std::time::{Duration, Instant};

pub trait AdventOfCode {
    fn new(input: &str) -> Self
    where
        Self: Sized;

    fn part1(&self) -> u64;
    fn part2(&self) -> u64;
}

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
            example,
            solution,
            day,
        })
    }

    fn get(day: u8) -> Option<Self> {
        match day {
            01 => Self::new::<day01::SonarSweep>(1),
            02 => Self::new::<day02::Dive>(2),
            03 => Self::new::<day03::BinaryDiagnostic>(3),
            04 => Self::new::<day04::GiantSquid>(4),
            05 => Self::new::<day05::HydrothermalVenture>(5),
            06 => Self::new::<day06::Lanternfish>(6),
            07 => Self::new::<day07::TheTreacheryOfWhales>(7),
            08 => Self::new::<day08::SevenSegmentSearch>(8),
            09 => Self::new::<day09::SmokeBasin>(9),
            10 => Self::new::<day10::SyntaxScoring>(10),
            11 => Self::new::<day11::DumboOctopus>(11),
            12 => Self::new::<day12::PassagePathing>(12),
            13 => Self::new::<day13::TransparentOrigami>(13),
            14 => Self::new::<day14::ExtendedPolymerization>(14),
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
