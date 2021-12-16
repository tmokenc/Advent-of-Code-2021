use std::collections::{HashMap, HashSet};

type Node = HashSet<String>;

pub struct PassagePathing {
    start: Node,
    nodes: HashMap<String, Node>,
    end: Node,
}

impl crate::AdventOfCode for PassagePathing {
    fn new(input: &str) -> Self {
        let mut start: Node = Default::default();
        let mut end: Node = Default::default();
        let mut nodes: HashMap<String, Node> = Default::default();

        for line in input.lines() {
            let mut iter = line.split('-');
            let lhs = iter.next().unwrap();
            let rhs = iter.next().unwrap();

            match (lhs, rhs) {
                ("start", v) | (v, "start") => {
                    start.insert(String::from(v));
                }

                ("end", v) | (v, "end") => {
                    end.insert(String::from(v));
                }

                (a, b) => {
                    nodes
                        .entry(String::from(a))
                        .or_insert_with(Default::default)
                        .insert(String::from(b));

                    nodes
                        .entry(String::from(b))
                        .or_insert_with(Default::default)
                        .insert(String::from(a));
                }
            }
        }

        Self { start, end, nodes }
    }

    fn part1(&self) -> u64 {
        let mut found_exist = 0;
        let mut next_path = self.start.iter().map(|v| vec![v]).collect::<Vec<_>>();

        while let Some(path) = next_path.pop() {
            let last = path.last().unwrap();

            if self.end.contains(*last) {
                found_exist += 1;
            }

            for next in self.nodes.get(*last).unwrap() {
                if next.chars().next().unwrap().is_lowercase() {
                    if path.iter().any(|&v| v == next) {
                        continue;
                    }
                }

                let mut new_path = path.clone();
                new_path.push(next);
                next_path.push(new_path);
            }
        }

        found_exist
    }

    fn part2(&self) -> u64 {
        let mut found_exist = 0;
        let mut next_path = self
            .start
            .iter()
            .map(|v| (vec![v], false))
            .collect::<Vec<_>>();

        while let Some((path, visited_twice)) = next_path.pop() {
            let last = path.last().unwrap();

            if self.end.contains(*last) {
                found_exist += 1;
            }

            for next in self.nodes.get(*last).unwrap() {
                let mut visited_twice = visited_twice;

                if next.chars().next().unwrap().is_lowercase() {
                    if path.iter().any(|&v| v == next) {
                        if visited_twice {
                            continue;
                        }

                        visited_twice = true;
                    }
                }

                let mut new_path = path.clone();
                new_path.push(next);
                next_path.push((new_path, visited_twice));
            }
        }

        found_exist
    }
}
