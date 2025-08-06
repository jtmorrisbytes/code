use std::io::{BufRead, Bytes, Cursor, Read, Write};
use std::net::{Ipv4Addr, UdpSocket};

pub const DEFAULT_IO_BUFFER_BYTE_COUNT: usize = 1024;
type IoBuffer<const SIZE: usize = DEFAULT_IO_BUFFER_BYTE_COUNT> = [u8; SIZE];


pub fn read_vector_of_bytes<T: AsRef<[u8]>>(size:usize,cursor: &mut std::io::Cursor<T>) -> self::LibDNSResult<Vec<u8>> {
    let mut buf = vec![0_u8;size];
    let err = format!("Failed to read {size} bytes from the cursor into the buffer");
    cursor.read_exact(&mut buf).context(err)?;
    Ok(buf)
}



pub fn read_byte_array<const BYTECOUNT: usize, T:AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<[u8;BYTECOUNT]> {
    let mut buf: [u8;BYTECOUNT] = [0_u8;BYTECOUNT];
    cursor
        .read_exact(&mut buf)
        .context("Failed to read {BYTECOUNT} bytes into an array".to_string())?;
    Ok(buf)
}

// parser / network helper functions
pub fn read_u16_from_cursor_as_be<T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<u16> {
    const SIZE: usize = std::mem::size_of::<u16>() / std::mem::size_of::<u8>();
    let bytes: [u8; SIZE] = read_byte_array(cursor)
        .context("Failed to get byte array from cursor for u16 be".to_string())?;
    Ok(u16::from_be_bytes(bytes))
}
pub fn read_u8_from_cursor<T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<u8> {
    let buf: [u8; 1] = read_byte_array(cursor)?;
    Ok(buf[0])
}

pub fn read_u32_from_cursor_as_be<T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<u32> {
    const SIZE: usize = std::mem::size_of::<u32>() / std::mem::size_of::<u8>();
    let buf: [u8; SIZE] = read_byte_array(cursor)
        .context("Failed to get byte array from cursor for u32 be".to_string())?;
    Ok(u32::from_be_bytes(buf))
}

pub fn read_u128_from_cursor_as_be<T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<u128> {
    const SIZE: usize = std::mem::size_of::<u128>() / std::mem::size_of::<u8>();
    let buf: [u8; SIZE] = read_byte_array(cursor)
        .context("Failed to get byte array from cursor for u128 be".to_string())?;
    Ok(u128::from_be_bytes(buf))
}

pub trait TryFromCursor<T> where T: AsRef<[u8]> {
    fn try_from_cursor_be(_cursor: &mut std::io::Cursor<T>) -> self::LibDNSResult<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn try_from_cursor_le(_cursor: &mut std::io::Cursor<T>) -> self::LibDNSResult<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

#[repr(transparent)]
pub struct DNSHeaderID(pub u16);
impl DNSHeaderID {
    pub fn as_u16(&self) -> u16 {
        self.0
    }
    pub fn as_byte_array_be(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl<T> TryFromCursor<T> for DNSHeaderID where T: AsRef<[u8]> {
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
    where
        Self: Sized,
    {
        Ok(Self(read_u16_from_cursor_as_be(cursor).context(
            "Failed to read u16 from cursor for DNSHeaderID".to_string(),
        )?))
    }
}

pub struct DNSHeaderFlags(u16);

impl<T> TryFromCursor<T> for DNSHeaderFlags where T: AsRef<[u8]> {
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
    where
        Self: Sized,
    {
        Ok(Self(read_u16_from_cursor_as_be(cursor).context(
            "Failed to read u16 from cursor for DNSHeaderFlags".to_string(),
        )?))
    }
}

impl DNSHeaderFlags {
    pub fn get_query_response(&self) -> bool {
        ((self.0 & QR_BITFLAG) >> 15) != 0
    }
    pub fn get_opcode(&self) -> u16 {
        (self.0 & OPCODE_BITFLAG) >> 11
    }
    pub fn get_authoritative_answer(&self) -> bool {
        ((self.0 & AA_BITFLAG) >> 10) != 0
    }
    pub fn get_truncated(&self) -> bool {
        ((self.0 & TC_BITFLAG) >> 9) != 0
    }
    pub fn get_recursion_desired(&self) -> bool {
        ((self.0 & RD_BITFLAG) >> 8) != 0
    }
    pub fn get_recursion_available(&self) -> bool {
        ((self.0 & RA_BITFLAG) >> 7) != 0
    }
    pub fn get_z(&self) -> bool {
        ((self.0 & Z_BITFLAG) >> 6) != 0
    }
    pub fn get_ad(&self) -> bool {
        (self.0 & AD_BITFLAG) >> 5 != 0
    }
    pub fn get_cd(&self) -> bool {
        ((self.0 & CD_BITFLAG) >> 4) != 0
    }
    pub fn get_rcode(&self) -> RCode {
        match self.0 & RCODE_BITFLAG {
            0 => RCode::NoError,
            1 => RCode::FormErr,
            2 => RCode::ServFail,
            3 => RCode::NXDomain,
            4 => RCode::NotImp,
            5 => RCode::Refused,
            unknown => panic!("Invalid RCode {unknown}"),
        }
    }

    pub fn set_query_response(&mut self, b: bool) {
        self.0 |= (b as u16) << 15;
    }
    pub fn set_opcode(&mut self, n: u16) {
        self.0 |= (n & 0b000000000000001111) << 14;
    }
    pub fn set_recursion_desired(&mut self, b: bool) {
        self.0 |= (b as u16 & BOOLEAN_BITFLAG) << 8;
    }
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

pub struct LibDNSError {
    inner: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl std::fmt::Display for LibDNSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LibDNSError: {}", self.inner)?;
        let mut err = self.inner.source();
        while let Some(some_err) = err {
            writeln!(f, "because: {}", some_err)?;
            err = some_err.source();
        }
        Ok(())
    }
}
impl std::fmt::Debug for LibDNSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b = f.debug_struct("LibDNSError");

        b.field("inner", &self.inner);
        // let err = self.inner;
        b.finish()
    }
}

#[derive(Debug)]
pub struct MessageError {
    msg: String,
}

impl MessageError {
    pub fn new(msg: impl Into<String>) -> Self {
        MessageError { msg: msg.into() }
    }
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for MessageError {}

macro_rules! msg {
    ($msg:expr) => {
        LibDNSError {
            inner: Box::new(MessageError::new($msg)),
        }
    };
}

pub struct ContextError {
    msg: String,
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "context: {}", self.msg)
    }
}
impl std::fmt::Debug for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextError")
            .field("msg", &self.msg)
            .field("inner", &self.source)
            .finish()
    }
}
impl std::error::Error for ContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}
pub type LibDNSResult<T> = std::result::Result<T, self::LibDNSError>;

impl std::convert::From<ContextError> for LibDNSError {
    fn from(error: ContextError) -> Self {
        Self {
            inner: Box::new(error),
        }
    }
}
pub trait LibDNSErrorContext<T> {
    fn context(self, msg: String) -> self::LibDNSResult<T>;
}
impl<T, E> LibDNSErrorContext<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, msg: String) -> self::LibDNSResult<T> {
        self.map_err(|e| {
            let wrapped = ContextError {
                msg,
                source: Box::new(e),
            };
            LibDNSError::from(wrapped)
        })
    }
}

impl std::error::Error for self::LibDNSError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

// ===== Constants =====
pub const OPCODE_A: u16 = 0;

// Bit order for flags left -> right
const QR_BITFLAG: u16 = 0b1000000000000000;
const OPCODE_BITFLAG: u16 = 0b0111100000000000;
const AA_BITFLAG: u16 = 0b0000010000000000;
const TC_BITFLAG: u16 = 0b0000001000000000;
const RD_BITFLAG: u16 = 0b0000000100000000;
const RA_BITFLAG: u16 = 0b0000000010000000;
const Z_BITFLAG: u16 = 0b0000000001000000;
const AD_BITFLAG: u16 = 0b0000000000100000;
const CD_BITFLAG: u16 = 0b0000000000010000;
const RCODE_BITFLAG: u16 = 0b0000000000001111;
const BOOLEAN_BITFLAG: u16 = 0b0000000000000001;

// ===== Structs and Implementations =====
#[derive(Debug)]
pub struct DNSHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl<T> TryFromCursor<T> for DNSHeader where T: AsRef<[u8]> {
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> self::LibDNSResult<Self>
        where
            Self: Sized, {
        let id = DNSHeaderID::try_from_cursor_be(cursor)?;
        let flags= DNSHeaderFlags::try_from_cursor_be(cursor)?;
        let qdcount = read_u16_from_cursor_as_be(cursor)?;
        let ancount = read_u16_from_cursor_as_be(cursor)?;
        let nscount = read_u16_from_cursor_as_be(cursor)?;
        let arcount = read_u16_from_cursor_as_be(cursor)?;
        Ok(Self{id:id.0,flags:flags.0,qdcount,ancount,nscount,arcount})
    }
}

impl DNSHeader {
    pub fn get_query_response(&self) -> bool {
        ((self.flags & QR_BITFLAG) >> 15) != 0
    }
    pub fn get_opcode(&self) -> u16 {
        (self.flags & OPCODE_BITFLAG) >> 11
    }
    pub fn get_authoritative_answer(&self) -> bool {
        ((self.flags & AA_BITFLAG) >> 10) != 0
    }
    pub fn get_truncated(&self) -> bool {
        ((self.flags & TC_BITFLAG) >> 9) != 0
    }
    pub fn get_recursion_desired(&self) -> bool {
        ((self.flags & RD_BITFLAG) >> 8) != 0
    }
    pub fn get_recursion_available(&self) -> bool {
        ((self.flags & RA_BITFLAG) >> 7) != 0
    }
    pub fn get_z(&self) -> bool {
        ((self.flags & Z_BITFLAG) >> 6) != 0
    }
    pub fn get_ad(&self) -> bool {
        (self.flags & AD_BITFLAG) >> 5 != 0
    }
    pub fn get_cd(&self) -> bool {
        ((self.flags & CD_BITFLAG) >> 4) != 0
    }
    pub fn get_rcode(&self) -> RCode {
        match self.flags & RCODE_BITFLAG {
            0 => RCode::NoError,
            1 => RCode::FormErr,
            2 => RCode::ServFail,
            3 => RCode::NXDomain,
            4 => RCode::NotImp,
            5 => RCode::Refused,
            unknown => panic!("Invalid RCode {unknown}"),
        }
    }

    pub fn set_query_response(&mut self, b: bool) {
        self.flags |= (b as u16) << 15;
    }
    pub fn set_opcode(&mut self, n: u16) {
        self.flags |= (n & 0b000000000000001111) << 14;
    }
    pub fn set_recursion_desired(&mut self, b: bool) {
        self.flags |= (b as u16 & BOOLEAN_BITFLAG) << 8;
    }

    pub fn to_be_bytes(self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.extend_from_slice(&self.id.to_be_bytes());
        buf.extend_from_slice(&self.flags.to_be_bytes());
        buf.extend_from_slice(&self.qdcount.to_be_bytes());
        buf.extend_from_slice(&self.ancount.to_be_bytes());
        buf.extend_from_slice(&self.nscount.to_be_bytes());
        buf.extend_from_slice(&self.arcount.to_be_bytes());
        buf
    }
}

// a wrapper over a DNS Query Name
#[derive(Debug)]
pub struct QName(Vec<u8>);
impl QName {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    // I know that I should really validate this but just dont care rn
    pub fn try_from_slice(buffer: &[u8]) -> self::LibDNSResult<Self> {
        let mut cursor = std::io::Cursor::new(buffer);
        let start = cursor.position();
        let first = read_u8_from_cursor(&mut cursor)?;
        if first == 0x0C {
            let offset = read_u8_from_cursor(&mut cursor)?;
            cursor.set_position(offset as u64);
        }
        let mut buf = Vec::new();
        cursor.read_until(0, &mut buf);
        if first == 0x0C {
            cursor.set_position(start + (std::mem::size_of::<u8>() * 2) as u64);
        }

        Ok(Self(buf))
    }

    // pub fn try_from_io_buffer(start_offset: usize, buffer: &[u8]) -> LibDNSResult<(usize, Self)> {
    //     let mut bytes_taken = 0;

    //     let byte = buffer.get(start_offset).ok_or(msg!(format!(
    //         "failed to get first byte at start_offset {start_offset} for buffer {buffer:?}. cant determine if this is a pointer reference or not"
    //     )))?;

    //     let mut slice: &[u8] = &[];
    //     if byte == &0x0C {
    //         // this is a pointer. the next byte is a size
    //         let pointer = buffer.get(start_offset + 1).unwrap();
    //         slice = buffer.get(*pointer as usize..).unwrap();
    //         bytes_taken = 2;
    //     } else {
    //         slice = buffer.get(start_offset..).unwrap();
    //     }
    //     // find the null terminator
    //     assert!(slice.len() > 0);
    //     bytes_taken = slice.len();

    //     let mut byte_index = 0;
    //     let mut buf: Vec<u8> = vec![];
    //     loop {
    //         let byte = slice.get(byte_index).unwrap();

    //         // ok now we have a constrained slice that should contain our data

    //         buf.push(*byte);
    //         if byte == &0 {
    //             // assuming that a zero size is a null byte
    //             break;
    //         }
    //         // advance the slice index by one;
    //         byte_index = byte_index + 1;
    //         // get the data specified by the length byte_index..byte_index + size
    //         let label = buffer
    //             .get(byte_index..byte_index + *byte as usize)
    //             .ok_or(msg!(format!(
    //                 "Failed to get the whole slice for label at byte offset {byte_index} for slice {buffer:?}"
    //             )))?;
    //         // let _ = std::str::from_utf8(label).context("Utf-8 string slice check failed")?;
    //         buf.extend_from_slice(label);
    //         byte_index = byte_index + *byte as usize;
    //     }
    //     Ok((bytes_taken, Self(buf)))
    // }
    pub fn try_from_fdqn(s: &str) -> LibDNSResult<Self> {
        if s.len() == 0 {
            return Err(msg!("A zero size string is not a valid FDQN"));
        }
        if s == "." {
            return Ok(Self(vec![0]));
        }
        if !s.ends_with(".") {
            // FDQNS must start end with a .
            return Err(msg!("A valid FDQN must end with a '.'"));
        }
        let mut qname = vec![];
        // this is the root domain
        let mut label = vec![];

        for character in s.chars() {
            if character == '.' {
                if label.len() > 63 {
                    // label too long
                    return Err(msg!(format!(
                        "the label {label:?} is too long. Max len is 63 chars"
                    )));
                }
                qname.push(label.len() as u8);
                qname.extend_from_slice(&label);
                label.clear();
            } else {
                label.push(character as u8);
            }
        }
        qname.push(0);
        Ok(Self(qname))
    }
    pub fn to_fdqn(&self) -> String {
        println!("{:?}", self.0);
        if self.0.len() == 0 || self.0 == vec![0] {
            return ".".to_string();
        }
        let mut s = String::new();
        let mut byte_index = 0;
        loop {
            let size = (self.0[byte_index]) as usize;
            if size == 0 {
                break;
            }
            byte_index = byte_index + 1;
            let label = &self.0[byte_index..byte_index + size];
            let label = str::from_utf8(label).unwrap();
            s = s + label + ".";
            byte_index = byte_index + size;
        }
        s
    }
}
impl<T>  TryFromCursor<T> for QName where T: AsRef<[u8]> {
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
        where
            Self: Sized, {
        let start = cursor.position();
        let first_byte = read_u8_from_cursor(cursor)?;
        if first_byte == 0x0C {
            let offset = read_u8_from_cursor(cursor)?;
            cursor.set_position(offset as u64);
        }
        let mut buf: Vec<u8> = Vec::new();
        loop {
            let size = read_u8_from_cursor(cursor)?;
            buf.push(size);
            for _ in 0..size {
                let character = read_u8_from_cursor(cursor)?;
                buf.push(character);
            }
            if size == 0 {
                break;
            }
        }
        if first_byte == 0x0C {
            cursor.set_position( start + 2);
        }

        if buf.len() > 255 {
            return Err(msg!(format!("Buffer too large for QName. max size 255 {}",buf.len())))
        }

        Ok(Self(buf))
        // validate the buffer? probably a good idea


        
    }
}
// #[test]
// pub fn libdns_test_qname_from_slice() {
//     let mut slice = vec![3];
//     slice.extend_from_slice(b"www");
//     slice.push(6);
//     slice.extend_from_slice(b"github");
//     slice.push(3);
//     slice.extend_from_slice(b"com");
//     slice.push(0);
//     let r = QName::try_from_io_buffer(0, &slice);
//     assert!(r.is_ok())
// }
#[test]
pub fn libdns_test_qname_from_fdqn() {
    let fdqn = "";
    let r = QName::try_from_fdqn(fdqn);
    assert!(r.is_err());
    // the root domain
    let fdqn = ".";
    let r = QName::try_from_fdqn(fdqn);
    assert!(r.is_ok());

    let fdqn = "a.";
    let r = QName::try_from_fdqn(fdqn);
    assert!(r.is_ok());

    let fdqn = "www.github.com.";
    let q = QName::try_from_fdqn(fdqn).unwrap();
    let s = q.to_fdqn();
    println!("{:?},{}", &q, &s);
    assert_eq!(fdqn, s);
}

#[derive(Debug)]
pub struct DNSQuestion {
    qname: QName,
    qtype: QType,
    qclass: u16,
}
impl<T> TryFromCursor<T> for DNSQuestion where T: AsRef<[u8]> {
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
        where
            Self: Sized, {
        let qname = QName::try_from_cursor_be(cursor).context("Failed to parse Qname for DNSQuestion".to_string())?;
        let qtype = QType::try_from_cursor_be(cursor).context("Failed to parse QType for DNSQuestion".to_string())?;
        let qclass = read_u16_from_cursor_as_be(cursor)?;
        Ok(Self{qname,qtype,qclass})
    
        
    }
}



impl DNSQuestion {
    pub fn to_be_bytes(self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        let mut qname = self.qname.to_bytes();
        buf.append(&mut qname);
        buf.extend_from_slice(&self.qtype.to_be_bytes());
        buf.extend_from_slice(&self.qclass.to_be_bytes());
        buf
    }
}
//this type generated by CHATGPT (shrug)
#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Type {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    NAPTR = 35,
    OPT = 41,
    DS = 43,
    RRSIG = 46,
    DNSKEY = 48,
    TLSA = 52,
    SVCB = 64,
    HTTPS = 65,
    ANY = 255,

    // Reserved / uncommon values
    UNKNOWN(u16),
}
// this impl generated by chatgpt
impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Type::A,
            2 => Type::NS,
            5 => Type::CNAME,
            6 => Type::SOA,
            12 => Type::PTR,
            15 => Type::MX,
            16 => Type::TXT,
            28 => Type::AAAA,
            33 => Type::SRV,
            35 => Type::NAPTR,
            41 => Type::OPT,
            43 => Type::DS,
            46 => Type::RRSIG,
            48 => Type::DNSKEY,
            52 => Type::TLSA,
            64 => Type::SVCB,
            65 => Type::HTTPS,
            255 => Type::ANY,
            other => Type::UNKNOWN(other),
        }
    }
}

// the following code for  was generated by chatgpt

#[derive(Debug, PartialEq, Eq)]
pub struct  ARecord {
    pub address: std::net::Ipv4Addr,
}

impl<T> TryFromCursor<T> for ARecord  where T: AsRef<[u8]> {
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
        where
            Self: Sized, {
        let bits = read_u32_from_cursor_as_be(cursor)?;
        let ipaddr= std::net::Ipv4Addr::from_bits(bits);
        Ok((Self{address:ipaddr}))
    }
}

impl  {
    fn try_from_slice_be(slice: &[u8]) -> self::LibDNSResult<Self> {
        let mut cursor = std::io::Cursor::new(slice);
        Self::try_from_cursor_be(&mut cursor)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub address: std::net::Ipv6Addr,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub cname: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub nsdname: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub ptrdname: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub preference: u16,
    pub exchange: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub text: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub mname: String,
    pub rname: String,
    pub serial: u32,
    pub refresh: u32,
    pub retry: u32,
    pub expire: u32,
    pub minimum: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub priority: u16,
    pub weight: u16,
    pub port: u16,
    pub target: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub flags: u16,
    pub protocol: u8,
    pub algorithm: u8,
    pub public_key: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub type_covered: u16,
    pub algorithm: u8,
    pub labels: u8,
    pub original_ttl: u32,
    pub signature_expiration: u32,
    pub signature_inception: u32,
    pub key_tag: u16,
    pub signer_name: String,
    pub signature: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub key_tag: u16,
    pub algorithm: u8,
    pub digest_type: u8,
    pub digest: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub usage: u8,
    pub selector: u8,
    pub matching_type: u8,
    pub cert_data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct  {
    pub priority: u16,
    pub target: String,
    pub params: Vec<(u16, Vec<u8>)>,
}



#[derive(Debug, PartialEq, Eq)]
pub enum  {
    A(),
    AAAA(),
    CNAME(),
    NS(),
    PTR(),
    MX(),
    TXT(),
    SOA(),
    SRV(),
    DNSKEY(),
    RRSIG(),
    DS(),
    TLSA(),
    SVCB(),
    HTTPS(), // HTTPS uses the same format as SVCB
    UNKNOWN {
        _type: u16,
        data: Vec<u8>,
    },
}


pub struct DnsAnswer {
    name: QName,
    rtype: QType,
    class: u16,
    rdata: 
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum RCode {
    NoError = 0,
    FormErr,
    ServFail,
    NXDomain,
    NotImp,
    Refused,
    YXDomain,
    YXRRSet,
    NXRRSet,
    NotAuth,
}

#[derive(Debug, Default)]
#[repr(u16)]
pub enum QType {
    #[default]
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    Any = 255,
}
impl<T> TryFromCursor<T> for QType where T: AsRef<[u8]> {
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
        where
            Self: Sized, {
        let n = read_u16_from_cursor_as_be(cursor)?;
        Self::try_from(n)
    }
}
impl std::convert::TryFrom<u16> for QType {
    type Error = LibDNSError;
    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::NS),
            5 => Ok(Self::CNAME),
            6 => Ok(Self::SOA),
            12 => Ok(Self::PTR),
            15 => Ok(Self::MX),
            16 => Ok(Self::TXT),
            28 => Ok(Self::AAAA),
            33 => Ok(Self::SRV),
            255 => Ok(Self::Any),
            _ => Err(msg!(format!(
                "The u16 value {value} is not valid for type QType"
            ))),
        }
    }
}

impl std::convert::TryFrom<[u8; 2]> for QType {
    type Error = LibDNSError;
    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        let n = u16::from_be_bytes(value);
        Self::try_from(n)
    }
}
impl std::convert::TryFrom<&[u8; 2]> for QType {
    type Error = LibDNSError;
    fn try_from(value: &[u8; 2]) -> Result<Self, Self::Error> {
        let n = u16::from_be_bytes(*value);
        Self::try_from(n)
    }
}
impl std::convert::TryFrom<&[u8]> for QType {
    type Error = LibDNSError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut arr = [0, 0];
        arr.copy_from_slice(value);
        let n = u16::from_be_bytes(arr);
        Self::try_from(n)
    }
}
impl std::convert::TryFrom<Vec<u8>> for QType {
    type Error = LibDNSError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let bytes = value.get(0..2).ok_or(msg!(
            "QType TryFrom<Vec<u8>> Failed to get 2 bytes for u16 converson"
        ))?;
        let n = u16::from_be_bytes(
            bytes
                .try_into()
                .context("failed to convert byte slice into u16 value for QType".to_string())?,
        );
        Self::try_from(n)
    }
}

pub struct DNSResponse {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DnsAnswer>,
}

// impl std::convert::Into<u16> for QType {
//     fn into(self) -> u16 {
//         (self as u16).to_be()
//     }
// }
// impl std::convert::Into<[u8;2]> for QType {
//     fn into(self) -> [u8;2] {
//         self.to_be_bytes()
//     }
// }

impl QType {
    pub fn to_be_bytes(self) -> [u8; 2] {
        (self as u16).to_be_bytes()
    }
    pub fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self::try_from(bytes).expect("Valid byte sequence for QType")
    }
}
// ===== Functions =====
pub fn hostname_is_valid(name: &str) -> bool {
    if name.len() > 253 {
        return false;
    }
    if name.starts_with('-') && name.ends_with('-') {
        return false;
    }

    for segment in name.split('.') {
        if segment.len() > 63 {
            return false;
        }
        for char in segment.chars() {
            if ('\0'..='/').contains(&char)
                || (':'..='@').contains(&char)
                || ('['..='`').contains(&char)
                || char >= '{'
            {
                return false;
            }
        }
    }

    true
}

/// for a DNS answer field: handles compressed responses

pub fn qname_to_bytes(qname: &str) -> Vec<u8> {
    let mut output = Vec::<u8>::new();
    for segment in qname.split('.') {
        output.push(segment.bytes().len() as u8);
        output.extend_from_slice(segment.as_bytes());
    }
    output.push(0);
    output
}

pub fn dns_resolve_hostname(
    dns_address: &str,
    question_types: Vec<QType>,
    name: &str,
) -> LibDNSResult<DNSResponse> {
    if !hostname_is_valid(name) {
        return Err(msg!(format!(
            "The string input '{name}' is not a valid hostname"
        )));
    }

    let socket = UdpSocket::bind("0.0.0.0:0")
        .context("Failed to bind to a random socket on any host address".to_string())?;
    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .context("Failed to set read timeout".to_string())?;
    socket
        .connect(dns_address)
        .context("Failed to connect to the dns server".to_string())?;

    let mut header = DNSHeader {
        id: 0xdead,
        flags: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };
    header.set_query_response(false);
    header.set_opcode(0);
    header.set_recursion_desired(true);
    header.qdcount = question_types.len() as u16;


    println!("request_header {header:?}");

    let mut header_bytes = header.to_be_bytes();
    let mut output = vec![];
    output.append(&mut header_bytes);
    
    for qtype in question_types {
        let question = DNSQuestion {
            qtype: qtype,
            qclass: 1,
            qname: QName::try_from_fdqn(name)
            .context("Failed to convert the input name to a QName value".to_string())?,
        };
        output.append(&mut question.to_be_bytes());
    }
    println!("output {output:?}");
    socket.send(&output).unwrap();

    // maximum safe size is 512, but we will chose a common MTU size
    let mut io_buffer: [u8; DEFAULT_IO_BUFFER_BYTE_COUNT] = [0_u8; DEFAULT_IO_BUFFER_BYTE_COUNT];
    socket.recv(&mut io_buffer).unwrap();
    println!("io_buffer: {io_buffer:?}");

    // instead of indexing and math: try using std::io::Cursor
    let mut cursor = std::io::Cursor::new(&io_buffer);
    // try to parse the dns header again from a cursor instead
    let header = DNSHeader::try_from_cursor_be(&mut cursor)?;
    println!("response_header: {header:?}");
    if header.get_query_response() != true {
        panic!(
            "QR bit not set properly. flags: {:16b}, qr: {}",
            header.flags,
            header.get_query_response()
        );
    }

    // we probably still want to parse the rest of the information we have and return an empty response here
    if header.get_rcode() != RCode::NoError {
        return Err(msg!(format!(
            "The dns server returned the response code {:?}",
            header.get_rcode()
        )));
    }
    let mut questions = vec![];
    // parse the questions
    for _ in 0..header.qdcount {
        let position = cursor.position();
        // try to parse the question. if that fails, then there are less questions than specified in the header, we are not parsing right, or the packet is badly formatted.
        // put the cursor back and break.
        let question = DNSQuestion::try_from_cursor_be(&mut cursor);
        if question.is_err() {
            cursor.set_position(position);
            break;
        }
        let question = question.unwrap();
        questions.push(question);
        // dbg!(a,b);
    }
    // println!("{:?}",slice);
    // parse the answers
    // println!("slice ancount: {slice:X}");
    let mut answers: Vec<> = vec![];
    for _ in 0..header.ancount {
        println!("parse answer");
        // let slice = io_buffer.get(byte_index..).unwrap();
        let name = QName::try_from_cursor_be(&mut cursor).context("failed to parse qname for answer".to_string()).unwrap();
        let rtype = read_u16_from_cursor_as_be(&mut cursor)?;
        let class = read_u16_from_cursor_as_be(&mut cursor)?;
        let ttl = read_u32_from_cursor_as_be(&mut cursor)?;
        // parse and extract the RDATA section
        // first the length of the rdata as a u16
        let rd_length = read_u16_from_cursor_as_be(&mut cursor)?;


        // this is probably not valid but if for some reason there is no rdata just continue to the next answer
        if rd_length == 0 {
            continue;
        }
        // the rdata is of variable size depending
        let rdata = read_vector_of_bytes(rd_length as usize, &mut cursor)?;


        println!("rdata: {rdata:?}");
        let  = match rtype {
            0x0001 => {
                // interpreted exactly as &s[0].&s[1].&s[2].&s[3]
                // octet 0, 1, 2, 3
                // ip v4 address []
                // read 4 bytes
                let octests:[u8;4] = [rdata[0],rdata[1],rdata[2],rdata[3]];
                let n = u32::from_be_bytes(octests);        
                let  = ::A( { address: Ipv4Addr::from_bits(n) });
                answers.push(DnsAnswer{name,rtype,class,rdata})
            }
            0x0002 => {
                // name server
                todo!("NS ");
            }
            0x0005 => {
                println!("Cname bytes: {rdata:?}");
                let cname = QName::try_from_slice(&rdata)?;
                ::CNAME( { cname: cname.to_fdqn() })
                // cname
            }
            0x0006 => {
                todo!("SOA ")
                // SOA
                // 0
            }
            0x000C => {
                todo!("PTR ")
                // ptr 
            }
            0x000F => {
                todo!("MX ")
            }
            0x0010 => {
                todo!("TXT ")
            }
            0x001C => {
                todo!("IPV6 ")
            }
            _ => {
                todo!("Other RDATA type values")
            }
        };
        answers.push();

        // ip_addr (A )
    }
    // dbg!(header);

    Ok(DNSResponse {
        header,
        questions,
        answers,
    })
}

// ===== Tests =====
#[test]
pub fn libdns_test_qname_to_bytes() {
    let bytes = qname_to_bytes("github.com");
    assert_eq!(
        bytes,
        [
            0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f, 0x6d, 0x00
        ]
    );
}

#[test]
pub fn test_resolve_hostname() {
    let response =
        dns_resolve_hostname("8.8.4.4:53", vec![QType::A, QType::AAAA], "www.github.com.").context("test failed".to_string()).ok();
        dns_resolve_hostname("1.1.1.1:53", vec![QType::A, QType::AAAA], "www.github.com.").context("test failed".to_string()).unwrap();
    
    // println!("{bytes:?}");
    // println!("{bytes2:?}");
}
