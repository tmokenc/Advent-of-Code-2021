use std::collections::{HashMap, HashSet};
use std::iter;

type Coord = (usize, usize);

// #[derive(Clone)]
// struct QueueNode {
//     coord: Coord,
//     value: u64,
// }
//
// use std::cmp::Ordering;
// use std::collections::BTreeSet;
//
// impl PartialEq for QueueNode {
//     fn eq(&self, other: &Self) -> bool {
//         self.coord == other.coord
//     }
// }
//
// impl Eq for QueueNode {}
//
// impl Ord for QueueNode {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.value.cmp(&self.value)
//     }
// }
//
// impl PartialOrd for QueueNode {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// fn pop_first(queue: &mut BTreeSet<QueueNode>) -> Option<QueueNode> {
//     let first = queue.iter().next()?.clone();
//     queue.take(&first)
// }

fn take_min(queue: &mut HashSet<Coord>, dist: &HashMap<Coord, u64>) -> Option<Coord> {
    let min = queue.iter().min_by_key(|v| dist[v])?.to_owned();
    queue.take(&min)
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
    // dijkstra
    fn cal_lowest_risk(&self) -> u64 {
        let mut queue: HashSet<Coord> = HashSet::new();
        let mut dist: HashMap<Coord, u64> = HashMap::new();

        let last_x_idx = self.map.len() - 1;
        let last_y_idx = self.map[0].len() - 1;

        (0..=last_x_idx)
            .flat_map(|x| iter::repeat(x).zip(0..=last_y_idx))
            .for_each(|c| {
                dist.insert(c.clone(), u64::MAX);
                queue.insert(c);
            });

        dist.insert((0, 0), 0);

        while let Some(coord) = take_min(&mut queue, &dist) {
            for next_coord in next_coords(&coord, last_x_idx, last_y_idx) {
                let alt = dist[&coord] + self.map[next_coord.0][next_coord.1] as u64;

                if alt < dist[&next_coord] {
                    dist.insert(next_coord, alt);
                }
            }
        }

        dist.get(&(last_x_idx, last_y_idx)).copied().unwrap()
    }

    // fn cal_lowest_risk(&self) -> u64 {
    //     let mut queue: BTreeSet<QueueNode> = BTreeSet::new();
    //     let mut dist: HashMap<Coord, u64> = HashMap::new();

    //     let last_x_idx = self.map.len() - 1;
    //     let last_y_idx = self.map[0].len() - 1;

    //     (0..=last_x_idx)
    //         .flat_map(|x| iter::repeat(x).zip(0..=last_y_idx))
    //         .for_each(|c| {
    //             dist.insert(c.clone(), u64::MAX);
    //         });

    //     dist.insert((0, 0), 0);
    //     queue.insert(QueueNode {
    //         coord: (0, 0),
    //         value: 0,
    //     });

    //     while let Some(QueueNode { coord, .. }) = pop_first(&mut queue) {
    //         for next_coord in next_coords(&coord, last_x_idx, last_y_idx) {
    //             let alt = dist[&coord] + self.map[next_coord.0][next_coord.1] as u64;

    //             if alt < dist[&next_coord] {
    //                 dist.insert(next_coord.clone(), alt);
    //                 queue.insert(QueueNode {
    //                     coord: next_coord,
    //                     value: alt,
    //                 });
    //             }
    //         }
    //     }

    //     dist.get(&(last_x_idx, last_y_idx)).copied().unwrap()
    // }
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
        self.cal_lowest_risk()
    }

    fn part2(&self) -> u64 {
        let mut new_map = self.clone();

        // new_map.map.resize(self.map.len() * 5, Vec::new());

        new_map.cal_lowest_risk()
    }
}
