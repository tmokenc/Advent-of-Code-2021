use std::cmp;
use std::collections::HashSet;

type Pixel = bool;
type Coord = (isize, isize);

const DARK: Pixel = false;

struct Enhancement([u128; 4]);

impl Enhancement {
    fn parse(input: &str) -> Self {
        let mut enhancement = [0u128; 4];
        let mut idx = 0;
        let mut bits = 0;
        
        for ch in input.chars() {
            let bit_idx = idx % 128;
            let bit = (ch == '#') as u128;
            
            bits |= bit << bit_idx;
            
            if bit_idx == 127 {
                let arr_idx = idx / 128;
                enhancement[arr_idx] = bits;
                bits = 0
            }
            
            idx += 1;
        }
        
        Self(enhancement)
    }
    
    fn is_light(&self, idx: usize) -> bool {
        let arr_idx = idx / 128;
        let bit_idx = idx % 128;
        let bit = (self.0[arr_idx] >> bit_idx) & 1;
        bit == 1
    }
}

#[derive(Clone)]
struct Image {
    light_pixels: HashSet<Coord>,
    outside: Pixel,
    top: isize,
    left: isize,
    bottom: isize,
    right: isize,
}

impl Image {
    fn parse<'a>(lines: impl Iterator<Item=&'a str>) -> Self {
        let mut light_pixels = HashSet::new();
        let mut max_width = 0;
        let mut max_height = 0;
        
        for (line, y) in lines.zip(0..) {
            for (ch, x) in line.chars().zip(0..) {
                max_width = cmp::max(max_width, x);
                
                match ch {
                    '#' => { light_pixels.insert((x, y)); },
                    '.' => (),
                    _ => unreachable!(),
                }
            }
            
            max_height = cmp::max(max_height, y);
        }
        
        Self { 
            light_pixels,
            outside: DARK,
            top: 0,
            left: 0,
            bottom: max_height as isize,
            right: max_width as isize,
        }
    }
    
    fn is_coord_inbound(&self, coord: &Coord) -> bool {
        let check_x = coord.0 >= self.left && coord.0 <= self.right;
        let check_y = coord.1 >= self.top && coord.1 <= self.bottom;
        check_x && check_y
    }
    
    fn get_enhancement_idx(&self, x: isize, y: isize) -> usize {
        let mut idx = 0;
        
        let coords = [
            (x+1, y+1),
            (x, y+1),
            (x-1, y+1),
            (x+1, y),
            (x, y),
            (x-1, y),
            (x+1, y-1),
            (x, y-1),
            (x-1, y-1),
        ];
        
        for (i, coord) in coords.into_iter().enumerate() {
            let bit = if self.is_coord_inbound(&coord) {
                self.light_pixels.contains(&coord) as usize
            } else {
                self.outside as usize
            };
            
            idx |= bit << i;
        }
        
        idx
    }
    
    fn padding_one(&mut self) {
        self.top -= 1;
        self.left -= 1;
        self.right += 1;
        self.bottom += 1;
        
        if self.outside == DARK {
            return
        }
        
        let top_iter = (self.left..=self.right).map(|x| (x, self.top));
        let bottom_iter = (self.left..=self.right).map(|x| (x, self.bottom));
        let left_iter = ((self.top+1)..self.bottom).map(|y| (self.left, y));
        let right_iter = ((self.top+1)..self.bottom).map(|y| (self.right, y));
        
        top_iter
            .chain(bottom_iter)
            .chain(left_iter)
            .chain(right_iter)
            .for_each(|coord| { self.light_pixels.insert(coord); })
    }
    
    fn apply_enhancement(&mut self, enhancement: &Enhancement) {
        self.padding_one();
        
        let clone = Self {
            light_pixels: std::mem::take(&mut self.light_pixels),
            outside: self.outside,
            top: self.top,
            left: self.left,
            right: self.right,
            bottom: self.bottom,
        };
        
        for x in self.left..=self.right {
            for y in self.top..=self.bottom {
                let idx = clone.get_enhancement_idx(x, y);
                if enhancement.is_light(idx) {
                    self.light_pixels.insert((x, y));
                }
            }
        }
        
        if enhancement.is_light(0) {
            self.outside = !self.outside;
        }
    }
    
    fn count_light(&self) -> u64 {
        self.light_pixels.len() as u64
    }
}

pub struct TrenchMap {
    enhancement: Enhancement,
    image: Image,
}

impl crate::AdventOfCode for TrenchMap {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        
        Self { 
            enhancement: Enhancement::parse(lines.next().unwrap()), 
            image: Image::parse(lines.skip(1)),
        }
    }
    
    fn part1(&self) -> u64 {
        let mut image = self.image.clone();
        image.apply_enhancement(&self.enhancement);
        image.apply_enhancement(&self.enhancement);
        image.count_light()
    }
    
    fn part2(&self) -> u64 {
        let mut image = self.image.clone();
        
        for _ in 0..50 {
            image.apply_enhancement(&self.enhancement);
        }
        
        image.count_light()
    }
}