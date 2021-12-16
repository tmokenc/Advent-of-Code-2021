struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(input: &str) -> Self {
        let mut iter = input.split(',').map(|v| v.parse::<usize>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        Self { x, y }
    }
}

struct Node {
    from: Coordinate,
    to: Coordinate,
}

fn count_overlaps<'a>(nodes: impl Iterator<Item = &'a Node>) -> u64 {
    let mut map = [[0u8; 1000]; 1000];

    for Node { from, to } in nodes {
        let Coordinate { mut x, mut y } = from;
        map[x][y] += 1;

        while x != to.x || y != to.y {
            if x < to.x {
                x += 1;
            } else if x > to.x {
                x -= 1;
            }

            if y < to.y {
                y += 1;
            } else if y > to.y {
                y -= 1;
            }

            map[x][y] += 1;
        }
    }

    map.into_iter().flatten().filter(|&v| v > 1).count() as u64
}

pub struct HydrothermalVenture {
    nodes: Vec<Node>,
}

impl crate::AdventOfCode for HydrothermalVenture {
    fn new(input: &str) -> Self {
        let nodes = input
            .lines()
            .map(|v| {
                let mut iter = v.split(" -> ");
                let from = iter.next().unwrap();
                let to = iter.next().unwrap();

                Node {
                    from: Coordinate::new(from),
                    to: Coordinate::new(to),
                }
            })
            .collect::<Vec<_>>();

        Self { nodes }
    }

    fn part1(&self) -> u64 {
        count_overlaps(
            self.nodes
                .iter()
                .filter(|Node { from, to }| from.x == to.x || from.y == to.y),
        )
    }

    fn part2(&self) -> u64 {
        count_overlaps(self.nodes.iter())
    }
}
