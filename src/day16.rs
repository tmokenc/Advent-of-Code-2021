struct BitReader {
    bytes: Vec<u8>,
    remain: usize,
}

impl BitReader {
    fn new(input: &str) -> Self {
        let mut bytes = Vec::new();
        let mut current = 0;
        let mut remain = 0;

        let iter = input
            .chars()
            .map(|v| v.to_digit(16).unwrap() as u8)
            .zip([0, 4].into_iter().cycle());

        for (bits, shift) in iter {
            remain += 4;

            if shift == 0 && remain / 8 > 0 {
                bytes.push(current);
                current = 0;
            }

            current |= bits << shift;
        }

        Self { bytes, remain }
    }

    fn read(&mut self, mut count: usize) -> Self {
        let mut bytes = Vec::with_capacity(count / 8 + (count % 8).clamp(0, 1));
        let remain = count;

        // while count != 0 {
        //     let mut to_read = count.clamp(0, 8);
        //     let first_remain = self.remain % 8;
        //
        //
        //
        //     count -= to_read;
        // }

        Self { bytes, remain }
    }

    fn to_number(self) -> u64 {
        0
    }
}

enum DataType {
    Literal(u64),
    Operator(Box<ElvesLang>),
}

struct ElvesLang {
    version: u8,
    data: DataType,
}

fn decode(mut reader: BitReader) -> ElvesLang {
    let version = reader.read(3).to_number() as u8;
    // let typ = match reader.read(3)[0] {
    //     4 =>
    // }

    ElvesLang {
        version,
        data: DataType::Literal(0),
    }
}

pub struct PacketDecoder {
    decoded: ElvesLang,
}

impl crate::AdventOfCode for PacketDecoder {
    fn new(input: &str) -> Self {
        let reader = BitReader::new(input);

        Self {
            decoded: decode(reader),
        }
    }

    fn part1(&self) -> u64 {
        self.decoded.version as u64
    }

    fn part2(&self) -> u64 {
        0
    }
}
