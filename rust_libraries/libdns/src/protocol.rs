    + size_of::<ANCount>()
    + size_of::<ARCount>();
    + size_of::<HeaderFlags>()
    + size_of::<NSCount>()
    + size_of::<QDCount>()
const AA_BITFLAG: u16 = 0b0000010000000000;
const AD_BITFLAG: u16 = 0b0000000000100000;
const BOOLEAN_BITFLAG: u16 = 0b0000000000000001;
const BYTE_COUNT_OF_HEADER: usize = BYTE_LENTH_OF_HEADER / size_of::<u8>();
const BYTE_COUNT_OF_U128: usize = size_of::<u128>() / size_of::<u8>();

#[allow(non_upper_case_globals)]
const BYTE_COUNT_OF_U16: usize = size_of::<u16>() / size_of::<u8>();
const BYTE_COUNT_OF_U32: usize = size_of::<u32>() / size_of::<u8>();
const BYTE_LENTH_OF_HEADER: usize = size_of::<HeaderID>()
const CD_BITFLAG: u16 = 0b0000000000010000;
const OPCODE_BITFLAG: u16 = 0b0111100000000000;

// Bit order for flags left -> right
const QR_BITFLAG: u16 = 0b1000000000000000;
const RA_BITFLAG: u16 = 0b0000000010000000;
const RCODE_BITFLAG: u16 = 0b0000000000001111;
const RD_BITFLAG: u16 = 0b0000000100000000;
const TC_BITFLAG: u16 = 0b0000001000000000;
const Z_BITFLAG: u16 = 0b0000000001000000;
impl BitAnd<&u16> for HeaderFlags {
    type Output = Self;
    fn bitand(self, rhs: &u16) -> Self::Output {
        Self(self.0 & rhs)
    }
}
impl BitAnd<u16> for HeaderFlags {
    type Output = Self;
    fn bitand(self, rhs: u16) -> Self::Output {
        Self(self.0 & rhs)
    }
}
impl BitAndAssign<&mut Self> for HeaderFlags {
    fn bitand_assign(&mut self, rhs: &mut Self) {
        self.0 |= rhs.0
    }
}
impl BitAndAssign<Self> for HeaderFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}
impl BitAndAssign<u16> for HeaderFlags {
    fn bitand_assign(&mut self, rhs: u16) {
        self.0 |= rhs
    }
}
impl PartialEq<u16> for HeaderFlags {
    fn eq(&self, other: &u16) -> bool {
        &self.0 == other
    }
    fn ne(&self, other: &u16) -> bool {
        &self.0 != other
    }
}
impl QName {
    pub fn to_string(&self) -> anyhow::Result<String> {
        let mut output = String::new();
        let mut cursor = std::io::Cursor::new(self.0.as_slice());
        loop {
            let size = cursor.read_u8()?;
            if size == 0 {
                break;
            }
            let buffer = read_bytes(size as usize, &mut cursor)?;
            let s = String::from_utf8(buffer)?;
            output = output + &s
        }
        Ok(output)
    }
    pub fn read_from_slice<Container: AsRef<[u8]>>(container: Container) -> anyhow::Result<Self> {
        let buffer = container.as_ref();
        let mut cursor = std::io::Cursor::new(buffer);
        Self::read_from_reader(&mut cursor)
    }
    pub fn read_from_reader<Reader: Read + Seek>(reader: &mut Reader) -> anyhow::Result<Self> {
        let mut name: Vec<u8> = vec![];

        loop {
            let size = read_byte(reader)?;
            if size == 0 {
                break;
            }
            if size & 0b11000000 == 0b11000000 {
                return Err(anyhow::Error::msg(
                    "A pointer reference is not valid for a QName",
                ));
            }
            if size > 63 {
                return Err(anyhow::Error::msg(format!("Invalid label length {size}")));
            }
            if name.len() > 253 {
                return Err(anyhow::Error::msg(format!(
                    "The whole name is too large. Max Len is 253: current len is {}",
                    name.len()
                )));
            }
            name.push(size);
            let mut bytes = read_bytes(size as usize, reader)?;
            name.append(&mut bytes);
        }
        Ok(Self(name))
    }
}
impl Shr<u16> for HeaderFlags {
    type Output = Self;
    fn shr(self, rhs: u16) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

// ===== Constants =====
pub const OPCODE_A: u16 = 0;
pub enum RData {
    A(std::net::Ipv4Addr),
    AAAA(std::net::Ipv6Addr),
    CNAME(()),
    TXT(()),
    Other(Vec<u8>),
}
pub fn get_ad(f: &HeaderFlags) -> bool {
    (*f & AD_BITFLAG) >> 5 != 0
}
pub fn get_authoritative_answer(f: &HeaderFlags) -> bool {
    ((*f & AA_BITFLAG) >> 10) != 0
}
pub fn get_cd(f: &HeaderFlags) -> bool {
    ((*f & CD_BITFLAG) >> 4) != 0
}
pub fn get_opcode(f: &HeaderFlags) -> u16 {
    ((*f & OPCODE_BITFLAG) >> 11).0
}

pub fn get_qr(f: &HeaderFlags) -> bool {
    ((*f & QR_BITFLAG) >> 15) != 0
}
pub fn get_rcode(f: &HeaderFlags) -> RCode {
    (*f & RCODE_BITFLAG).0
}
pub fn get_recursion_available(f: &HeaderFlags) -> bool {
    ((*f & RA_BITFLAG) >> 7) != 0
}
pub fn get_recursion_desired(f: &HeaderFlags) -> bool {
    ((*f & RD_BITFLAG) >> 8) != 0
}

pub fn get_truncated(f: &HeaderFlags) -> bool {
    ((*f & TC_BITFLAG) >> 9) != 0
}
pub fn get_z(f: &HeaderFlags) -> bool {
    ((*f & Z_BITFLAG) >> 6) != 0
}

// we are trying parsing again from scratch because we are having trouble with aligment.
pub fn parse_raw_dns_response_packet<Packet: AsRef<[u8]>>(packet: Packet) -> anyhow::Result<()> {
    let buffer = packet.as_ref();
    let mut cursor = std::io::Cursor::new(buffer);

    // attempt to extract the header
    if buffer.len() < BYTE_LENTH_OF_HEADER {
        return Err(anyhow::Error::msg(
            "the buffer is too small to contain a dns header",
        ));
    }
    let header_id: HeaderID = cursor
        .read_u16::<byteorder::BigEndian>()
        .context("Failed to read the header id from the packet")?;
    let header_flags: u16 = cursor.read_u16::<byteorder::BigEndian>()?;
    let header_flags = HeaderFlags(header_flags);

    let qr = get_qr(&header_flags);
    let opcode = get_opcode(&header_flags);
    let rcode = get_rcode(&header_flags);
    let tc = get_truncated(&header_flags);
    let rd = get_truncated(&header_flags);
    let ra = get_truncated(&header_flags);
    dbg!(qr, opcode, rcode, tc, rd, ra);

    let qdcount: QDCount = cursor.read_u16::<byteorder::BigEndian>()?;
    let ancount: ANCount = cursor.read_u16::<byteorder::BigEndian>()?;
    let nscount: NSCount = cursor.read_u16::<byteorder::BigEndian>()?;
    let arcount: ARCount = cursor.read_u16::<byteorder::BigEndian>()?;

    // questions
    dbg!(qdcount, ancount, nscount, arcount);

    for parsed_count in 0..qdcount {
        // store the original position;
        let fdqn = read_qname(&mut cursor)?;

        // qtype
        let qtype: u16 = cursor.read_u16::<byteorder::BigEndian>()?;
        let qclass: u16 = cursor.read_u16::<byteorder::BigEndian>()?;
        dbg!(fdqn, qtype, qclass);
    }
    for _ in 0..ancount {
        let name =
            read_qname(&mut cursor).context("Failed to read fdqn from answer".to_string())?;
        dbg!(name);
        let r#type = cursor.read_u16::<byteorder::BigEndian>()?;
        let class = cursor.read_u16::<byteorder::BigEndian>()?;
        let ttl = read_u32_be(&mut cursor)?;
        let rdata_len = cursor.read_u16::<byteorder::BigEndian>()?;
        let rdata = read_bytes(rdata_len as usize, &mut cursor)?;
        println!("r#type {}", r#type);
        println!("rdata {rdata:?}");
        match r#type {
            1 => {
                // ip v4 address
                let ipv4 = Ipv4Addr::from_bits(u32::from_be_bytes([
                    rdata[0], rdata[1], rdata[2], rdata[3],
                ]));
                dbg!(ipv4);
            }
            5 => {
                rdata.read
                // cname
            }
            16 => {
                let txt = read_txt_record_from_buffer(&rdata)?;
                println!("txt {txt}")
            }
            28 => {
                println!("ipv6")
                // ipv6
            }
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "Unsupported record type {}",
                    r#type
                )));
            }
        }
    }

    for _ in 0..nscount {
        println!("warn: ns response not supported");
    }
    for _ in 0..arcount {
        println!("warn: ar response not supported");
    }
    Ok(())
}

pub fn read_128_be<Reader: Read>(reader: &mut Reader) -> anyhow::Result<u128> {
    let mut buf = [0_u8; 16];
    reader.read_exact(&mut buf).context(format!(
        "Failed to read {} bytes from the reader",
        buf.len()
    ))?;
    let n: u128 = u128::from_be_bytes(buf);
    return Ok(n);
}

pub fn read_byte<Reader: Read>(reader: &mut Reader) -> anyhow::Result<u8> {
    let mut buf = [0_u8];
    let bytes_written = reader
        .read(&mut buf)
        .context("Failed to read a byte from the reader".to_string())?;
    if bytes_written < buf.len() {
        return Err(anyhow::Error::msg(
            "Not all of the required data was read from the reader",
        ));
    }
    return Ok(buf[0]);
}

pub fn read_bytes<Reader: Read>(size: usize, reader: &mut Reader) -> anyhow::Result<Vec<u8>> {
    let mut buffer = vec![0_u8; size];
    reader
        .read_exact(&mut buffer)
        .context(format!("failed to read {} bytes", size))?;
    Ok(buffer)
}

pub fn read_qname<Reader: Read + Seek>(reader: &mut Reader) -> anyhow::Result<QName> {
    let current_position = reader
        .seek(std::io::SeekFrom::Current(0))
        .context("Failed SeekFrom::Current(0)".to_string())?;
    let byte = read_byte(reader)?;

    let mut name: Vec<u8> = vec![];
    if byte & 0b11000000 == 0b11000000 {
        println!("compressed pointer");
        let next = read_byte(reader)?;
        let offset = (((byte & 0x3F) as u16) << 8) | (next as u16);
        println!("offset: {offset:016b}");
        reader
            .seek(std::io::SeekFrom::Start(offset as u64))
            .context(
                "Failed to set the current position to the start of the pointed name".to_string(),
            )?;
    } else {
        reader
            .seek(std::io::SeekFrom::Start(current_position))
            .context("Failed to seek to the start position".to_string())?;
    }
    loop {
        let size = read_byte(reader)?;
        name.push(size);
        if size == 0 {
            break;
        }
        if size > 63 {
            return Err(anyhow::Error::msg(format!("Invalid label length {size}")));
        }
        if name.len() > 253 {
            return Err(anyhow::Error::msg(format!(
                "The whole name is too large. Max Len is 253: current len is {}",
                name.len()
            )));
        }
        let mut bytes = read_bytes(size as usize, reader)?;
        name.append(&mut bytes);
    }
    if byte & 0xC0 == 0xC0 {
        println!("compressed pointer. restoring buffer");
        reader
            .seek(std::io::SeekFrom::Start(current_position + 2))
            .context("Failed to seek start + 2 when handling name pointers".to_string())?;
    }
    Ok(QName(name))
}
pub fn read_txt_record_from_buffer<Buffer: AsRef<[u8]>>(buffer: Buffer) -> anyhow::Result<String> {
    let slice = buffer.as_ref();
    if slice.len() < 2 {
        return Err(anyhow::Error::msg("Invalid TXT Record."));
    }
    let size = slice.first().unwrap();
    if *size as usize > slice.len() {
        return Err(anyhow::Error::msg(
            "Invalid size byte or buffer is not large enough",
        ));
    }
    println!("txt size {size}");
    let slice = slice
        .get(1..=(*size + 1) as usize)
        .ok_or(anyhow::Error::msg("Range not valid"))?;
    Ok(str::from_utf8(slice)
        .context(
            "Failed to get a string slice from a raw slice while extracting txt record".to_string(),
        )?
        .to_string())
}
pub fn read_u16_be<Reader: Read>(reader: &mut Reader) -> anyhow::Result<u16> {
    let mut buf = [0_u8, 0_u8];
    reader.read_exact(&mut buf).context(format!(
        "Failed to read {} bytes from the reader",
        buf.len()
    ))?;

    let n: u16 = u16::from_be_bytes(buf);
    return Ok(n);
}
pub fn read_u32_be<Reader: Read>(reader: &mut Reader) -> anyhow::Result<u32> {
    let mut buf = [0_u8; 4];
    reader.read_exact(&mut buf).context(format!(
        "Failed to read {} bytes from the reader",
        buf.len()
    ))?;
    let n: u32 = u32::from_be_bytes(buf);
    return Ok(n);
}
pub fn set_opcode(f: &mut HeaderFlags, n: u16) {
    f.0 |= (n & 0b000000000000001111) << 14;
}

pub fn set_query_response(f: &mut HeaderFlags, b: bool) {
    f.0 |= ((b as u16) << 15);
}
pub fn set_recursion_desired(f: &mut HeaderFlags, b: bool) {
    f.0 |= (b as u16 & BOOLEAN_BITFLAG) << 8;
}

#[test]
pub fn test_parse_raw_dns_response() {
    pub const VALID_DNS_RESPONSE_1: &[u8] = &[
        // Header
        0b00000000, 0b00000001, 0b10000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        // QNAME: example.com
        0b00000111, 0b01100101, 0b01111000, 0b01100001, 0b01101101, 0b01110000, 0b01101100,
        0b01100101, 0b00000011, 0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000,
        0b00000001, 0b00000000, 0b00000001, // Answer
        0b11000000, 0b00001100, 0b00000000, 0b00000001, 0b00000000, 0b00000001, 0b00000000,
        0b00000000, 0b00000000, 0b00000010, 0b00000000, 0b00000100, 0b11000000, 0b00000001,
        0b00000010, 0b00000011,
    ];
    let r = parse_raw_dns_response_packet(&VALID_DNS_RESPONSE_1);
    dbg!(&r);
    assert!(r.is_ok());
    pub const VALID_DNS_RESPONSE_2: &[u8] = &[
        0b00000000, 0b00000010, 0b10000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000,
        0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        // QNAME: www.google.com
        0b00000011, 0b01110111, 0b01110111, 0b01110111, 0b00000110, 0b01100111, 0b01101111,
        0b01101111, 0b01100111, 0b01101100, 0b01100101, 0b00000011, 0b01100011, 0b01101111,
        0b01101101, 0b00000000, 0b00000000, 0b00000101, 0b00000000, 0b00000001,
        // Answer
        0b11000000, 0b00001100, 0b00000000, 0b00000101, 0b00000000, 0b00000001, 0b00000000,
        0b00000000, 0b00000000, 0b00000010, 0b00000000, 0b00001111, 0b00000101, 0b01100001,
        0b01101100, 0b01101001, 0b01100001, 0b01110011, 0b00000001, 0b01101100, 0b00000110,
        0b01100111, 0b01101111, 0b01101111, 0b01100111, 0b01101100, 0b01100101, 0b00000011,
        0b01100011, 0b01101111, 0b01101101, 0b00000000,
    ];
    let r2 = parse_raw_dns_response_packet(&VALID_DNS_RESPONSE_2);
    assert!(r2.is_ok());
    pub const VALID_DNS_RESPONSE_3: &[u8] = &[
        0b00010010, 0b00110100, 0b10000001, 0b10000000, 0b00000000, 0b00000001, 0b00000000,
        0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000100, 0b01101001,
        0b01110000, 0b01110110, 0b00110110, 0b00000111, 0b01100101, 0b01111000, 0b01100001,
        0b01101101, 0b01110000, 0b01101100, 0b01100101, 0b00000000, 0b00000000, 0b00011100,
        0b00000000, 0b00000001, 0b11000000, 0b00001100, 0b00000000, 0b00011100, 0b00000000,
        0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00111100, 0b00000000, 0b00010000,
        0b00100000, 0b00000001, 0b00001101, 0b10111000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000001,
    ];
    let r3 = parse_raw_dns_response_packet(&VALID_DNS_RESPONSE_3);
    assert!(r3.is_ok());

    pub const VALID_DNS_RESPONSE_4: &[u8] = &[
        0b10101010, 0b10101010, 0b10000001, 0b10000000, 0b00000000, 0b00000001, 0b00000000,
        0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000111, 0b01100101,
        0b01111000, 0b01100001, 0b01101101, 0b01110000, 0b01101100, 0b01100101, 0b00000011,
        0b01100011, 0b01101111, 0b01101101, 0b00000000, 0b00000000, 0b00010000, 0b00000000,
        0b00000001, 0b11000000, 0b00001100, 0b00000000, 0b00010000, 0b00000000, 0b00000001,
        0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000, 0b00001101, 0b00001011,
        0b01110110, 0b01100101, 0b01110010, 0b01101001, 0b01100110, 0b01101001, 0b01100011,
        0b01100001, 0b01110100, 0b01101001, 0b01101111, 0b01101110,
    ];

    let r4 = parse_raw_dns_response_packet(VALID_DNS_RESPONSE_4);
    assert!(r4.is_ok());
}
pub struct DNSAnswer {
    name: RName,
    r#type: RType,
    class: RClass,
    rdata: RData,
}

// create a function that is general over any container: slice, array, or vector of bytes
// that reads exactly N bytes from the container

pub struct DNSHeaderFlags(u16);
pub struct DNSPacket {
    id: HeaderID,
    flags: HeaderFlags,

    qdcount: QDCount,
    ancount: ANCount,
    nscount: NSCount,
    arcount: ARCount,

    questions: Vec<DNSQuestion>,
    answers: Vec<()>,
    nameservers: Vec<()>,
    authoriative_responses: Vec<()>,
}
// name for responses. DOES respect pointers

pub struct DNSQuestion {
    name: QName,
    r#type: QType,
    r#class: QClass,
}
#[repr(transparent)]
#[derive(PartialEq, Clone, Copy)]
pub struct HeaderFlags(u16);

pub struct QClass;
// name for question. DOES NOT respect pointers
pub struct QName(Vec<u8>);

// question type
pub struct QType;
pub struct RClass;
pub struct RName;
pub struct RType;
type ANCount = u16;
type ARCount = u16;

type HeaderID = u16;
type NSCount = u16;

type QDCount = u16;
type RCode = u16;

use anyhow::{Context, Error, Result};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek};
use std::mem::size_of;
use std::net::Ipv4Addr;
use std::ops::{BitAnd, BitAndAssign, Shr};
