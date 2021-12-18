pub struct PacketDecoder {
    bytes: Vec<u8>,
}

impl crate::AdventOfCode for PacketDecoder {
    fn new(input: &str) -> Self {
        let bytes = input.bytes().collect();
        Self { bytes }
    }
    
    fn part1(&self) -> u64 {
        0
    }
    
    fn part2(&self) -> u64 {
        0
    }
}