const AA_BITFLAG: u16 = 0b0000010000000000;
const AD_BITFLAG: u16 = 0b0000000000100000;
const BOOLEAN_BITFLAG: u16 = 0b0000000000000001;
const CD_BITFLAG: u16 = 0b0000000000010000;
const OPCODE_BITFLAG: u16 = 0b0111100000000000;

// Bit order for flags left -> right
const QR_BITFLAG: u16 = 0b1000000000000000;
const RA_BITFLAG: u16 = 0b0000000010000000;
const RCODE_BITFLAG: u16 = 0b0000000000001111;
const RD_BITFLAG: u16 = 0b0000000100000000;
const TC_BITFLAG: u16 = 0b0000001000000000;
const Z_BITFLAG: u16 = 0b0000000001000000;

impl<T> TryFromCursor<T> for DNSHeader
where
    T: AsRef<[u8]>,
{
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> self::LibDNSResult<Self>
    where
        Self: Sized,
    {
        let id = DNSHeaderID::try_from_cursor_be(cursor)?;
        let flags = DNSHeaderFlags::try_from_cursor_be(cursor)?;
        let qdcount = read_u16_from_cursor_as_be(cursor)?;
        let ancount = read_u16_from_cursor_as_be(cursor)?;
        let nscount = read_u16_from_cursor_as_be(cursor)?;
        let arcount = read_u16_from_cursor_as_be(cursor)?;
        Ok(Self {
            id: id.0,
            flags: flags.0,
            qdcount,
            ancount,
            nscount,
            arcount,
        })
    }
}

impl<T> TryFromCursor<T> for DNSHeaderFlags
where
    T: AsRef<[u8]>,
{
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
    where
        Self: Sized,
    {
        Ok(Self(read_u16_from_cursor_as_be(cursor).context(
            "Failed to read u16 from cursor for DNSHeaderFlags".to_string(),
        )?))
    }
}

impl<T> TryFromCursor<T> for DNSHeaderID
where
    T: AsRef<[u8]>,
{
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
    where
        Self: Sized,
    {
        Ok(Self(read_u16_from_cursor_as_be(cursor).context(
            "Failed to read u16 from cursor for DNSHeaderID".to_string(),
        )?))
    }
}
impl<T> TryFromCursor<T> for DNSQuestion
where
    T: AsRef<[u8]>,
{
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
    where
        Self: Sized,
    {
        let qname = QName::try_from_cursor_be(cursor)
            .context("Failed to parse Qname for DNSQuestion".to_string())?;
        let qtype = QType::try_from_cursor_be(cursor)
            .context("Failed to parse QType for DNSQuestion".to_string())?;
        let qclass = read_u16_from_cursor_as_be(cursor)?;
        Ok(Self {
            qname,
            qtype,
            qclass,
        })
    }
}
impl<T> TryFromCursor<T> for QName
where
    T: AsRef<[u8]>,
{
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
    where
        Self: Sized,
    {
        let start = cursor.position();
        let first_byte = read_u8_from_cursor(cursor)?;
        if first_byte == 0x0C {
            let offset = read_u8_from_cursor(cursor)?;
            cursor.set_position(offset as u64);
        }
        let mut buf: Vec<u8> = Vec::new();
        loop {
            let size = read_u8_from_cursor(cursor)?;
            if size > 63 {
                return Err(anyhow::Error::msg(format!(
                    "Invalid size for qname label: {size}"
                )));
            }
            println!("size: {size}");
            if size == 0 {
                break;
            }
            buf.push(size);
            for _ in 0..size {
                let character = read_u8_from_cursor(cursor)?;
                buf.push(character);
            }
        }
        if first_byte == 0x0C {
            cursor.set_position(start + 2);
        }

        if buf.len() > 255 {
            return Err(anyhow::Error::msg(format!(
                "Buffer too large for QName. max size 255 {}",
                buf.len()
            )));
        }

        Ok(Self(buf))
        // validate the buffer? probably a good idea
    }
}
impl<T> TryFromCursor<T> for QType
where
    T: AsRef<[u8]>,
{
    fn try_from_cursor_be(cursor: &mut std::io::Cursor<T>) -> crate::LibDNSResult<Self>
    where
        Self: Sized,
    {
        let n = read_u16_from_cursor_as_be(cursor)?;
        Self::try_from(n)
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
impl DNSHeaderID {
    pub fn as_u16(&self) -> u16 {
        self.0
    }
    pub fn as_byte_array_be(&self) -> [u8; 2] {
        self.0.to_be_bytes()
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
impl DNSRecord {
    pub fn parse_from_raw_parts<RData>(
        name: QName,
        rtype: u16,
        class: u16,
        ttl: u32,
        rdata: RData,
    ) -> Self
    where
        RData: AsRef<[u8]>,
    {
        let name = name.to_fdqn();
        let rtype = DnsRType::from(rtype);
        let class = DNSClass::from(class);
        let mut cursor = std::io::Cursor::new(rdata.as_ref());

        let kind = match rtype {
            DnsRType::A => {
                let bits = read_u32_from_cursor_as_be(&mut cursor).unwrap();
                let addr = std::net::Ipv4Addr::from_bits(bits);
                DNSRecordKind::A(addr)
            }
            DnsRType::AAAA => {
                let bits = read_u128_from_cursor_as_be(&mut cursor).unwrap();
                DNSRecordKind::AAAA(Ipv6Addr::from_bits(bits))
            }
            other => {
                todo!("implement {other:?}")
            }
        };
        Self {
            domain: name,
            ttl,
            class,
            kind,
        }
    }
}
impl From<u16> for DNSClass {
    fn from(value: u16) -> Self {
        match value {
            1 => DNSClass::IN,
            2 => DNSClass::CS,
            3 => DNSClass::CH,
            4 => DNSClass::HS,
            254 => DNSClass::NONE,
            255 => DNSClass::ANY,
            other => DNSClass::Unknown(other),
        }
    }
}

impl From<u16> for DnsRType {
    fn from(value: u16) -> Self {
        match value {
            1 => DnsRType::A,
            2 => DnsRType::NS,
            5 => DnsRType::CNAME,
            6 => DnsRType::SOA,
            12 => DnsRType::PTR,
            15 => DnsRType::MX,
            16 => DnsRType::TXT,
            28 => DnsRType::AAAA,
            33 => DnsRType::SRV,
            43 => DnsRType::DS,
            46 => DnsRType::RRSIG,
            48 => DnsRType::DNSKEY,
            52 => DnsRType::TLSA,
            64 => DnsRType::SVCB,
            65 => DnsRType::HTTPS,
            41 => DnsRType::OPT,
            252 => DnsRType::AXFR,
            253 => DnsRType::MAILB,
            254 => DnsRType::MAILA,
            255 => DnsRType::ANY,
            other => DnsRType::Unknown(other),
        }
    }
}
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
        cursor.read_until(0, &mut buf).unwrap();
        if first == 0x0C {
            cursor.set_position(start + (std::mem::size_of::<u8>() * 2) as u64);
        }

        Ok(Self(buf))
    }

    pub fn try_from_fdqn(s: &str) -> LibDNSResult<Self> {
        if s.len() == 0 {
            return Err(anyhow::Error::msg("A zero size string is not a valid FDQN"));
        }
        if s == "." {
            return Ok(Self(vec![0]));
        }
        if !s.ends_with(".") {
            // FDQNS must start end with a .
            return Err(anyhow::Error::msg("A valid FDQN must end with a '.'"));
        }
        let mut qname = vec![];
        // this is the root domain
        let mut label = vec![];

        for character in s.chars() {
            if character == '.' {
                if label.len() > 63 {
                    // label too long
                    return Err(anyhow::Error::msg(format!(
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
        println!("to fdqn bytes: {:?}", self.0);
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

impl QType {
    pub fn to_be_bytes(self) -> [u8; 2] {
        (self as u16).to_be_bytes()
    }
    pub fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self::try_from(bytes).expect("Valid byte sequence for QType")
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
        let bytes = value.get(0..2).ok_or(anyhow::Error::msg(
            "QType TryFrom<Vec<u8>> Failed to get 2 bytes for u16 converson",
        ))?;
        let n = u16::from_be_bytes(
            bytes
                .try_into()
                .context("failed to convert byte slice into u16 value for QType".to_string())?,
        );
        Self::try_from(n)
    }
}

impl std::convert::TryFrom<[u8; 2]> for QType {
    type Error = LibDNSError;
    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        let n = u16::from_be_bytes(value);
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
            _ => Err(anyhow::Error::msg(format!(
                "The u16 value {value} is not valid for type QType"
            ))),
        }
    }
}
pub const DEFAULT_IO_BUFFER_BYTE_COUNT: usize = 1024;

// ===== Constants =====
pub const OPCODE_A: u16 = 0;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DNSClass {
    IN = 1, // Internet
    CS = 2, // CSNET (obsolete)
    CH = 3, // CHAOS
    HS = 4, // Hesiod

    NONE = 254, // Used in DNS UPDATE
    ANY = 255,  // Wildcard match

    Unknown(u16), // Fallback for undefined values
}

#[derive(Debug)]
pub enum DNSRecordKind {
    // --- Standard Types ---
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    NS(String),
    MX {
        preference: u16,
        exchange: String,
    },
    TXT(String),
    PTR(String),
    SOA {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },

    // --- Extended Types ---
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: String,
    },
    TLSA {
        usage: u8,
        selector: u8,
        matching_type: u8,
        certificate_data: Vec<u8>,
    },
    DS {
        key_tag: u16,
        algorithm: u8,
        digest_type: u8,
        digest: Vec<u8>,
    },
    RRSIG {
        type_covered: u16,
        algorithm: u8,
        labels: u8,
        original_ttl: u32,
        signature_expiration: u32,
        signature_inception: u32,
        key_tag: u16,
        signer_name: String,
        signature: Vec<u8>,
    },
    DNSKEY {
        flags: u16,
        protocol: u8,
        algorithm: u8,
        public_key: Vec<u8>,
    },
    SVCB {
        priority: u16,
        target: String,
        params: Vec<SvcParam>,
    },
    HTTPS {
        priority: u16,
        target: String,
        params: Vec<SvcParam>,
    },

    // --- Catch-all ---
    Unknown {
        qtype: u16,
        data: Vec<u8>,
    },
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DnsRType {
    // Standard types
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,

    // DNSSEC types
    RRSIG = 46,
    DNSKEY = 48,
    DS = 43,

    // DANE
    TLSA = 52,

    // Extended types
    SVCB = 64,
    HTTPS = 65,

    // Reserved / Meta
    OPT = 41,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ANY = 255,

    // Unknown (fallback)
    Unknown(u16),
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

#[derive(Debug)]
pub enum SvcParam {
    Alpn(Vec<String>),
    NoDefaultAlpn,
    Port(u16),
    Ipv4Hint(Vec<Ipv4Addr>),
    Ipv6Hint(Vec<Ipv6Addr>),
    Ech(Vec<u8>),
    DohPath(String),
    Opaque(u16, Vec<u8>), // Key-value pair for unrecognized keys
}

pub fn dns_resolve_hostname(
    dns_address: &str,
    question_types: Vec<QType>,
    name: &str,
) -> LibDNSResult<DNSResponse> {
    if !hostname_is_valid(name) {
        return Err(anyhow::Error::msg(format!(
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
        return Err(anyhow::Error::msg(format!(
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
            cursor.set_position(position - 1);
            break;
        }
        let question = question.unwrap();
        questions.push(question);
        // dbg!(a,b);
    }
    // println!("{:?}",slice);
    // parse the answers
    // println!("slice ancount: {slice:X}");
    let mut records: Vec<DNSRecord> = vec![];
    for _ in 0..header.ancount {
        println!("parse answer");
        // let slice = io_buffer.get(byte_index..).unwrap();
        let name: QName = QName::try_from_cursor_be(&mut cursor)
            .context("failed to parse qname for answer".to_string())
            .unwrap();

        let rtype: u16 = read_u16_from_cursor_as_be(&mut cursor)?;

        let class: u16 = read_u16_from_cursor_as_be(&mut cursor)?;

        let ttl: u32 = read_u32_from_cursor_as_be(&mut cursor)?;
        // parse and extract the RDATA section
        // first the length of the rdata as a u16
        let rd_length = read_u16_from_cursor_as_be(&mut cursor)?;

        // the rdata is of variable size depending
        let rdata = read_vector_of_bytes(rd_length as usize, &mut cursor)?;

        let record = DNSRecord::parse_from_raw_parts(name, rtype, class, ttl, rdata);

        records.push(record);
    }
    // dbg!(header);

    Ok(DNSResponse {
        header,
        questions,
        records,
    })
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

pub fn read_byte_array<const BYTECOUNT: usize, T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<[u8; BYTECOUNT]> {
    let mut buf: [u8; BYTECOUNT] = [0_u8; BYTECOUNT];
    cursor
        .read_exact(&mut buf)
        .context("Failed to read {BYTECOUNT} bytes into an array".to_string())?;
    Ok(buf)
}

pub fn read_u128_from_cursor_as_be<T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<u128> {
    const SIZE: usize = std::mem::size_of::<u128>() / std::mem::size_of::<u8>();
    let buf: [u8; SIZE] = read_byte_array(cursor)
        .context("Failed to get byte array from cursor for u128 be".to_string())?;
    Ok(u128::from_be_bytes(buf))
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

pub fn read_u32_from_cursor_as_be<T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<u32> {
    const SIZE: usize = std::mem::size_of::<u32>() / std::mem::size_of::<u8>();
    let buf: [u8; SIZE] = read_byte_array(cursor)
        .context("Failed to get byte array from cursor for u32 be".to_string())?;
    Ok(u32::from_be_bytes(buf))
}
pub fn read_u8_from_cursor<T: AsRef<[u8]>>(
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<u8> {
    let buf: [u8; 1] = read_byte_array(cursor)?;
    Ok(buf[0])
}

pub fn read_vector_of_bytes<T: AsRef<[u8]>>(
    size: usize,
    cursor: &mut std::io::Cursor<T>,
) -> self::LibDNSResult<Vec<u8>> {
    let mut buf = vec![0_u8; size];
    let err = format!("Failed to read {size} bytes from the cursor into the buffer");
    cursor.read_exact(&mut buf).context(err)?;
    Ok(buf)
}

#[test]
pub fn test_resolve_hostname() {
    let response =
        dns_resolve_hostname("1.1.1.1:53", vec![QType::A, QType::AAAA], "www.github.com.")
            .context("test failed".to_string())
            .unwrap();
    dbg!(response);
    // println!("{bytes:?}");
    // println!("{bytes2:?}");
}
pub mod protocol;

pub struct DNSHeaderFlags(u16);

#[repr(transparent)]
pub struct DNSHeaderID(pub u16);

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

#[derive(Debug)]
pub struct DNSQuestion {
    qname: QName,
    qtype: QType,
    qclass: u16,
}

#[derive(Debug)]
pub struct DNSRecord {
    pub domain: String,
    pub ttl: u32,
    pub class: DNSClass,
    pub kind: DNSRecordKind,
}
#[derive(Debug)]
pub struct DNSResponse {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub records: Vec<DNSRecord>,
}

// a wrapper over a DNS Query Name
#[derive(Debug)]
pub struct QName(Vec<u8>);

pub trait TryFromCursor<T>
where
    T: AsRef<[u8]>,
{
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
pub use anyhow::Error as LibDNSError;
pub use anyhow::Result as LibDNSResult;
type IoBuffer<const SIZE: usize = DEFAULT_IO_BUFFER_BYTE_COUNT> = [u8; SIZE];
use anyhow::Context;
use std::io::{BufRead, Bytes, Cursor, Read, Write};
use std::net::{Ipv4Addr, Ipv6Addr, UdpSocket};
