use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use self::PacketInfo::{Literal, Operator};

pub(crate) fn run() {
    // let _input = "D2FE28";
    // let _input = "38006F45291200";
    // let _input = "EE00D40C823060";
    // let _input = "8A004A801A8002F478";
    // let _input = "620080001611562C8802118E34";
    // let _input = "C0015000016115A2E0802F182340";
    // let _input = "A0016C880162017C3686B18A3D4780";
    let _input = "C200B40A82";
    // let _input = "04005AC33890";
    // let _input = "880086C3E88112";
    // let _input = "CE00C43D881120";
    // let _input = _get_input();

    let bits: Bits = _input.parse().unwrap();
    println!("input: {:?}", bits);
    let packet: Packet = parse_packet(&bits.0).0;
    println!("{:?}", packet);
    println!("sum versions: {}", packet.sum_versions());
    println!("result: {}", packet.calculate_result());
}

struct Bits(Vec<bool>);

impl Debug for Bits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|&b| match b {
            true => '1',
            false => '0'
        }).collect::<String>())
    }
}

#[derive(Debug)]
enum PacketInfo {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u64,
    packet_type: u64,
    packet_info: PacketInfo
}

impl Packet {
    pub fn sum_versions(&self) -> u64 {
        if let PacketInfo::Operator(children) = &self.packet_info {
            self.version + children.iter().map(|c| c.sum_versions()).sum::<u64>()
        } else {
            self.version
        }
    }
    pub fn calculate_result(&self) -> u64 {
        match &self.packet_info {
            Literal(val) => *val,
            Operator(children) => {
                let mut values = children.iter().map(|p| p.calculate_result());
                match &self.packet_type {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => if values.next().unwrap() > values.next().unwrap() { 1 } else { 0 },
                    6 => if values.next().unwrap() < values.next().unwrap() { 1 } else { 0 },
                    7 => if values.next().unwrap() == values.next().unwrap() { 1 } else { 0 },
                    _ => panic!("out of range")
                }
            }
        }
    }
}

fn parse_packet(slice: &[bool]) -> (Packet, usize) {
    let version: u64 = parse_bits(&slice[0..3]);
    let packet_type: u64 = parse_bits(&slice[3..6]);
    if packet_type == 4 {
        let mut i = 6;
        let mut bits: Vec<bool> = Default::default();
        loop {
            bits.extend(&slice[i + 1..=i + 4]);
            if !slice[i] {
                break;
            }
            i += 5;
        }
        let value: u64 = parse_bits(&bits);

        (Packet {
            version,
            packet_type,
            packet_info: Literal(value)
        }, i + 5)
    } else {
        let mut sub_packets: Vec<Packet> = Default::default();
        let mut construction_bits_used = 7;

        if slice[6] {
            // 11-bit number representing the number of sub-packets
            construction_bits_used += 11;
            let num_packets = parse_bits(&slice[7..18]);
            for _ in 0..num_packets {
                let (packet, used) = parse_packet(&slice[construction_bits_used..]);
                construction_bits_used += used;
                sub_packets.push(packet);
            }
        } else {
            // 15-bit number representing the number of bits in the sub-packets
            construction_bits_used += 15;
            let num_bits = parse_bits(&slice[7..22]) as usize;
            let mut num_bits_used = 0usize;
            while num_bits_used < num_bits {
                let (packet, used) = parse_packet(&slice[construction_bits_used + num_bits_used..construction_bits_used + num_bits]);
                num_bits_used += used;
                sub_packets.push(packet);
            }
            construction_bits_used += num_bits_used;
        }

        (Packet {
            version,
            packet_type,
            packet_info: Operator(sub_packets)
        }, construction_bits_used)
    }
}

fn parse_bits(bits: &[bool]) -> u64 {
    bits.iter().rev().enumerate().map(|(i, &b)| if b { 2u64.pow(i as u32) } else { 0u64 }).sum()
}

impl FromStr for Bits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits: Vec<bool> = Vec::with_capacity(s.len() * 4);

        for c in s.chars() {
            let val = c.to_digit(16).map_or(Err(()), |val| Ok(val))? as u64;
            for i in (0..4).rev() {
                bits.push(2u64.pow(i) & val != 0);
            }
        }

        Ok(Self(bits))
    }
}

fn _get_input() -> &'static str {
    "005473C9244483004B001F79A9CE75FF9065446725685F1223600542661B7A9F4D001428C01D8C30C61210021F0663043A20042616C75868800BAC9CB59F4BC3A40232680220008542D89B114401886F1EA2DCF16CFE3BE6281060104B00C9994B83C13200AD3C0169B85FA7D3BE0A91356004824A32E6C94803A1D005E6701B2B49D76A1257EC7310C2015E7C0151006E0843F8D000086C4284910A47518CF7DD04380553C2F2D4BFEE67350DE2C9331FEFAFAD24CB282004F328C73F4E8B49C34AF094802B2B004E76762F9D9D8BA500653EEA4016CD802126B72D8F004C5F9975200C924B5065C00686467E58919F960C017F00466BB3B6B4B135D9DB5A5A93C2210050B32A9400A9497D524BEA660084EEA8EF600849E21EFB7C9F07E5C34C014C009067794BCC527794BCC424F12A67DCBC905C01B97BF8DE5ED9F7C865A4051F50024F9B9EAFA93ECE1A49A2C2E20128E4CA30037100042612C6F8B600084C1C8850BC400B8DAA01547197D6370BC8422C4A72051291E2A0803B0E2094D4BB5FDBEF6A0094F3CCC9A0002FD38E1350E7500C01A1006E3CC24884200C46389312C401F8551C63D4CC9D08035293FD6FCAFF1468B0056780A45D0C01498FBED0039925B82CCDCA7F4E20021A692CC012B00440010B8691761E0002190E21244C98EE0B0C0139297660B401A80002150E20A43C1006A0E44582A400C04A81CD994B9A1004BB1625D0648CE440E49DC402D8612BB6C9F5E97A5AC193F589A100505800ABCF5205138BD2EB527EA130008611167331AEA9B8BDCC4752B78165B39DAA1004C906740139EB0148D3CEC80662B801E60041015EE6006801364E007B801C003F1A801880350100BEC002A3000920E0079801CA00500046A800C0A001A73DFE9830059D29B5E8A51865777DCA1A2820040E4C7A49F88028B9F92DF80292E592B6B840"
}