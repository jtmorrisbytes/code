use std::io::{Read, Write};
use std::net::UdpSocket;

// bit order for flags left -> right
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
    pub fn get_qr(&self) -> u16 {
        self.flags & QR_BITFLAG >> 15
    }
    pub fn get_opcode(&self) -> u16 {
        self.flags & OPCODE_BITFLAG >> 11
    }
    pub fn get_aa(&self) -> u16 {
        self.flags & AA_BITFLAG >> 10
    }
    pub fn get_tc(&self) -> u16 {
        self.flags & TC_BITFLAG >> 9
    }
    pub fn get_rd(&self) -> u16 {
        self.flags & RD_BITFLAG >> 8
    }
    pub fn get_ra(&self) -> u16 {
        self.flags & RA_BITFLAG >> 7
    }
    pub fn get_z(&self) -> u16 {
        self.flags & Z_BITFLAG >> 6
    }
    pub fn get_ad(&self) -> u16 {
        self.flags & AD_BITFLAG >> 5
    }
    pub fn get_cd(&self) -> u16 {
        self.flags & CD_BITFLAG >> 4
    }
    pub fn get_rcode(&self) -> u16 {
        self.flags & RCODE_BITFLAG
    }

    pub fn set_query_response(&mut self, n: u16) {
        self.flags |= (n & 0b00000000000000001) << 15;
    }
    pub fn set_opcode(&mut self, n: u16) {
        self.flags |= (n & 0b000000000000001111) << 14
    }
    pub fn set_recursion_desired(&mut self,n:u16) {
        self.flags |= n & BOOLEAN_BITFLAG << 8
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        if !bytes.len() < 12 {
            panic!("invalid length");
        }
        let mut id = [0_u8; 2];
        id[0] = bytes[0];
        id[1] = bytes[1];
        let id = std::ffi::c_ushort::from_be_bytes(id);
        let mut header: DNSHeader = unsafe { std::mem::zeroed() };
        header.id = id;

        // flags
        let mut flags = [0_u8, 0_u8];
        flags[0] = bytes[2];
        flags[1] = bytes[3];
        let flags = u16::from_be_bytes(flags);
        header.flags = flags;

        let mut num_questions: [u8; 2] = [0, 0];
        num_questions[0] = bytes[4];
        num_questions[1] = bytes[5];
        let qdcount = u16::from_be_bytes(num_questions);
        header.qdcount = qdcount;

        let mut ancount: [u8; 2] = [0, 0];
        ancount[0] = bytes[6];
        ancount[1] = bytes[7];
        let ancount = u16::from_be_bytes(ancount);

        header.ancount = ancount;

        header
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

pub const OPCODE_A: u16 = 0;

// #[derive(Default,Debug)]
// pub struct DnsHeader {
//     pub id: std::ffi::c_ushort, // Identification number
//     pub rd: u8, // Recursion Desired: std::ffi::c_char,
//     pub tc: u8, // Truncation: std::ffi::c_char,
//     pub aa: std::ffi::c_uchar, // Authoritative Answer
//     pub opcode: std::ffi::c_uchar, // Query type (0 for standard query)
//     pub qr: std::ffi::c_uchar, // Query/Response (0 for query)
//     pub rcode: std::ffi::c_uchar, // Response code
//     pub cd: std::ffi::c_uchar, // Checking Disabled
//     pub ad: std::ffi::c_uchar, // Authenticated Data
//     pub z: std::ffi::c_uchar, // Reserved for future use
//     pub ra: std::ffi::c_uchar, // Recursion Available
//     pub qdcount: std::ffi::c_ushort, // Number of questions
//     pub ancount: std::ffi::c_ushort, // Number of answer RRs
//     pub nscount: std::ffi::c_ushort, // Number of authority RRs
//     pub arcount: std::ffi::c_ushort, // Number of additional RRs
// }
#[derive(Default)]
pub struct DNSQuestion {
    qname: String,
    qtype: u16,
    qclass: u16,
}
impl DNSQuestion {
    pub fn to_be_bytes(self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        let mut qname = qname_to_bytes(&self.qname);
        buf.append(&mut qname);
        buf.extend_from_slice(&self.qtype.to_be_bytes());
        buf.extend_from_slice(&self.qclass.to_be_bytes());
        buf
    }
}

/*
 limitations:
 the hostname is limited to the ASCII character set
 a-z, A-Z, 0-9, '.' and '-'.


 each segment 'example' 'com' in example.com
 no more than 63 characters.

 the entire string including all periods:
 no more than 253 characters

*/

pub fn hostname_is_valid(name: &str) -> bool {
    if name.len() > 253 {
        return false;
    }
    if name.starts_with('-') && name.ends_with('-') {
        return false;
    }
    for segment in name.split(".").into_iter() {
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
    return true;
}

pub fn qname_to_bytes(qname: &str) -> Vec<u8> {
    // label: \x(count)
    // label for each segment
    let mut output = Vec::<u8>::new();
    for segment in qname.split(".") {
        output.push(segment.bytes().len() as u8);
        output.extend_from_slice(segment.as_bytes());
    }
    output.push(0);

    return output;
}

#[test]
pub fn libdns_test_qname_to_bytes() {
    let bytes = qname_to_bytes("github.com");
    assert_eq!(bytes,[0x06, 0x67,0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f, 0x6d, 0x0])
}



pub fn dns_resolve_hostname(dns_address:&str,name: &str) -> Vec<u8> {
    if !hostname_is_valid(name) {
        panic!("invalid hostname {name}");
    }

    // F* it we use cloudflare
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .unwrap();
    socket.connect(dns_address).unwrap();

    let mut header = DNSHeader {
        id: 0xdead,
        flags: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };
    header.set_query_response(0);
    header.set_opcode(0);
    header.set_recursion_desired(1);

    let question = DNSQuestion {
        qtype: 1,
        qclass: 1,
        qname: name.to_string(),
    };

    let mut header = header.to_be_bytes();
    println!("{header:x?}");
    let mut output = vec![];
    output.append(&mut header);
    output.append(&mut question.to_be_bytes());
    

    for byte in &output {
        print!("{byte:08b}");
    }
    println!("");
    socket.send(&output).unwrap();

    // let header = [0_u8;12];
    let mut io_buffer: Vec<u8> = vec![0_u8; 512];
    socket.recv(&mut io_buffer).unwrap();
    io_buffer

    
    // println!("{rest:?}");
}
#[test]
pub fn test_resolve_hostname() {
    let bytes = dns_resolve_hostname("192.168.1.254:53","www.github.com");
    println!("{bytes:?}");
    let bytes2 = dns_resolve_hostname("1.1.1.1:53", "www.github.com");
    println!("{bytes2:?}");

}
