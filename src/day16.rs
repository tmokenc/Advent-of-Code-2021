// struct BitReader {
//     bytes: Vec<u8>,
//     remain: usize,
// }
// 
// impl BitReader {
//     fn new(input: &str) -> Self {
//         let mut bytes = Vec::new();
//         let mut current = 0;
//         let mut remain = 0;
// 
//         let iter = input
//             .chars()
//             .map(|v| v.to_digit(16).unwrap() as u8);
// 
//         for (bits, shift) in iter {
//             remain += 4;
// 
//             if shift == 0 && remain / 8 > 0 {
//                 bytes.push(current);
//                 current = 0;
//             }
// 
//             current |= bits << shift;
//         }
// 
//         Self { bytes, remain }
//     }
// 
//     fn read(&mut self, count: usize) -> Self {
//         let capacity = count / 8 + (count % 8).clamp(0, 1);
//         let mut bytes = Vec::with_capacity(capacity);
//         let mut to_read = count;
//         
//         let mut byte = 0;
//         let mut bit_left = 8;
//         
//         while to_read > 0 {
//             let remainder = self.remain % 8;
//             let bit_read = to_read
//                 .clamp(0, 8)
//                 .min(bit_left)
//                 .min(remainder);
//             
//             
//             
//             
//             to_read -= bit_read;
//         }
// 
//         Self { 
//             bytes, 
//             remain: count,
//         }
//     }
// 
//     fn to_number(self) -> u64 {
//         let mut num = self.bytes[0];
//     }
// }

#[derive(Default)]
struct BitReader {
    bits: Vec<u8>,
}

impl BitReader {
    fn new(input: &str) -> Self {
        let mut bits = Vec::new();
        
        for ch in input.chars() {
            let num = ch.to_digit(16).unwrap() as u8;
            let mut tmp = [0; 4];
            
            for i in 0..4 {
                tmp[i] = num >> i & 1;
            }
            
            bits.extend(tmp.into_iter().rev());
        }
        
        Self {
            bits
        }
    }
    
    fn remain(&self) -> usize {
        self.bits.len()
    }
    
    fn read(&mut self, count: usize) -> Self {
        let mut new = self.bits.split_off(count);
        std::mem::swap(&mut self.bits, &mut new);
        
        Self {
            bits: new
        }
    }
    
    fn extend(&mut self, other: Self) {
        self.bits.extend(other.bits);
    }
    
    fn to_number(self) -> u64 {
        let mut number = 0;
        
        for (idx, bit) in self.bits.into_iter().rev().enumerate() {
            number |= (bit as u64) << idx;
        }
        
        number
    }
}

enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

enum Value {
    Literal(u64),
    Operator {
        t: OperatorType,
        packets: Vec<ElvesLang>,
    }
}

impl Value {
    // the reader must start at TYPE index
    fn parse(reader: &mut BitReader) -> Self {
        let t = reader.read(3).to_number();
        
        if t == 4 {
            let mut bits = BitReader::default();
            
            while reader.read(1).to_number() == 1 {
                bits.extend(reader.read(4));
            }
            
            bits.extend(reader.read(4));
            return Self::Literal(bits.to_number());
        }
        
        let mut packets = Vec::new();
        
        if reader.read(1).to_number() == 0 {
            let length = reader.read(15).to_number() as usize;
            let mut sub_reader = reader.read(length);
            
            while sub_reader.remain() > 0 {
                packets.push(ElvesLang::decode(&mut sub_reader));
            }
        } else {
            let count = reader.read(11).to_number();
            
            for _ in 0..count {
                packets.push(ElvesLang::decode(reader));
            }
        }
        
        let t = match t {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => unreachable!(),
        };
        
        Self::Operator { t, packets }
    }
}

struct ElvesLang {
    version: u8,
    value: Value,
}

impl ElvesLang {
    fn decode(reader: &mut BitReader) -> Self {
        let version = reader.read(3).to_number() as u8;
        let value = Value::parse(reader);
    
        ElvesLang {
            version,
            value,
        }
    }
    
    fn sub_packets(&self) -> Vec<&ElvesLang> {
        let mut sub_packets = Vec::new();
        
        match self.value {
            Value::Literal(_) => (),
            Value::Operator{ ref packets, .. } => sub_packets.extend(packets),
        }
        
        sub_packets
    }
    
    fn compute(&self) -> u64 {
        match self.value {
            Value::Literal(num) => num,
            Value::Operator { ref t, ref packets } => {
                let mut values = packets.iter().map(|v| v.compute());
                
                match t {
                    OperatorType::Sum => values.sum(),
                    OperatorType::Product => values.product(),
                    OperatorType::Minimum => values.min().unwrap(),
                    OperatorType::Maximum => values.max().unwrap(),
                    OperatorType::GreaterThan => {
                        (values.next().unwrap() > values.next().unwrap()) as u64
                    }
                    OperatorType::LessThan => {
                        (values.next().unwrap() < values.next().unwrap()) as u64
                    }
                    OperatorType::EqualTo => {
                        (values.next().unwrap() == values.next().unwrap()) as u64
                    }
                }
            }
        }
    }
}

pub struct PacketDecoder {
    decoded: ElvesLang,
}

impl crate::AdventOfCode for PacketDecoder {
    fn new(input: &str) -> Self {
        let mut reader = BitReader::new(input);

        Self {
            decoded: ElvesLang::decode(&mut reader),
        }
    }

    fn part1(&self) -> u64 {
        let mut packets: Vec<&ElvesLang> = vec![&self.decoded];
        let mut res = 0;
        
        while let Some(packet) = packets.pop() {
            res += packet.version as u64;
            packets.extend(packet.sub_packets());
        }
        
        res
    }

    fn part2(&self) -> u64 {
        self.decoded.compute()
    }
}
