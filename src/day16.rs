use itertools::Itertools;
use std::convert::TryFrom;

#[derive(Debug)]
enum PacketError {
    BitReaderError(bitreader::BitReaderError),
    BadValue,
}

impl From<bitreader::BitReaderError> for PacketError {
    fn from(error: bitreader::BitReaderError) -> Self {
        Self::BitReaderError(error)
    }
}

#[derive(Debug)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
    Literal(u64),
}

// Only handles operators, not literals.
impl TryFrom<u8> for PacketType {
    type Error = PacketError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let packet_type = match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => return Err(PacketError::BadValue),
        };
        Ok(packet_type)
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn new(version: u8, packet_type: PacketType, subpackets: Vec<Packet>) -> Self {
        Self {
            version,
            packet_type,
            subpackets,
        }
    }

    fn sum_versions(&self) -> u64 {
        let subsum: u64 = self
            .subpackets
            .iter()
            .map(|packet| packet.sum_versions())
            .sum();
        subsum + self.version as u64
    }

    fn evaluate(&self) -> u64 {
        let mut subvalues = self.subpackets.iter().map(|packet| packet.evaluate());
        match &self.packet_type {
            PacketType::Literal(value) => *value,
            PacketType::Sum => subvalues.sum(),
            PacketType::Product => subvalues.product(),
            PacketType::Minimum => subvalues.min().unwrap(),
            PacketType::Maximum => subvalues.max().unwrap(),
            PacketType::GreaterThan => {
                (subvalues.next().unwrap() > subvalues.next().unwrap()).into()
            }
            PacketType::LessThan => (subvalues.next().unwrap() < subvalues.next().unwrap()).into(),
            PacketType::EqualTo => (subvalues.next().unwrap() == subvalues.next().unwrap()).into(),
        }
    }
}

fn read_packet(reader: &mut bitreader::BitReader) -> Result<Packet, PacketError> {
    let version = reader.read_u8(3)?;
    let packet_type = reader.read_u8(3)?;
    let packet = match packet_type {
        4 => {
            // Literal
            let mut value: u64 = 0;
            loop {
                let continues = reader.read_bool()?;
                let bits = reader.read_u64(4)?;
                value = value << 4 | bits;
                if !continues {
                    break;
                }
            }
            Packet::new(version, PacketType::Literal(value), vec![])
        }
        _ => {
            // Operator
            let operator = PacketType::try_from(packet_type)?;
            let mut subpackets: Vec<Packet> = vec![];
            let count_packets = reader.read_bool()?;
            if count_packets {
                let count = reader.read_u16(11)?;
                for _ in 0..count {
                    let subpacket = read_packet(reader)?;
                    subpackets.push(subpacket);
                }
            } else {
                let length = reader.read_u64(15)?;
                let start = reader.position();
                while reader.position() < start + length {
                    let subpacket = read_packet(reader)?;
                    subpackets.push(subpacket);
                }
                if reader.position() > start + length {
                    return Err(PacketError::BadValue);
                }
            }
            Packet::new(version, operator, subpackets)
        }
    };
    Ok(packet)
}

pub(crate) fn day16() {
    let input = std::fs::read_to_string("data/day16.txt").unwrap();
    let transmission: Vec<u8> = input
        .chars()
        .tuples()
        .map(|(a, b)| {
            let hi = a.to_digit(16).unwrap() as u8;
            let lo = b.to_digit(16).unwrap() as u8;
            hi << 4 | lo
        })
        .collect();
    let mut reader = bitreader::BitReader::new(&transmission);
    let packet = read_packet(&mut reader).unwrap();

    println!("Part one answer is {}", packet.sum_versions());
    println!("Part two answer is {}", packet.evaluate());
}
