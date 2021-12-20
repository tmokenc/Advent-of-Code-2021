use std::collections::HashSet;

type Point3D = (i16, i16, i16);
type Scanner = HashSet<Point3D>;

/*
const FACE [
    (x, y, z),
    (x, -y, -z)
    (x, -z, y),
    (-y, -z, x),
    (y, -z, -x),
    (-x, -z, -y),
]

ROTATION [
    (x, y, z),
    (-y, x, z),
    (-x, -y, z),
    (y, -x, z),
];
*/

fn points_diff(lhs: &Point3D, rhs: &Point3D) -> Point3D {
    (lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
}

fn add_points(lhs: &Point3D, rhs: &Point3D) -> Point3D {
    (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
}

fn points_distance(lhs: &Point3D, rhs: &Point3D) -> u64 {
    let diff = points_diff(lhs, rhs);
    
    let x = diff.0.abs() as u64;
    let y = diff.1.abs() as u64;
    let z = diff.2.abs() as u64;
    
    x + y + z
}

#[inline]
fn to_view_point(&(x, y, z): &Point3D, view_idx: u8) -> Point3D {
    match view_idx {
        1 => (x, y, z),
        2 => (x, z, -y),
        3 => (x, -y, -z),
        4 => (x, -z, y),
        
        5 => (y, x, -z),
        6 => (y, z, x),
        7 => (y, -x, z),
        8 => (y, -z, -x),
        
        9 => (z, x, y),
        10 => (z, y, -x),
        11 => (z, -x, -y),
        12 => (z, -y, x),
        
        13 => (-x, y, -z),
        14 => (-x, z, y),
        15 => (-x, -y, z),
        16 => (-x, -z, -y),
        
        17 => (-y, x, z),
        18 => (-y, z, -x),
        19 => (-y, -x, -z),
        20 => (-y, -z, x),
        
        21 => (-z, x, -y),
        22 => (-z, y, x),
        23 => (-z, -x, y),
        24 => (-z, -y, -x),
        
        _ => unreachable!(),
    }
}

pub struct BeaconScanner {
    beacons: HashSet<Point3D>,
    scanners: Vec<Point3D>,
}

impl BeaconScanner {
    fn locate(&mut self, scanner: &Scanner) -> bool {
        for view_idx in 1..=24 {
            let new_view: Scanner = scanner
                .iter()
                .map(|p| to_view_point(p, view_idx))
                .collect();
                
            let distance_iter = self
                .beacons
                .iter()
                .map(|point| std::iter::repeat(point))
                .flat_map(|point| point.zip(new_view.iter()))
                .map(|(lhs, rhs)| points_diff(lhs, rhs));
                
            for distance in distance_iter {
                let relocated: Scanner = new_view
                    .iter()
                    .map(|v| add_points(v, &distance))
                    .collect();
                    
                if relocated.intersection(&self.beacons).count() >= 12 {
                    self.beacons.extend(relocated);
                    self.scanners.push(distance);
                    return true;
                }
            }
        }
        
        false
    }
}

impl crate::AdventOfCode for BeaconScanner {
    fn new(input: &str) -> Self {
        let mut iter = input.lines().skip(1);
        
        let mut unlocated_scanners = Vec::new();
        let mut res = Self {
            scanners: vec![(0, 0, 0)],
            beacons: iter
                .by_ref()
                .take_while(|v| !v.is_empty())
                .map(|v| {
                    let mut i = v.split(',').map(|n| n.parse().unwrap());
                    let x = i.next().unwrap();
                    let y = i.next().unwrap();
                    let z = i.next().unwrap();
                    (x, y, z)
                })
                .collect(),
        };
        
        while iter.next().is_some() {
            let scanner = iter.by_ref()
                .take_while(|v| !v.is_empty())
                .map(|v| {
                    let mut i = v.split(',').map(|n| n.parse().unwrap());
                    let x = i.next().unwrap();
                    let y = i.next().unwrap();
                    let z = i.next().unwrap();
                    (x, y, z)
                })
                .collect();
                
            if !res.locate(&scanner) {
                unlocated_scanners.insert(0, scanner);
            }
        }
        
        while let Some(scanner) = unlocated_scanners.pop() {
            if !res.locate(&scanner) {
                unlocated_scanners.insert(0, scanner);
            }
        }
        
        res
    }
    
    fn part1(&self) -> u64 {
        self.beacons.len() as u64
    }
    
    fn part2(&self) -> u64 {
        (0..self.scanners.len()-1)
            .flat_map(|v| std::iter::repeat(v).zip((v+1)..self.scanners.len()))
            .map(|(lhs, rhs)| (&self.scanners[lhs], &self.scanners[rhs]))
            .map(|(lhs, rhs)| points_distance(lhs, rhs))
            .max()
            .unwrap()
    }
}