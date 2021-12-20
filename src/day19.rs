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

fn to_view_point((x, y, z): Point3D, view: u8) -> Point3D {
    match view {
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

// location and the beacons
fn scanners_intersect(base: &Scanner, rhs: &Scanner) -> Option<(Point3D, Scanner)> {
    for view_idx in 1..=24 {
        let new_view: Scanner = rhs
            .iter()
            .copied()
            .map(|p| to_view_point(p, view_idx))
            .collect();
            
        let distance_iter = base
            .iter()
            .map(|point| std::iter::repeat(point))
            .flat_map(|point| point.zip(new_view.iter()))
            .map(|(lhs, rhs)| points_diff(lhs, rhs));
            
        for diff in distance_iter {
            let relocated: Scanner = new_view
                .iter()
                .map(|v| add_points(v, &diff))
                .collect();
                
            if relocated.intersection(&base).count() >= 12 {
                return Some((diff, relocated))
            }
        }
    }
    
    None
}

pub struct BeaconScanner {
    scanners: Vec<Scanner>,
}

impl BeaconScanner {
    fn locate(&self) -> (Vec<Point3D>, Scanner) {
        let mut base = self.scanners[0].to_owned(); // also the base location
        let mut located_scanners: Vec<Point3D> = Default::default();
        let mut unlocated_scanners: Vec<Scanner> = self.scanners.get(1..).unwrap().to_owned().to_vec();
        
        while let Some(scanner) = unlocated_scanners.pop() {
            match scanners_intersect(&base, &scanner) {
                None => unlocated_scanners.insert(0, scanner),
                Some((location, beacons)) => {
                    base.extend(beacons);
                    located_scanners.push(location);
                }
            }
        }
        
        (located_scanners, base)
    }
}

impl crate::AdventOfCode for BeaconScanner {
    fn new(input: &str) -> Self {
        let mut scanners = Vec::new();
        let mut iter = input.lines();
        
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
                
            scanners.push(scanner);
        }
        
        Self { scanners }
    }
    
    fn part1(&self) -> u64 {
        self.locate().1.len() as u64
    }
    
    fn part2(&self) -> u64 {
        let scanner_locations = self.locate().0.into_iter().collect::<Vec<_>>();
        
        (0..scanner_locations.len()-1)
            .flat_map(|v| std::iter::repeat(v).zip((v+1)..scanner_locations.len()))
            .map(|(lhs, rhs)| (&scanner_locations[lhs], &scanner_locations[rhs]))
            .map(|(lhs, rhs)| points_distance(lhs, rhs))
            .max()
            .unwrap()
    }
}