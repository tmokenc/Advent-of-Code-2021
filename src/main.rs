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
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
//mod day23;
mod day24;
mod day25;

const NAMES: &[&str] = &[
    "Sonar Sweep",
    "Dive!",
    "Binary Diagnostic",
    "Giant Squid",
    "Hydrothermal Venture",
    "Lanternfish",
    "The Treachery of Whales",
    "Seven Segment Search",
    "Smoke Basin",
    "Syntax Scoring",
    "Dumbo Octopus",
    "Passage Pathing",
    "Transparent Origami",
    "Extended Polymerization",
    "Chiton",
    "Packet Decoder",
    "Trick Shot",
    "Snailfish",
    "Beacon Scanner",
    "Trench Map",
    "Dirac Dice",
    "Reactor Reboot",
    "Amphipod",
    "Arithmetic Logic Unit",
    "Sea Cucumber",
];

use humantime::format_duration;
use owo_colors::OwoColorize as _;
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
        println!(
            "Input parsed in {}",
            format_duration(self.parse_time).bright_magenta()
        );

        let (res, timed) = time(|| self.app.part1());
        println!(
            "Part 1: {} ({}) (total {})",
            res,
            format_duration(timed).cyan(),
            format_duration(timed + self.parse_time).bright_cyan(),
        );

        let (res, timed) = time(|| self.app.part2());
        println!(
            "Part 2: {} ({}) (total {})",
            res,
            format_duration(timed).cyan(),
            format_duration(timed + self.parse_time).bright_cyan(),
        );

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
            01 => Self::new::<day01::SonarSweep>(day),
            02 => Self::new::<day02::Dive>(day),
            03 => Self::new::<day03::BinaryDiagnostic>(day),
            04 => Self::new::<day04::GiantSquid>(day),
            05 => Self::new::<day05::HydrothermalVenture>(day),
            06 => Self::new::<day06::Lanternfish>(day),
            07 => Self::new::<day07::TheTreacheryOfWhales>(day),
            08 => Self::new::<day08::SevenSegmentSearch>(day),
            09 => Self::new::<day09::SmokeBasin>(day),
            10 => Self::new::<day10::SyntaxScoring>(day),
            11 => Self::new::<day11::DumboOctopus>(day),
            12 => Self::new::<day12::PassagePathing>(day),
            13 => Self::new::<day13::TransparentOrigami>(day),
            14 => Self::new::<day14::ExtendedPolymerization>(day),
            15 => Self::new::<day15::Chiton>(day),
            16 => Self::new::<day16::PacketDecoder>(day),
            17 => Self::new::<day17::TrickShot>(day),
            18 => Self::new::<day18::Snailfish>(day),
            19 => Self::new::<day19::BeaconScanner>(day),
            20 => Self::new::<day20::TrenchMap>(day),
            21 => Self::new::<day21::DiracDice>(day),
            22 => Self::new::<day22::ReactorReboot>(day),
            // day23
            24 => Self::new::<day24::ArithmeticLogicUnit>(day),
            25 => Self::new::<day25::SeaCucumber>(day),
            _ => None,
        }
    }

    fn run(&self) {
        let title = format!("DAY {} - {}", self.day, NAMES[self.day as usize - 1]);
        println!("{}", title.bold());

        println!("Example");
        self.example.run();

        println!("Solution");
        self.solution.run();
    }
}

fn main() {
    let mut days = std::env::args()
        .skip(1)
        .filter_map(|v| v.parse::<u8>().ok())
        .collect::<Vec<_>>();

    if days.is_empty() {
        days = (1..=25).collect();
    }

    days.into_iter()
        .filter_map(|d| Solutions::get(d))
        .for_each(|v| v.run());
}
