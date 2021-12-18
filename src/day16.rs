struct BitReader {
    bits: Vec<u8>,
    pos: usize,
}

impl BitReader {
    fn new(input: &str) -> Self {
        Self {
            bits: input.bytes().collect(),
            pos: 0,
        }
    }
}

struct ElvesLang {
    version: u8,
}

fn decode(input: &str) -> ElvesLang {
    let _reader = BitReader::new(input);

    ElvesLang { version: 0 }
}

pub struct PacketDecoder {
    decoded: ElvesLang,
}

impl crate::AdventOfCode for PacketDecoder {
    fn new(input: &str) -> Self {
        Self {
            decoded: decode(input),
        }
    }

    fn part1(&self) -> u64 {
        self.decoded.version as u64
    }

    fn part2(&self) -> u64 {
        0
    }
}
