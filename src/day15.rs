use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

type Coord = (usize, usize);

#[derive(Clone, Eq)]
struct QueueNode {
    coord: Coord,
    value: u64,
}

impl PartialEq for QueueNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn next_coords(coord: &Coord, max_x: usize, max_y: usize) -> Vec<Coord> {
    let mut coords = Vec::new();

    if coord.0 > 0 {
        coords.push((coord.0 - 1, coord.1));
    }

    if coord.1 > 0 {
        coords.push((coord.0, coord.1 - 1));
    }

    if coord.0 < max_x {
        coords.push((coord.0 + 1, coord.1));
    }

    if coord.1 < max_y {
        coords.push((coord.0, coord.1 + 1));
    }

    coords
}

#[derive(Clone)]
pub struct Chiton {
    map: Vec<Vec<u8>>,
}

impl Chiton {
    fn get(&self, coord: &Coord) -> u8 {
        let x_size = self.map.len();
        let y_size = self.map[0].len();
        let mx = (coord.0 / x_size) as u8;
        let my = (coord.1 / y_size) as u8;
        let x = coord.0 % x_size;
        let y = coord.1 % y_size;
        let mut risk = self.map[x][y] + mx + my;

        while risk > 9 {
            risk -= 9;
        }

        risk
    }
    
    fn cal_lowest_risk(&self, dest: Coord) -> u64 {
        let mut queue: BinaryHeap<QueueNode> = Default::default();
        let mut visited: HashSet<Coord> = Default::default();
        
        queue.push(QueueNode {
            coord: (0, 0),
            value: 0,
        });
        
        while let Some(QueueNode { coord, value }) = queue.pop() {
            if coord == dest {
                return value;
            }
            
            if !visited.insert(coord.clone()) {
                continue;
            }
            
            for next_coord in next_coords(&coord, dest.0, dest.1) {
                queue.push(QueueNode {
                    value: value + self.get(&next_coord) as u64,
                    coord: next_coord,
                });
            }
        }
        
        unreachable!()
    }
}

impl crate::AdventOfCode for Chiton {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|v| v.bytes().map(|x| x - 48).collect())
            .collect();

        Self { map }
    }

    fn part1(&self) -> u64 {
        let last_x_idx = self.map.len() - 1;
        let last_y_idx = self.map[0].len() - 1;
        self.cal_lowest_risk((last_x_idx, last_y_idx))
    }

    fn part2(&self) -> u64 {
        let last_x_idx = self.map.len() * 5 - 1;
        let last_y_idx = self.map[0].len() * 5 - 1;
        self.cal_lowest_risk((last_x_idx, last_y_idx))
    }
}
