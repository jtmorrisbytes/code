use std::io::{Bytes, Read, Write};
use std::net::UdpSocket;
use std::str::FromStr;

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
    msg: &'static str,
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "context: {}", self.msg)
    }
}
impl std::fmt::Debug for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextError").field("msg", &self.msg).field("inner", &self.source).finish()
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
    fn context(self, msg: &'static str) -> self::LibDNSResult<T>;
}
impl<T, E> LibDNSErrorContext<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, msg: &'static str) -> self::LibDNSResult<T> {
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

    pub fn from_be_bytes(start_offset: usize, bytes: &[u8]) -> LibDNSResult<(usize, Self)> {
        let bytes = bytes
            .get(start_offset..)
            .ok_or(msg!("unable to get byte slice"))?;
        if !bytes.len() < 12 {
            return Err(msg!("the buffer is too small"));
        }

        let mut header: DNSHeader = unsafe { std::mem::zeroed() };

        let id_byte_slice = bytes
            .get(0..=1)
            .ok_or(msg!("unable to get byte slice for id"))?;
        let id_byte_array: [u8; 2] = id_byte_slice
            .try_into()
            .context("unable to convert slice to array for id")?;
        header.id = u16::from_be_bytes(id_byte_array);

        let flags_byte_slice = bytes
            .get(2..=3)
            .ok_or(msg!("unable to get byte slice for flags"))?;
        let flags_byte_array: [u8; 2] = flags_byte_slice
            .try_into()
            .context("Unable to convert flags byte slice into flags byte array")?;
        header.flags = u16::from_be_bytes(flags_byte_array);

        let qdcount_byte_slice = bytes
            .get(4..=5)
            .ok_or(msg!("unable to get byte slice for qdcount"))?;
        let qdcount_byte_array: [u8; 2] = qdcount_byte_slice
            .try_into()
            .context("Unable to convert byte slice into array for qdcount")?;
        header.qdcount = u16::from_be_bytes(qdcount_byte_array);

        let ancount_byte_slice = bytes
            .get(6..=7)
            .ok_or(msg!("Unable to get byte slice for ancount"))?;
        let ancount_byte_array: [u8; 2] = ancount_byte_slice
            .try_into()
            .context("Unable to convert byte slice into array for ancount")?;
        header.ancount = u16::from_be_bytes(ancount_byte_array);

        let nscount_byte_slice = bytes
            .get(8..=9)
            .ok_or(msg!("Unable to get byte slice for nscount"))?;
        let nscount_byte_array: [u8; 2] = nscount_byte_slice
            .try_into()
            .context("Unable to convert byte slice into array for nscount")?;
        header.nscount = u16::from_be_bytes(nscount_byte_array);

        let arcount_byte_slice = bytes
            .get(10..=11)
            .ok_or(msg!("Unable to get byte slice for arcount"))?;
        let arcount_byte_array: [u8; 2] = arcount_byte_slice
            .try_into()
            .context("Unable to convert byte slice to an array for arcount")?;
        header.arcount = u16::from_be_bytes(arcount_byte_array);
        // Optionally populate nscount/arcount if needed
        Ok((std::mem::size_of::<Self>(), header))
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
    pub fn try_from_io_buffer(start_offset: usize, buffer: &[u8]) -> LibDNSResult<(usize,Self)> {
        let mut bytes_taken = 0;

        let byte = buffer.get(start_offset).ok_or(msg!(format!(
            "failed to get first byte at start_offset {start_offset} for buffer {buffer:?}. cant determine if this is a pointer reference or not"
        )))?;

        let mut slice: &[u8] = &[];
        if byte == &0x0C {
            // this is a pointer. the next byte is a size
            let pointer = buffer.get(start_offset + 1).unwrap();
            slice = buffer.get(*pointer as usize..).unwrap();
            bytes_taken = 2;
        } else {
            slice = buffer.get(start_offset..).unwrap();
        }
        // find the null terminator
        assert!(slice.len() > 0);
        bytes_taken = slice.len();

        let mut byte_index = 0;
        let mut buf: Vec<u8> = vec![];
        loop {
            let byte = slice.get(byte_index).unwrap();
            
            // ok now we have a constrained slice that should contain our data

            buf.push(*byte);
            if byte == &0 {
                // assuming that a zero size is a null byte
                break;
            }
            // advance the slice index by one;
            byte_index = byte_index + 1;
            // get the data specified by the length byte_index..byte_index + size
            let label = buffer
                .get(byte_index..byte_index + *byte as usize)
                .ok_or(msg!(format!(
                    "Failed to get the whole slice for label at byte offset {byte_index} for slice {buffer:?}"
                )))?;
            // let _ = std::str::from_utf8(label).context("Utf-8 string slice check failed")?;
            buf.extend_from_slice(label);
            byte_index = byte_index + *byte as usize;
        }
        Ok((bytes_taken,Self(buf)))
    }
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
#[test]
pub fn libdns_test_qname_from_slice() {
    let mut slice = vec![3];
    slice.extend_from_slice(b"www");
    slice.push(6);
    slice.extend_from_slice(b"github");
    slice.push(3);
    slice.extend_from_slice(b"com");
    slice.push(0);
    let r = QName::try_from_io_buffer(0,&slice);
    assert!(r.is_ok())
}
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

impl DNSQuestion {
    pub fn to_be_bytes(self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        let mut qname = self.qname.to_bytes();
        buf.append(&mut qname);
        buf.extend_from_slice(&self.qtype.to_be_bytes());
        buf.extend_from_slice(&self.qclass.to_be_bytes());
        buf
    }
    pub fn from_be_bytes(mut offset: usize, buffer: &[u8]) -> (usize, Self) {
        
        let first_byte = buffer.get(offset).unwrap();
        let (bytes_taken,qname) = QName::try_from_io_buffer(offset, buffer).unwrap();
        if first_byte == &0x0C {
            offset = offset + 2;
        }
        else {
            offset = offset + qname.len();
        }

        // attempt to read the qname. relies on the null byte. may not work if bytes are not aligned!
        // let mut byte_index = 0;

        // qname
        // find the first null byte assume that is the null terminator.
        const U16_BYTE_COUNT: usize = std::mem::size_of::<u16>() / std::mem::size_of::<u8>();
        let qtype = buffer.get(offset..offset + U16_BYTE_COUNT).unwrap();
        let qtype = QType::from_be_bytes([qtype[0],qtype[1]]);
        offset = offset + U16_BYTE_COUNT;

        let qclass_bytes = buffer.get(offset..offset + U16_BYTE_COUNT).unwrap();
        let qclass = u16::from_be_bytes([qclass_bytes[0],qclass_bytes[1]]);
        (
            bytes_taken + U16_BYTE_COUNT * 2,
            Self {
                qname,
                qtype,
                qclass,
            },
        )
    }
}

pub enum DnsAnswer {
    CNAME(QName),
    IPv4Addr(std::net::Ipv4Addr),
    IPv6Addr(std::net::Ipv6Addr),
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
                .context("failed to convert byte slice into u16 value for QType")?,
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
        .context("Failed to bind to a random socket on any host address")?;
    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .context("Failed to set read timeout")?;
    socket
        .connect(dns_address)
        .context("Failed to connect to the dns server")?;

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

    let mut header_bytes = header.to_be_bytes();
    let mut output = vec![];
    output.append(&mut header_bytes);

    for qtype in question_types {
        let question = DNSQuestion {
            qtype: qtype,
            qclass: 1,
            qname: QName::try_from_fdqn(name)
                .context("Failed to convert the input name to a QName value")?,
        };
        output.append(&mut question.to_be_bytes());
    }
    socket.send(&output).unwrap();

    let mut io_buffer: Vec<u8> = vec![0_u8; 512];
    socket.recv(&mut io_buffer).unwrap();

    let mut byte_index = 0;
    let (size, header) = DNSHeader::from_be_bytes(byte_index, &io_buffer).unwrap();
    byte_index = byte_index + size;

    if header.get_query_response() != true {
        panic!(
            "QR bit not set properly. flags: {:16b}, qr: {}",
            header.flags,
            header.get_query_response()
        );
    }
    if header.get_rcode() != RCode::NoError {
        return Err(msg!(format!(
            "The dns server returned the response code {:?}",
            header.get_rcode()
        )));
    }
    let mut questions = vec![];
    // parse the questions
    for _ in 0..header.qdcount {
        let (byte_count, question) = DNSQuestion::from_be_bytes(byte_index, &io_buffer);
        byte_index = byte_index + byte_count;
        dbg!(&question);
        questions.push(question);
        // dbg!(a,b);
    }
    // println!("{:?}",slice);
    // parse the answers
    // println!("slice ancount: {slice:X}");
    let mut answers: Vec<DnsAnswer> = vec![];
    for _ in 0..header.ancount {
        println!("parse answer");
        // let slice = io_buffer.get(byte_index..).unwrap();
        let (size, name) = QName::try_from_io_buffer(byte_index, &io_buffer).unwrap();
        dbg!(&name);
        byte_index = byte_index + size;
        let rtype = io_buffer.get(byte_index..=byte_index + 1).unwrap();
        println!("rtype slice: {rtype:?}");

        let rtype_array: [u8; 2] = rtype.try_into().unwrap();
        let rtype: u16 = u16::from_be_bytes(rtype_array);
        println!(
            "rtype_bytes:{:08b} {:08b} rtype{rtype}",
            rtype_array[0], rtype_array[1]
        );

        byte_index = byte_index + 2;

        let class = io_buffer.get(byte_index..=byte_index + 1).unwrap();
        dbg!(&class);
        let class_array: [u8; 2] = class.try_into().unwrap();
        let class: u16 = u16::from_be_bytes(class_array);
        byte_index = byte_index + 2;

        // i like this better
        let ttl_size_in_bytes = std::mem::size_of::<u32>() / std::mem::size_of::<u8>();
        let ttl: &[u8] = io_buffer
            .get(byte_index..byte_index + ttl_size_in_bytes)
            .expect("TTL slice");
        let ttl: [u8; 2] = [ttl[0], ttl[1]];
        let ttl: u16 = u16::from_be_bytes(ttl);

        byte_index = byte_index + ttl_size_in_bytes;

        // parse and extract the RDATA section
        // first the length of the rdata as a u16
        let sizeof_rdlength = std::mem::size_of::<u16>() / std::mem::size_of::<u8>();
        let rd_length = io_buffer
            .get(byte_index..byte_index + sizeof_rdlength)
            .expect("rdlength");
        let rd_length: [u8; 2] = rd_length.try_into().unwrap();
        let rd_length = u16::from_be_bytes(rd_length);

        byte_index = byte_index + sizeof_rdlength;

        // this is probably not valid but if for some reason there is no rdata just continue to the next answer
        if rd_length == 0 {
            continue;
        }
        // the rdata is of variable size depending
        let rdata = io_buffer
            .get(byte_index..byte_index + rd_length as usize)
            .expect("rdata");
        println!("rdata: {rdata:?}");
        let DnsAnswerKind = match rtype {
            0x0001 => {
                // interpreted exactly as &s[0].&s[1].&s[2].&s[3]
                // octet 0, 1, 2, 3
                // ip v4 address []
                DnsAnswer::IPv4Addr(std::net::Ipv4Addr::from_bits(u32::from_be_bytes([
                    rdata[0], rdata[1], rdata[2], rdata[3],
                ])))
            }
            0x0002 => {
                // name server
                todo!("NS record");
            }
            0x0005 => {
                let (_,cname) = QName::try_from_io_buffer(byte_index, &io_buffer)?;
                DnsAnswer::CNAME(cname)
                // cname
            }
            0x0006 => {
                todo!("SOA record")
                // SOA
                // 0
            }
            0x000C => {
                todo!("PTR record")
                // ptr record
            }
            0x000F => {
                todo!("MX record")
            }
            0x0010 => {
                todo!("TXT record")
            }
            0x001C => {
                todo!("IPV6 record")
            }
            _ => {
                todo!("Other RDATA type values")
            }
        };
        answers.push(DnsAnswerKind);

        // ip_addr (A record)

        byte_index = byte_index + rd_length as usize;
        println!("rdata len {rd_length} rdata:{rdata:?}");
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
        dns_resolve_hostname("1.1.1.1:53", vec![QType::A, QType::AAAA], "www.github.com.").unwrap();
    // println!("{bytes:?}");
    // println!("{bytes2:?}");
}
