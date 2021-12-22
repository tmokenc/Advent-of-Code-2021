// Just a port of this https://www.reddit.com/r/adventofcode/comments/rlxhmg/2021_day_22_solutions/hpjbx3t/
// Because it's extremely cool

use std::ops::RangeInclusive;

#[derive(Clone)]
struct Cube {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

fn line_overlap(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    let start = a.start().max(b.start());
    let end = a.end().min(b.end());
    *start..=*end
}

impl Cube {
    fn volume(&self) -> u64 {
        let x = self.x.end() - self.x.start() + 1;
        let y = self.y.end() - self.y.start() + 1;
        let z = self.z.end() - self.z.start() + 1;
        
        (x * y * z)  as u64
    }
    
    fn overlap(&self, cubes: &[Self]) -> u64 {
        let mut result = 0;
        
        for (cube, i) in cubes.iter().zip(1..) {
            let overlap = Self {
                x: line_overlap(&self.x, &cube.x),
                y: line_overlap(&self.y, &cube.y),
                z: line_overlap(&self.z, &cube.z),
            };
            
            if !overlap.x.is_empty() && !overlap.y.is_empty() && !overlap.z.is_empty() {
                result += overlap.volume() - overlap.overlap(cubes.get(i..).unwrap());
            }
        }
        
        result
    }
}

#[derive(Clone)]
struct Instruction {
    on: bool,
    cube: Cube,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let mut iter = line.split_whitespace();
        
        let on = iter.next().unwrap() == "on";
        let mut lines = iter
            .next()
            .unwrap()
            .split(',')
            .map(|v| {
                let mut iter = v[2..]
                    .split("..")
                    .map(|v| v.parse::<i64>().unwrap());
                
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();
                a.min(b)..=a.max(b)
            });
        
        let cube = Cube {
            x: lines.next().unwrap(),
            y: lines.next().unwrap(),
            z: lines.next().unwrap(),
        };
        
        Self {
            on, cube
        }
    }
}

fn count_light(instructions: &[Instruction]) -> u64 {
    let mut lights = 0;
    let mut cubes = Vec::new();
    
    for instruction in instructions.iter().rev() {
        let cube = &instruction.cube;
        
        if instruction.on {
            lights += cube.volume() - cube.overlap(&cubes);
        }
        
        cubes.push(cube.to_owned());
    }
    
    lights as u64
}

pub struct ReactorReboot {
    steps: Vec<Instruction>
}

impl crate::AdventOfCode for ReactorReboot {
    fn new(input: &str) -> Self {
        let steps = input
            .lines()
            .map(Instruction::parse)
            .collect();
        
        Self { steps }
    }
    
    fn part1(&self) -> u64 {
        let instructions = self
            .steps
            .iter()
            .filter(|v| {
                let check_x = *v.cube.x.start() >= -50 && *v.cube.x.end() <= 50;
                let check_y = *v.cube.y.start() >= -50 && *v.cube.y.end() <= 50;
                let check_z = *v.cube.z.start() >= -50 && *v.cube.z.end() <= 50;
                
                check_x && check_y && check_z
            })
            .cloned()
            .collect::<Vec<_>>();
            
        count_light(&instructions)
    }
    
    fn part2(&self) -> u64 {
        count_light(&self.steps)
    }
}