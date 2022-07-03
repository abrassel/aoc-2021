use std::{fs, io::Read, iter};

struct BitStream<S>(S, usize);

impl<S> BitStream<S> {
    pub fn new(stream: S) -> Self {
        Self(stream, 0)
    }
}

pub fn bit_stream_from_hex_file() -> impl Iterator<Item = bool> {
    struct NibbleIter {
        num: u8,
        offset: usize,
    }
    impl NibbleIter {
        pub fn new(num: u8) -> Self {
            Self { num, offset: 4 }
        }
    }
    impl Iterator for NibbleIter {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.offset == 0 {
                return None;
            }
            let ret = ((self.num >> (self.offset - 1)) & 0b1) == 1;
            self.offset -= 1;
            Some(ret)
        }
    }
    fs::File::open("day16/day16.txt")
        .unwrap()
        .bytes()
        .map(Result::unwrap)
        .map(|digit| {
            let char = digit as char;
            char.to_digit(16).unwrap() as u8
        })
        .flat_map(NibbleIter::new)
}

impl<S: Iterator<Item = bool>> BitStream<S> {
    pub fn read_number<const COUNT: usize>(&mut self) -> usize {
        self.1 += COUNT;
        let mut count = 0;
        let res = (&mut self.0)
            .take(COUNT)
            .inspect(|_| count += 1)
            .fold(0, |acc, val| (acc << 1) | (val as usize));
        debug_assert_eq!(COUNT, count);
        res
    }

    pub fn read(&self) -> usize {
        self.1
    }
}

trait PacketParser {
    fn parse_literal(&mut self) -> u64;
    fn parse_operator(&mut self) -> Vec<u64>;
    fn parse_packet(&mut self) -> u64;
}

impl<S: Iterator<Item = bool>> PacketParser for BitStream<S> {
    fn parse_literal(&mut self) -> u64 {
        let mut there_is_more = true;
        let mut acc = 0;
        while there_is_more {
            there_is_more = self.read_number::<1>() != 0;
            let next = self.read_number::<4>() as u64;
            acc = (acc << 4) | next;
        }
        acc
    }

    fn parse_operator(&mut self) -> Vec<u64> {
        let length_type_id = self.read_number::<1>();
        let packets = if length_type_id == 0 {
            let read_limit = self.read_number::<15>() + self.read(); //important that the read comes after `read_number`
            let mut packets = vec![];
            while self.read() < read_limit {
                let packet = self.parse_packet();
                packets.push(packet);
            }
            packets
        } else {
            let packet_count = self.read_number::<11>();
            iter::repeat_with(|| self.parse_packet())
                .take(packet_count)
                .collect()
        };
        packets
    }

    fn parse_packet(&mut self) -> u64 {
        let _version = self.read_number::<3>() as u8;
        let ty = self.read_number::<3>();
        if ty == 4 {
            self.parse_literal()
        } else {
            let mut operator = self.parse_operator().into_iter();
            match ty {
                0 => operator.sum(),
                1 => operator.product(),
                2 => operator.min().unwrap(),
                3 => operator.max().unwrap(),
                5 => (operator.next() > operator.next()) as u64,
                6 => (operator.next() < operator.next()) as u64,
                7 => (operator.next() == operator.next()) as u64,
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let input_stream = bit_stream_from_hex_file();
    let result = BitStream::new(input_stream).parse_packet();
    println!("{}", result);
}
