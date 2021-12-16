use std::io;
use std::io::Write;

#[derive(Debug)]
enum LengthType {
    TOTAL = 0,
    SUBPACKETS = 1
}

impl From<bool> for LengthType {
    fn from(v: bool) -> Self {
        match v {
            false => LengthType::TOTAL,
            true => LengthType::SUBPACKETS,
        }
    }
}


trait Packet: std::fmt::Display { 
    fn new(version: u8, id: u8, it: &mut std::iter::Peekable<std::str::Chars>) -> Self where Self: Sized;
    fn version_sum(&self, acc: u16) -> u16;
    fn evaluate(&self) -> u64;
    fn print(&self, level: u8) -> String;
    fn bits(&self) -> u32;
}

struct DataPacket {
    version: u8,
    id: u8,
    value: u64,
    bits: u32
}

impl Packet for DataPacket {
    fn new(version: u8, id: u8, it: &mut std::iter::Peekable<std::str::Chars>) -> Self {
        let mut value = 0b0;
        let mut run = true;
        let mut bits = 3+3;
        while run {
            run = it.next().unwrap() != '0';
            value = (value << 4) | it.take(4).fold(0, |acc,c| (acc<<1)|c.to_digit(10).unwrap() as u64);
            bits += 5;
        }
        DataPacket {
            version: version, 
            id: id,
            value: value,
            bits: bits
        }
    }

    fn version_sum(&self, acc: u16) -> u16 {self.version as u16 + acc}

    fn evaluate(&self) -> u64 { self.value as u64 }

    fn print(&self, level: u8) -> String {
        return format!("{}", self);
    }

    fn bits(&self) -> u32 { self.bits }
}


impl std::fmt::Display for DataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Data - Version: {}, ID: {}, bits: {}, value: {:?}", 
            self.version, self.id, self.bits(), self.value)
    } 
}

struct OperatorPacket {
    version: u8,
    id: u8,
    length_type: LengthType,
    length: u16,
    bits: u32, 
    subpackets: Vec<Box<dyn Packet>>,
}


impl Packet for OperatorPacket {
    fn new(version: u8, id: u8, it: &mut std::iter::Peekable<std::str::Chars>) -> Self {
        let length_type: LengthType = ( it.next().unwrap().to_digit(10).unwrap() != 0).into();

        let num_bits = match length_type {
            LengthType::TOTAL => 15,
            LengthType::SUBPACKETS => 11,
        };

        let length = it.take(num_bits).fold(0 as u16, |acc,c| (acc<<1)|c.to_digit(10).unwrap() as u16);

        let mut subpackets = Vec::new();
        let mut bits = 3+3+1+num_bits as u32;

        match length_type {
            LengthType::TOTAL => {
                let mut total_bits = 0 as u32;
                while total_bits < length.into(){
                    match parse_packet(it) {
                        Some(packet) => {total_bits += packet.bits(); bits+= packet.bits(); subpackets.push(packet)},
                        None => {}
                    }
                }
                assert!(total_bits == length.into());
            },
            LengthType::SUBPACKETS => {
                for _ in 0..length {
                    match parse_packet(it) {
                        Some(packet) => {bits+= packet.bits(); subpackets.push(packet)},
                        None => {}
                    }
                }
                assert!(subpackets.len() == length.into());
                
            },
        }
        

        OperatorPacket {
            version: version, 
            id: id,
            length_type: length_type,
            length: length,
            bits: bits,
            subpackets: subpackets
        }
    }

    fn version_sum(&self, acc: u16) -> u16 {
        self.version as u16 + acc + self.subpackets.iter().fold(0, |acc,x| acc + x.version_sum(0))
    }

    fn evaluate(&self) -> u64 {
        let value = match self.id {
            0 => {self.subpackets.iter().map(|x| x.evaluate()).sum()},
            1 => {self.subpackets.iter().map(|x| x.evaluate()).product()},
            2 => {self.subpackets.iter().map(|x| x.evaluate()).min().unwrap_or(0)},
            3 => {self.subpackets.iter().map(|x| x.evaluate()).max().unwrap_or(0)},
            5 => {(self.subpackets[0].evaluate() > self.subpackets[1].evaluate()) as u64},
            6 => {(self.subpackets[0].evaluate() < self.subpackets[1].evaluate()) as u64},
            7 => {(self.subpackets[0].evaluate() == self.subpackets[1].evaluate()) as u64},
            _ => 0
        };
        value
    }

    fn print(&self, level: u8) -> String {
        let num_bits = match self.length_type {
            LengthType::TOTAL => 15,
            LengthType::SUBPACKETS => 11,
        };
        format!("Operator - Version: {}, ID: {}, LengthType: {:?}, Length: {}, Bits: {}, Bits Self: {}, Subpackets: {}", 
        self.version, self.id, self.length_type, self.length, self.bits(), (3+3+1+num_bits),
        
        self.subpackets.iter().fold(String::from(""), |acc, x| format!("{}\n{}{}", 
            acc, 
            std::iter::repeat("  ").take(level.into()).collect::<String>(),
            x.print(level+1))))
    }

    fn bits(&self) -> u32 { self.bits }
}


impl std::fmt::Display for OperatorPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.print(1))
    } 
}


fn parse_packet(it: &mut std::iter::Peekable<std::str::Chars>) -> Option<Box<dyn Packet>> {
    let version = it.take(3).fold(0, |acc,c| (acc<<1)|c.to_digit(10).unwrap() as u8);
    let id = it.take(3).fold(0, |acc,c| (acc<<1)|c.to_digit(10).unwrap() as u8);

    if it.size_hint().0 == 0 { return None }
    match id {
        4 => Some(Box::new(DataPacket::new(version, id, it))),
        _ => Some(Box::new(OperatorPacket::new(version, id, it)))
        
    }
}

pub fn first(content: &str) -> u16 {
    let bit_string = content
        .chars()
        .fold(String::from(""),|acc, x| format!("{}{:0>4b}", acc,x.to_digit(16).unwrap()).to_string());

    println!("{}", bit_string);
    let mut it = bit_string.chars().peekable();
    let packet = parse_packet(&mut it).unwrap();
    println!("{}", packet);
    packet.version_sum(0)
}

pub fn second(content: &str) -> u64 {
    let bit_string = content
        .chars()
        .fold(String::from(""),|acc, x| format!("{}{:0>4b}", acc,x.to_digit(16).unwrap()).to_string());

    let mut it = bit_string.chars().peekable();
    let packet = parse_packet(&mut it).unwrap();
    packet.evaluate()
}