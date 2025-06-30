// https://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html#request_format
// request format

// HEADER
// OPCODE 8bits, LENGTH (16 bits), DATA BYTE (assuming 8 bits)
// expressed in units of 8 bytes (8bit integers?)
// (Read 4 bytes (8 bits), then decode using the above data format)

// length defines the total length of the request including the above header

use std::{
    fmt::format,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpStream, ToSocketAddrs},
    str::FromStr,
};

use crate::sys::X_PROTOCOL_VERSION;

pub struct Request {
    header_opcode: u8,
    header_length: u16,
    data_byte: u8,
    // 0 or more additional data expressed as groups of 4 bytes
    payload: Vec<u32>,
}
pub struct Reply {
    // least signifcant 16 bits of the request id / sequence number
    sequence_number: u16,
    // like this? 32 bits of data expressed as units of 4 bytes
    length: [u8; 4],
    // 32 bytes
    data: [u8; 32],
}

pub struct Error {
    error_code: u8,
    // these fields are unknown
    major_opcode: (),
    minor_opcode: (),
    // the least significant 16 bits of the request sequence number
    sequence_number: u16,
}

pub enum Errors {
    Access,
    Alloc,
    Atom,
    Colormap,
    Cursor,
    Drawable,
    Font,
    GContext,
    IDChoice,
    Implementation,
    Length,
    Match,
    Name,
    Pixmap,
    Request,
    Value,
}

// The syntax {...} encloses a set of alternatives.
// The syntax [...] encloses a set of structure components
// In general, TYPEs are in uppercase and AlternativeValues are capitalized.
// If no ▶ is present in the description, then the request has no reply (it is asynchronous), although errors may still be reported.
// If ▶+ is used, then one or more replies can be generated for a single request.
// // used to hold 32 bit 'values'
// #[derive(Clone,Copy)]
// pub struct Bitfield32 {
//     bit_1: bool,
//     bit_2: bool,
//     bit_3: bool,
//     bit_4: bool,
//     bit_5: bool,
//     bit_6: bool,
//     bit_7: bool,
//     bit_8: bool,
//     bit_9: bool,
//     bit_10: bool,
//     bit_11: bool,
//     bit_12: bool,
//     bit_13: bool,
//     bit_14: bool,
//     bit_15: bool,
//     bit_16: bool,
//     bit_17: bool,
//     bit_18: bool,
//     bit_19: bool,
//     bit_20: bool,
//     bit_21: bool,
//     bit_22: bool,
//     bit_23: bool,
//     bit_24: bool,
//     bit_25: bool,
//     bit_26: bool,
//     bit_27: bool,
//     bit_28: bool,
//     bit_29: bool,
//     bit_30: bool,
//     bit_31: bool,
//     bit_32: bool
// }

// // used to hold generic 8 bit 'values'
// pub struct Bitfield8 {
//     bit_1: bool,
//     bit_2: bool,
//     bit_3: bool,
//     bit_4: bool,
//     bit_5: bool,
//     bit_6: bool,
//     bit_7: bool,
//     bit_8: bool,

// }

/// a 32 bit value top 3 bits guarenteed to be zero

#[derive(Clone, Copy, Debug)]
pub struct WINDOW(u32);

impl WINDOW {
    pub fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
    pub fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    pub fn try_from_iter_bytes_be(
        iterator: impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer: Vec<u8> = iterator.take(4).collect();
        let bytes: [u8; 4] = buffer.try_into().map_err(|old: Vec<u8>| {
            format!(
                "Could not create a {} from an iterator. expected 4 bytes, got {} bytes",
                std::any::type_name::<Self>(),
                old.len()
            )
        })?;
        Ok(Self::from_be_bytes(bytes))
    }
}

// top 3 bits guarenteed to be zero

/// a 32 bit value top 3 bits guarenteed to be zero
// top 3 bits guarenteed to be zero
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PIXMAP(u32);
impl PIXMAP {
    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}
#[repr(transparent)]
/// a 32 bit value top 3 bits guarenteed to be zero
pub struct CURSOR(u32);
impl CURSOR {
    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

/// a 32 bit value top 3 bits guarenteed to be zero

/// a 32 bit value top 3 bits guarenteed to be zero
#[derive(Clone, Copy)]
pub struct FONT(u32);

/// a 32 bit value top 3 bits guarenteed to be zero
#[derive(Clone, Copy)]
pub struct GContext(u32);

/// a 32 bit value top 3 bits guarenteed to be zero
#[derive(Debug)]
pub struct COLORMAP(u32);
// pub type COLORMAP = u32;
impl COLORMAP {
    pub fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
    pub fn try_from_iter_bytes_be(
        iterator: impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer: Vec<u8> = iterator.take(4).collect();
        let bytes: [u8; 4] = buffer
            .try_into()
            .map_err(|old: Vec<u8>| format!("COLORMAP"))?;
        Ok(Self::from_be_bytes(bytes))
    }
    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

/// a type that represents eithre a window OR a pixmap.
/// the spec says to use a union, but i want to use an enum

pub union Drawable {
    window: WINDOW,
    pixmap: PIXMAP,
}

// pub trait Drawable {}

// impl Drawable for Pixmap {}

// impl Drawable for Window {}

//  a marker trait that determines whether or not a type is 'Fontable'
pub union FONTABLE {
    font: FONT,
    gcontext: GContext,
}

/// a 32 bit value top 3 bits guarenteed to be zero
pub type ATOM = u32;

#[repr(transparent)]
pub struct Atom(ATOM);

pub type VISUALID = VisualID;
#[derive(Debug)]
pub struct VisualID(u32);

impl VisualID {
    pub fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
    pub fn try_from_iter_bytes_be(
        iter: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer: Vec<u8> = iter.take(4).collect();
        let bytes: [u8; 4] = buffer.try_into().map_err(|old: Vec<u8>| {
            format!(
                "Could not build a {} from an iterator of u8. 4 bytes needed but got {}",
                std::any::type_name::<Self>(),
                old.len()
            )
        })?;
        Ok(Self::from_be_bytes(bytes))
    }
    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

pub type BYTE = u8;

pub type INT8 = i8;
pub type INT16 = i16;
pub type INT32 = i32;
pub type CARD8 = Card8;
#[derive(Debug, Clone, Copy)]
pub struct Card8(u8);

impl std::ops::Mul for Card8 {
    type Output = Card8;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl std::ops::Mul<u8> for Card8 {
    type Output = Card8;
    fn mul(self, rhs: u8) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl std::cmp::PartialEq<u8> for Card8 {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}
impl std::cmp::PartialEq for Card8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::PartialEq<usize> for Card8 {
    fn eq(&self, other: &usize) -> bool {
        self.0 as usize == *other
    }
}

impl std::ops::Deref for Card8 {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::From<u8> for Card8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl std::convert::Into<usize> for Card8 {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Card8 {
    pub fn from_be_bytes(bytes: [u8; 1]) -> Self {
        Self(u8::from_be_bytes(bytes))
    }
    pub fn from_le_bytes(bytes: [u8; 1]) -> Self {
        Self(u8::from_le_bytes(bytes))
    }
    /// takes a mutable iterator as its first argument, advances the iterator by 4 bytes, trys to convert them into an array, then converts them into a CARD32
    pub fn try_from_iter_bytes_be(
        iter: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vector: Vec<u8> = iter.take(1).collect();
        let bytes: [u8; 1] = vector.try_into().map_err(|old: Vec<u8>| {
            format!(
                "Could not convert Vec<u8> of length {} into array [u8;1] for Card8",
                old.len()
            )
        })?;
        Ok(Self::from_be_bytes(bytes))
    }
    /// takes a mutable iterator as its first argument, advances the iterator by 4 bytes, converts them into an array, panicing if it fails, then converts them into a CARD32

    pub fn try_from_be_reader(stream: &mut impl Read) -> Result<Self, String> {
        let mut buf = [0_u8; 1];
        let read_result = stream.read(&mut buf);
        match read_result {
            Ok(1) => {
                /*all bytes read */
                Ok(Self::from_be_bytes(buf))
            }
            Ok(n) => {
                /* less than 4 bytes read */
                Err(format!("Error: {}::try_from_be_reader: The underlying Reader wrote {n} bytes. Cannot create a u8 from less than 1 byte",std::any::type_name::<Self>()))
            }
            Err(e) => Err(format!(
                "Error: {}::try_from_be_reader: The underlying Reader returned an error: {e}",
                std::any::type_name::<Self>()
            )),
        }
    }
    #[allow(dead_code)]
    pub fn from_iter_bytes_be(iter: &mut impl Iterator<Item = u8>) -> Self {
        let vector: Vec<u8> = iter.take(1).collect();
        let bytes: [u8; 1] = vector.try_into().expect(&format!(
            "valid byte (u8) from iterator for {}::from_iter_bytes_be",
            std::any::type_name::<Self>()
        ));
        Self::from_be_bytes(bytes)
    }
    pub fn to_be_bytes(&self) -> [u8; 1] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(&self) -> [u8; 1] {
        self.0.to_le_bytes()
    }
}

pub type CARD16 = Card16;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Card16(u16);
impl std::convert::Into<usize> for Card16 {
    fn into(self) -> usize {
        self.0 as usize
    }
}
impl std::ops::Add<usize> for CARD16 {
    type Output = Card16;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + TryInto::<u16>::try_into(rhs).unwrap_or(u16::MAX))
    }
}

impl Card16 {
    pub fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_be_bytes(bytes))
    }
    pub fn from_le_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
    /// takes a mutable iterator as its first argument, advances the iterator by 4 bytes, trys to convert them into an array, then converts them into a CARD32
    pub fn try_from_iter_bytes_be(
        iter: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vector: Vec<u8> = iter.take(2).collect();
        let bytes: [u8; 2] = vector.try_into().map_err(|e: Vec<u8>| {
            format!(
                "Could not convert Vec<u8> of length {} into array [u8;2] for Card16",
                e.len()
            )
        })?;
        Ok(Self::from_be_bytes(bytes))
    }
    /// takes a mutable iterator as its first argument, advances the iterator by 4 bytes, converts them into an array, panicing if it fails, then converts them into a CARD32

    pub fn try_from_be_reader(stream: &mut impl Read) -> Result<Self, String> {
        let mut buf = [0_u8; 2];
        let read_result = stream.read(&mut buf);
        match read_result {
            Ok(2) => {
                /*all bytes read */
                Ok(Self::from_be_bytes(buf))
            }
            Ok(n) => {
                /* less than 4 bytes read */
                Err(format!("Error: Card32::try_from_be_reader: The underlying Reader wrote {n} bytes. Cannot create a u16 from less than 2 bytes"))
            }
            Err(e) => Err(format!(
                "Error: Card32::try_from_be_reader: The underlying Reader returned an error: {e}"
            )),
        }
    }
    #[allow(dead_code)]
    pub fn from_iter_bytes_be(iter: &mut impl Iterator<Item = u8>) -> Self {
        let vector: Vec<u8> = iter.take(2).collect();
        let bytes: [u8; 2] = vector
            .try_into()
            .expect("valid sequence of 4 bytes (u8) from iterator for Card32::from_iter_bytes_be");
        Self::from_be_bytes(bytes)
    }
    pub fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

pub type CARD32 = Card32;
#[derive(Debug)]
pub struct Card32(u32);
impl Card32 {
    pub fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
    /// takes a mutable iterator as its first argument, advances the iterator by 4 bytes, trys to convert them into an array, then converts them into a CARD32
    pub fn try_from_iter_bytes_be(
        iter: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vector: Vec<u8> = iter.take(4).collect();
        let bytes: [u8; 4] = vector.try_into().map_err(|old: Vec<u8>| {
            format!(
                "Could not convert Vec<u8> of length {} to [u8;4] for Card32",
                old.len()
            )
        })?;
        Ok(Self::from_be_bytes(bytes))
    }
    /// takes a mutable iterator as its first argument, advances the iterator by 4 bytes, converts them into an array, panicing if it fails, then converts them into a CARD32

    pub fn try_from_be_reader(stream: &mut impl Read) -> Result<Self, String> {
        let mut buf = [0_u8; 4];
        let read_result = stream.read(&mut buf);
        match read_result {
            Ok(4) => {
                /*all bytes read */
                Ok(Self::from_be_bytes(buf))
            }
            Ok(n) => {
                /* less than 4 bytes read */
                Err(format!("Error: Card32::try_from_be_reader: The underlying Reader wrote {n} bytes. Cannot create a u32 from less than 4 bytes"))
            }
            Err(e) => Err(format!(
                "Error: Card32::try_from_be_reader: The underlying Reader returned an error: {e}"
            )),
        }
    }
    #[allow(dead_code)]
    pub fn from_iter_bytes_be(iter: &mut impl Iterator<Item = u8>) -> Self {
        let vector: Vec<u8> = iter.take(4).collect();
        let bytes: [u8; 4] = vector
            .try_into()
            .expect("valid sequence of 4 bytes (u8) from iterator for Card32::from_iter_bytes_be");
        Self::from_be_bytes(bytes)
    }
    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    pub fn to_le_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

pub type TIMESTAMP = CARD32;
#[repr(u8)]
pub enum BITGRAVITY {
    Forget = 0,
    NorthWest = 1,
    North = 2,
    NorthEast = 3,
    West = 4,
    Center = 5,
    East = 6,
    SouthWest = 7,
    South = 8,
    SouthEast = 9,
    Static = 10,
}
impl std::convert::Into<u8> for BITGRAVITY {
    fn into(self) -> u8 {
        self as u8
    }
}
impl BITGRAVITY {
    pub fn to_be_bytes(self) -> [u8; 1] {
        [self as u8]
    }
    pub fn to_le_bytes(self) -> [u8; 1] {
        [self as u8]
    }
}
#[repr(u8)]
pub enum WINGRAVITY {
    Unmap = 0,
    NorthWest = 1,
    North = 2,
    NorthEast = 3,
    West = 4,
    Center = 5,
    East = 6,
    SouthWest = 7,
    South = 8,
    SouthEast = 9,
    Static = 10,
}

impl WINGRAVITY {
    pub fn to_be_bytes(self) -> [u8; 1] {
        [self as u8]
    }
    pub fn to_le_bytes(self) -> [u8; 1] {
        [self as u8]
    }
}

pub type BOOL = Bool;

#[derive(Debug)]
#[repr(u8)]
pub enum Bool {
    False = 0,
    True = 1,
}
impl std::convert::From<bool> for Bool {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}
impl std::convert::TryFrom<u8> for Bool {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::False),
            1 => Ok(Self::True),
            _ => Err(format!(
                "Invalid value {} for type {}. Valid values are 0 or 1",
                value,
                std::any::type_name::<Self>()
            )
            .into()),
        }
    }
}
impl std::convert::TryFrom<Card8> for Bool {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Card8) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
impl TryFromIterBytesBe for Bool {
    fn try_from_iter_bytes_be(
        iterator: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let byte = iterator
            .next()
            .ok_or("Could not get byte from iterator for type Bool")?;
        Self::try_from(byte)
    }
}
impl Bool {
    pub fn to_be_bytes(self) -> [u8; 1] {
        (self as u8).to_be_bytes()
    }
    pub fn to_le_bytes(self) -> [u8; 1] {
        (self as u8).to_le_bytes()
    }
}
// this is valid because a BOOL is always either a 0 or a 1, but a u8 can be any of those
impl std::convert::Into<Card8> for Bool {
    fn into(self) -> Card8 {
        Card8(self as u8)
    }
}

// an 8 bit value
pub struct Byte {
    bit_0: bool,
    bit_1: bool,
    bit_2: bool,
    bit_3: bool,
    bit_4: bool,
    bit_5: bool,
    bit_6: bool,
    bit_7: bool,
}
#[derive(Debug)]
pub enum Event {
    KeyPress,
    KeyRelease,
    OwnerGrabButton,
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    Exposure,
    VisibilityChange,
    StructureNotify,
    ResizeRedirect,
    SubstructureNotify,
    SubstructureRedirect,
    FocusChange,
    PropertyChange,
    ColormapChange,
    KeymapState,
}

pub enum PointerEvent {
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    KeymapState,
}

pub enum DeviceEvent {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelase,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
}

#[repr(transparent)]
pub struct KeySym(u32);

pub type KEYCODE = KeyCode;

#[derive(Debug)]
pub struct KeyCode(Card8);
impl KeyCode {
    pub fn from_be_bytes(bytes: [u8; 1]) -> Self {
        Self(Card8::from_be_bytes(bytes))
    }
    // pub fn try_from_iter_bytes_be(iterator: &mut impl Iterator<Item=u8>) -> Result<Self,Box<dyn std::error::Error>> {
    //     Card8::try_from_iter_bytes_be(iterator).map(|card8|Self(card8))
    // }
}
impl TryFromIterBytesBe for KeyCode {
    fn try_from_iter_bytes_be(
        iterator: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Card8::try_from_iter_bytes_be(iterator).map(|card8| Self(card8))
    }
}

pub type BUTTON = CARD8;
pub struct Button(BUTTON);

pub enum KeyMask {
    Shift,
    Lock,
    Control,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}
pub enum ButMask {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
}

pub struct String8(String);
impl std::fmt::Debug for String8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl String8 {
    pub fn try_from_iter_bytes_be(
        iterator: &mut impl Iterator<Item = u8>,
        length: Card16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer: Vec<u8> = iterator.take(length.into()).collect();
        let string = String::from_utf8(buffer)?;

        Ok(Self(string))
    }
    pub fn as_str(&self) -> &'_ str {
        self.0.as_str()
    }
}
impl ToString for String8 {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub struct CHAR2B {
    byte1: CARD8,
    byte2: CARD8,
}

pub struct POINT {
    x: INT16,
    y: INT16,
    width: CARD16,
    height: CARD16,
}

pub struct RECTANGLE {
    /// the X coordinate starting from the upper left corner
    x: INT16,
    /// the Y coordinate starting from the upper right corner
    y: INT16,
    width: CARD16,
    height: CARD16,
}

pub struct ARC {
    x: INT16,
    y: INT16,
    width: CARD16,
    height: CARD16,
}

pub enum HostFamily {
    Internet,
    InternetV6,
    ServerInterprete,
    #[allow(non_camel_case_types)]
    DECNet,
    Chaos,
}

pub struct Host {
    family: HostFamily,
    address: Vec<BYTE>,
}

#[repr(u16)]
pub enum CreateWindowClass {
    InputOutput = 0,
    InputOnly = 1,
    CopyFromParent = 2,
}
impl CreateWindowClass {
    pub fn to_be_bytes(self) -> [u8;2] {
        (self as u16).to_be_bytes()
    }
}

pub enum CreateWindowVisual {
    VisualID(VISUALID),
    CopyFromParent,
}

const X_PROTO_MAJOR_VERSION: u16 = 11;
const X_PROTO_MINOR_VERSION: u16 = 0;

#[derive(Debug)]
pub struct PIXFORMAT {
    depth: CARD8,
    bits_per_pixel: CARD8,
    scanline_pad: CARD8,
}
impl PIXFORMAT {
    fn new(depth: u8, bits_per_pixel: u8, scanline_pad: u8) -> Self {
        Self {
            depth: Card8(depth),
            bits_per_pixel: Card8(bits_per_pixel),
            scanline_pad: Card8(scanline_pad),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ScreenBackingStores {
    Never = 0,
    WhenMapped = 1,
    Always = 2,
}
impl std::convert::TryFrom<u32> for ScreenBackingStores {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Never),
            1 => Ok(Self::WhenMapped),
            2 => Ok(Self::Always),
            _ => Err(format!(
                "Could not convert u32 to {}. Invalid value {}.  Valid range is from 0 to 2",
                std::any::type_name::<Self>(),
                value
            )
            .into()),
        }
    }
}
impl std::convert::TryFrom<u16> for ScreenBackingStores {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Never),
            1 => Ok(Self::WhenMapped),
            2 => Ok(Self::Always),
            _ => Err(format!(
                "Could not convert u16 to {}. Invalid value {}.  Valid range is from 0 to 2",
                std::any::type_name::<Self>(),
                value
            )
            .into()),
        }
    }
}
impl std::convert::TryFrom<u8> for ScreenBackingStores {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Never),
            1 => Ok(Self::WhenMapped),
            2 => Ok(Self::Always),
            _ => Err(format!(
                "Could not convert u8 to {}. Invalid value {}.  Valid range is from 0 to 2",
                std::any::type_name::<Self>(),
                value
            )
            .into()),
        }
    }
}
impl std::convert::TryFrom<Card8> for ScreenBackingStores {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Card8) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
impl std::convert::TryFrom<Card16> for ScreenBackingStores {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Card16) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
impl std::convert::TryFrom<Card32> for ScreenBackingStores {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Card32) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
impl std::convert::TryFrom<VISUALID> for ScreenBackingStores {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: VISUALID) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
impl std::convert::Into<u32> for ScreenBackingStores {
    fn into(self) -> u32 {
        self as u32
    }
}

impl ScreenBackingStores {
    pub fn try_from_iter_bytes_be(
        iter: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer: Vec<u8> = iter.take(1).collect();
        let bytes: [u8; 1] = buffer.try_into().map_err(|old: Vec<u8>| {
            format!(
                "Could not convert Vec<u8> of length {}to array [u8;4] from iterator for type {}",
                old.len(),
                std::any::type_name::<Self>()
            )
        })?;
        // let integer: u38 = u32::from_be_bytes(bytes);
        Self::try_from(bytes[0])
    }
    pub fn to_be_bytes(self) -> [u8; 1] {
        (self as u8).to_be_bytes()
    }
    pub fn to_le_bytes(self) -> [u8; 1] {
        (self as u8).to_le_bytes()
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum BitmapFormatBitOrder {
    LeastSignificant = 0,
    MostSignificant = 1,
}
impl std::convert::TryFrom<Card8> for BitmapFormatBitOrder {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Card8) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::LeastSignificant),
            1 => Ok(Self::MostSignificant),
            _ => Err(format!(
                "Invalid value {} for {}. Valid Range is from 0 to 1",
                value.0,
                std::any::type_name::<Self>()
            )
            .into()),
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum ImageByteOrder {
    LSBFirst = 0,
    MSBFirst = 1,
}
impl std::convert::TryFrom<CARD8> for ImageByteOrder {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: CARD8) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::LSBFirst),
            1 => Ok(Self::MSBFirst),
            _ => Err(format!(
                "Invalid value {} for ImageByteOrder. Valid range is from 0 to 1",
                value.0
            )
            .into()),
        }
    }
}

#[derive(Debug)]
pub struct Screen {
    root: WINDOW,
    default_colormap: COLORMAP,
    white_pixel: CARD32,
    black_pixel: CARD32,
    current_input_masks: Card32,
    width_in_pixels: CARD16,
    height_in_pixels: CARD16,
    width_in_millimeters: CARD16,
    height_in_millimeters: CARD16,
    min_installed_maps: CARD16,
    max_installed_maps: CARD16,
    root_visual: VISUALID,
    backing_stores: ScreenBackingStores,
    save_unders: Bool,
    root_depth: CARD8,
    allowed_depths: Vec<Depth>,
}

#[derive(Debug)]
pub struct Depth {
    depth: CARD8,
    visuals: Vec<VisualType>,
}
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VisualTypeClass {
    StaticGrey = 0,
    GrayScale = 1,
    StaticColor = 2,
    PseudoColor = 3,
    TrueColor = 4,
    DirectColor = 5,
}
impl std::convert::TryFrom<u8> for VisualTypeClass {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::StaticGrey),
            1 => Ok(Self::GrayScale),
            2 => Ok(Self::StaticColor),
            3 => Ok(Self::PseudoColor),
            4 => Ok(Self::TrueColor),
            5 => Ok(Self::DirectColor),
            _ => Err(format!(
                "Invalid value for type {}. Valid range is from 0 to 5",
                std::any::type_name::<Self>()
            )
            .into()),
        }
    }
}
impl std::convert::TryFrom<Card8> for VisualTypeClass {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Card8) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
impl std::convert::Into<u8> for VisualTypeClass {
    fn into(self) -> u8 {
        self as u8
    }
}
impl std::convert::Into<Card8> for VisualTypeClass {
    fn into(self) -> Card8 {
        Card8(self as u8)
    }
}

pub type VISUALTYPE = VisualType;

#[derive(Debug)]
pub struct VisualType {
    visual_id: VISUALID,
    class: VisualTypeClass,
    bits_per_rgb_value: CARD8,
    colormap_entries: Card16,
    red_mask: CARD32,
    green_mask: CARD32,
    blue_mask: CARD32,
}
impl VisualType {
    pub fn new(
        visual_id: VISUALID,
        class: VisualTypeClass,
        bits_per_rgb_value: CARD8,
        colormap_entries: Card16,
        red_mask: Card32,
        green_mask: Card32,
        blue_mask: Card32,
    ) -> Self {
        Self {
            visual_id,
            class,
            bits_per_rgb_value,
            colormap_entries,
            red_mask,
            green_mask,
            blue_mask,
        }
    }
}

pub enum DisplayAddress {
    Ipv6HostAddr {
        address: Ipv6Addr,
        display: u32,
        screen: Option<u32>,
    },
    Ipv4HostAddr {
        address: Ipv4Addr,
        display: u32,
        screen: Option<u32>,
    },
    SocketAddrs {
        hostname: String,
        addresses: Vec<std::net::SocketAddr>,
        display: u32,
        screen: Option<u32>,
    },
    WithHostname {
        hostname: String,
        display: u32,
        screen: Option<u32>,
    },
    WithoutHostname {
        display: u32,
        screen: Option<u32>,
    },
}

pub fn parse_display_str(s: &str) -> DisplayAddress {
    // let bytes: Vec<u8> = s.bytes().collect();
    let mut split = s.split(":");
    let hostname = split.next().unwrap_or_default();
    let display_and_screen = split.next().unwrap_or_default();

    let mut split = display_and_screen.split(".");
    let display = split.next().unwrap_or(display_and_screen);
    let screen = split.next();
    let display: u32 = display.parse().unwrap();
    let screen = screen.map(|v| v.parse::<u32>().unwrap());

    if hostname.len() == 0 {
        // no hostname provided
        return DisplayAddress::WithoutHostname { display, screen };
    }
    if hostname.starts_with('[') {
        let position = hostname
            .chars()
            .position(|c| c == ']')
            .ok_or("Expecting IPV6 address enclosed in [] brackets. missing closing bracket")
            .unwrap();
        let address_str = hostname
            .get(1..position - 1)
            .ok_or(format!(
                "Expecting IPV6 address in closing brackets:[] but instead got '{hostname}'"
            ))
            .unwrap();
        let address: Ipv6Addr = Ipv6Addr::from_str(address_str).unwrap();
        return DisplayAddress::Ipv6HostAddr {
            address,
            display,
            screen,
        };
    } else if let Ok(address) = std::net::Ipv4Addr::from_str(hostname) {
        return DisplayAddress::Ipv4HostAddr {
            address,
            display,
            screen,
        };
    } else if let Ok(addresses) = hostname.to_socket_addrs() {
        let addresses: Vec<std::net::SocketAddr> = addresses.collect();
        return DisplayAddress::SocketAddrs {
            hostname: hostname.to_string(),
            addresses,
            display,
            screen,
        };
    } else {
        return DisplayAddress::WithHostname {
            hostname: hostname.to_string(),
            display,
            screen,
        };
    }
}
#[derive(PartialEq, Eq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

pub struct Display {
    stream: Box<dyn X11Stream>,
    connection_info: X11ConnectionInfo,
    sequence_number: usize,
    byte_order: ByteOrder,
}
impl Display {
    pub fn connect(display_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let display_address = parse_display_str(display_str);
        let mut stream: Box<dyn X11Stream> = match display_address {
            DisplayAddress::WithoutHostname { display, screen } => {
                // first attempt to connect to a unix socket specified by the display number
                // then attempt to connect to a localhost: 6000 + the display number
                let screen_number_str = screen.map(|v| format!(".{v}")).unwrap_or(String::new());
                let path = format!("/tmp/.X11-unix/X{display}{screen_number_str}");
                Box::new(connect_unix_socket(&path)?)
            }
            DisplayAddress::Ipv4HostAddr {
                address,
                display,
                screen: _,
            } => {
                // attempt to connect to the ipv4 address specified by the connection string.
                // return an error if the connection was refused
                let address = format!("{address}:{}", display + 6000);
                let stream = std::net::TcpStream::connect(address)?;
                Box::new(stream) as Box<dyn X11Stream>
            }
            DisplayAddress::Ipv6HostAddr {
                address,
                display,
                screen: _,
            } => {
                // attempt to connect to the ipv6 address specified by the connection string
                // return an error if the connection was refused
                let address = format!("{address}:{}", display + 6000);
                let stream = connect_tcp_ip(address)?;
                Box::new(stream) as Box<dyn X11Stream>
            }
            DisplayAddress::SocketAddrs {
                hostname,
                addresses,
                display,
                screen: _,
            } => {
                // the std library network stack was able to resolve the hostnames to socket addresses.
                // attempt to connect to each one of them until you get a connection or return an error
                let mut stream: Option<Box<dyn X11Stream>> = None;
                for socket_address in addresses {
                    let address = format!("{}:{}", socket_address.ip(), display + 6000);
                    if let Ok(tstream) = connect_tcp_ip(address) {
                        stream = Some(Box::new(tstream) as Box<dyn X11Stream>);
                        break;
                    }
                }
                if stream.is_none() {
                    return Err(format!(
                        "No connections were accepted for the provided hostname {hostname})"
                    )
                    .into());
                } else {
                    stream.unwrap()
                }
            }
            DisplayAddress::WithHostname {
                hostname,
                display,
                screen: _,
            } => {
                // the std library was not able to resolve the hostname to an ip address and none of the above scenarios apply
                // decide what to do here?
                unimplemented!()
            }
        };
        let connection_info = Self::perform_handhake(&mut stream)?;
        Ok(Self {
            stream,
            connection_info,
            sequence_number: 0,
            byte_order: ByteOrder::BigEndian,
        })
    }

    pub fn perform_handhake(
        stream: &mut Box<dyn X11Stream>,
    ) -> Result<X11ConnectionInfo, Box<dyn std::error::Error>> {
        // let s = std::net::TcpStream::connect("::1:6001")?;
        let mut command_buffer = Vec::<u8>::new();
        command_buffer.write(&[0o102_u8, 0_u8])?;
        command_buffer.write(&X_PROTO_MAJOR_VERSION.to_be_bytes())?;
        command_buffer.write(&X_PROTO_MINOR_VERSION.to_be_bytes())?;
        //auth_proto_n_bytes
        command_buffer.write(&0_u16.to_be_bytes())?;
        // auth_proto_string_n_bytes
        command_buffer.write(&0_u16.to_be_bytes())?;
        // additional padding needed?
        command_buffer.write(&0_u16.to_be_bytes())?;
        // the protocol string (nul byte)
        // command_buffer.write(&[0,0])?;
        // the protocol data (nul byte)
        // command_buffer.write(&[0])?;
        stream.write_all(&command_buffer)?;
        // dbg!(command_buffer.len() == written);

        stream.flush()?;
        println!("flushed");

        // connection response

        // attempt to read the length of the response as a 32 bit integer as a sequence of 4 bytes
        let mut buffer = [0_u8; 1];

        stream.read_exact(&mut buffer)?;
        let success = CARD8::from_be_bytes(buffer).0;
        match success {
            // failure
            0 => {
                let len_reason_bytes = {
                    let mut buf = [0_u8; 1];
                    stream.read_exact(&mut buf)?;
                    u8::from_be_bytes(buf)
                };
                let x_protocol_major_version = {
                    let mut buf = [0_u8; 2];
                    stream.read_exact(&mut buf)?;
                    CARD16::from_be_bytes(buf)
                };
                let x_protocol_minor_version = {
                    let mut buf = [0_u8; 2];
                    stream.read_exact(&mut buf)?;
                    CARD16::from_be_bytes(buf)
                };
                let len_in_additional_data = {
                    let mut buf = [0_u8; 2];
                    stream.read_exact(&mut buf)?;
                    CARD16::from_be_bytes(buf)
                };
                let mut buffer = Vec::<u8>::new();
                for _ in 0..len_reason_bytes {
                    buffer.push(0);
                }
                stream.read_exact(buffer.as_mut_slice())?;
                let err_msg = String::from_utf8(buffer).unwrap_or_else(|_| format!("X Returned an error during the handshake and an error message string could not be built from the response."));
                Err(err_msg.into())
            }
            // connection accepted
            1 => {
                // first byte after the success byte is unused
                stream.read(&mut [0_u8])?;
                let protocol_major_version = {
                    let mut buf = [0_u8; 2];
                    stream.read_exact(&mut buf)?;
                    CARD16::from_be_bytes(buf)
                };
                let protocol_minor_version = {
                    let mut buf = [0_u8; 2];
                    stream.read_exact(&mut buf)?;
                    CARD16::from_be_bytes(buf)
                };
                let length = {
                    let mut buf = [0_u8; 2];
                    stream.read_exact(&mut buf)?;
                    u16::from_be_bytes(buf)
                };

                let mut buf = vec![0_u8; length as usize * 8];
                stream.read(&mut buf)?;
                println!("{:?}", &buf);

                let mut iter = buf.into_iter();
                // let release_number = { CARD32::from_be_bytes(iter.take(4).collect::<Vec<u8>>().try_into().unwrap()) };
                let release_number = Card32::try_from_iter_bytes_be(&mut iter)?;
                let release_id_base = Card32::try_from_iter_bytes_be(&mut iter)?;
                let resource_id_mask = Card32::try_from_iter_bytes_be(&mut iter)?;
                let motion_buffer_size = Card32::try_from_iter_bytes_be(&mut iter)?;
                let vendor_len = Card16::try_from_iter_bytes_be(&mut iter)?;
                let max_request_len = Card16::try_from_iter_bytes_be(&mut iter)?;
                let num_of_screens_in_roots = Card8::try_from_iter_bytes_be(&mut iter)?;
                let number_for_formats_in_pixmap_formats =
                    Card8::try_from_iter_bytes_be(&mut iter)?;
                let image_byte_order: ImageByteOrder =
                    ImageByteOrder::try_from(Card8::try_from_iter_bytes_be(&mut iter)?)?;

                let bitmap_format_bit_order = Card8::try_from_iter_bytes_be(&mut iter)?;
                let bitmap_format_bit_order: BitmapFormatBitOrder =
                    BitmapFormatBitOrder::try_from(bitmap_format_bit_order)?;

                let bitmap_format_scanline_unit = Card8::try_from_iter_bytes_be(&mut iter)?;
                let bitmap_format_scanline_pad = Card8::try_from_iter_bytes_be(&mut iter)?;
                let min_keycode = KeyCode::try_from_iter_bytes_be(&mut iter)?;
                let max_keycode = KeyCode::try_from_iter_bytes_be(&mut iter)?;
                // skip 4 bytes
                for _ in 0..=3 {
                    let _ = iter.next();
                }
                let vendor_name = String8::try_from_iter_bytes_be(&mut iter, vendor_len)?;

                // for _ in 0..2 {
                //     let _ = iter.next();
                // }

                // there may be padding after the vendor string but I cant tell.
                // get the list of PIXFORMATS
                let byte_len = (number_for_formats_in_pixmap_formats * 8);

                let mut formats = Vec::<PIXFORMAT>::new();
                for _ in (0..byte_len.into()).step_by(8) {
                    let depth: u8 = iter
                        .next()
                        .ok_or(format!("Ran out of data before extracting Screen depth"))?;
                    let bits_per_pixel: u8 = iter.next().ok_or(format!(
                        "Ran out of data before extracting Screen bits_per_pixel"
                    ))?;
                    let scanline_pad: u8 = iter.next().ok_or(format!(
                        "Ran out of data before extracting Screen scanline_pad"
                    ))?;
                    formats.push(PIXFORMAT::new(depth, bits_per_pixel, scanline_pad));
                    // skip 5 bytes
                    for _ in 0..5 {
                        let r = iter.next();
                        if r.is_none() {
                            return Err(format!("Ran out of bytes before the next Screen").into());
                        }
                    }
                }
                println!("{formats:?}");

                // get the list of roots
                let mut roots: Vec<Screen> = vec![];

                for _ in 0..num_of_screens_in_roots.into() {
                    // assuming this is the window id
                    let root: WINDOW = WINDOW::try_from_iter_bytes_be(&mut iter)?;
                    let default_colormap: COLORMAP = COLORMAP::try_from_iter_bytes_be(&mut iter)?;
                    let white_pixel: Card32 = Card32::try_from_iter_bytes_be(&mut iter)?;
                    let black_pixel: Card32 = Card32::try_from_iter_bytes_be(&mut iter)?;
                    let current_input_masks: Card32 = Card32::try_from_iter_bytes_be(&mut iter)?;
                    let width_in_pixels: Card16 = Card16::try_from_iter_bytes_be(&mut iter)?;
                    let height_in_pixels: Card16 = Card16::try_from_iter_bytes_be(&mut iter)?;
                    let width_in_millimeters: Card16 = Card16::try_from_iter_bytes_be(&mut iter)?;
                    let height_in_millimeters: Card16 = Card16::try_from_iter_bytes_be(&mut iter)?;
                    let min_installed_maps: Card16 = Card16::try_from_iter_bytes_be(&mut iter)?;
                    let max_installed_maps: Card16 = Card16::try_from_iter_bytes_be(&mut iter)?;

                    let root_visual: VISUALID = VISUALID::try_from_iter_bytes_be(&mut iter)?;
                    let backing_stores: ScreenBackingStores =
                        ScreenBackingStores::try_from_iter_bytes_be(&mut iter)?;
                    let save_unders: Bool = Bool::try_from_iter_bytes_be(&mut iter)?;
                    let root_depth: Card8 = Card8::try_from_iter_bytes_be(&mut iter)?;
                    let num_depths: Card8 = Card8::try_from_iter_bytes_be(&mut iter)?;

                    let mut depths: Vec<Depth> = vec![];
                    for _ in 0..num_depths.into() {
                        let depth: Card8 = Card8::try_from_iter_bytes_be(&mut iter)?;
                        // skip one
                        if let None = iter.next() {
                            return Err(format!(
                                "Ran out of data while after processing depth for {root:?}"
                            )
                            .into());
                        }
                        let number_visualtypes: Card16 = Card16::try_from_iter_bytes_be(&mut iter)?;
                        // skip 4 bytes
                        for _ in 0..4 {
                            if let None = iter.next() {
                                return Err(format!("Ran out of data while skipping 4 bytes after processing number of visualtypes for {root:?}").into());
                            }
                        }
                        let mut visuals: Vec<VisualType> =
                            Vec::with_capacity(number_visualtypes.into());
                        for _ in 0..number_visualtypes.into() {
                            let visual_id: VISUALID = VISUALID::try_from_iter_bytes_be(&mut iter)?;
                            let visual_class: VisualTypeClass = VisualTypeClass::try_from(
                                Card8::try_from_iter_bytes_be(&mut iter)?,
                            )?;
                            let bits_per_rgb_value: Card8 =
                                Card8::try_from_iter_bytes_be(&mut iter)?;
                            let colormap_entries: Card16 =
                                Card16::try_from_iter_bytes_be(&mut iter)?;
                            let red_mask: Card32 = Card32::try_from_iter_bytes_be(&mut iter)?;
                            let green_mask: Card32 = Card32::try_from_iter_bytes_be(&mut iter)?;
                            let blue_mask: Card32 = Card32::try_from_iter_bytes_be(&mut iter)?;

                            let vt = VisualType::new(
                                visual_id,
                                visual_class,
                                bits_per_rgb_value,
                                colormap_entries,
                                red_mask,
                                green_mask,
                                blue_mask,
                            );
                            visuals.push(vt);
                            // skip 4 bytes
                            for _ in 0..4 {
                                if let None = iter.next() {
                                    return Err("Ran out of data while skipping 4 bytes after processing VISUALTYPE".into());
                                }
                            }
                        }
                        depths.push(Depth { depth, visuals })
                    }
                    roots.push(Screen {
                        root,
                        default_colormap,
                        white_pixel,
                        black_pixel,
                        current_input_masks,
                        width_in_pixels,
                        height_in_pixels,
                        width_in_millimeters,
                        height_in_millimeters,
                        min_installed_maps,
                        max_installed_maps,
                        root_visual,
                        backing_stores,
                        save_unders,
                        root_depth,
                        allowed_depths: depths,
                    });
                }
                let success = X11ConnectionInfo {
                    release_id_base,
                    release_number,
                    resource_id_mask,
                    motion_buffer_size,
                    max_request_len,
                    image_byte_order,
                    bitmap_format_bit_order,
                    bitmap_format_scanline_pad,
                    bitmap_format_scanline_unit,
                    min_keycode,
                    max_keycode,
                    formats,
                    protocol_major_version,
                    protocol_minor_version,
                    roots,
                    vendor_name,
                };

                Ok(success)
            }
            2 => {
                todo!("Authentication Required");
            }
            _ => {
                unimplemented!("Success response: {success}. Not implemented");
            }
        }
    }
    pub fn create_window(&mut self) -> Result<(),Box<dyn std::error::Error>> {
        // the opcode associtated with this request
        let mut command_buffer: Vec<u8> = Vec::new();
        const OPCODE: CARD8 = Card8(1);

        let depth: Card8 = Card8(0);

        // calculate the request length from the final buffer
        let window_id = WINDOW(0);
        let parent = WINDOW(0);
        let position_x: INT16 = 0;
        let position_y: INT16 = 0;
        let width: Card16 = Card16(1);
        let height: Card16 = Card16(1);
        let border_width = Card16(0);
        let class: CreateWindowClass = CreateWindowClass::CopyFromParent;
        let visual_id: VisualID = VisualID(0);
        let value_mask: u32 = 0;

        let mut request_len = Card16(0);

        struct Value {
            background_pixmap: PIXMAP,
            background_pixel: Card32,
            border_pixmap: PIXMAP,
            border_pixel: Card32,
            bit_gravity: BITGRAVITY,
            win_gravity: WINGRAVITY,
            backing_store: ScreenBackingStores,
            backing_planes: Card32,
            backing_pixel: Card32,
            override_redirect: BOOL,
            save_under: Bool,
            event_mask: u32,
            do_not_propogate_mask: u32,
            colormap: COLORMAP,
            cursor: CURSOR,
        }
        impl Value {
            fn to_be_bytes(self) -> [u8; 41] {
                let mut buffer: Vec<u8> = Vec::new();
                let b = self.background_pixmap.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.background_pixel.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.border_pixmap.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.border_pixel.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.bit_gravity.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.win_gravity.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.backing_store.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.backing_planes.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.backing_pixel.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.override_redirect.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.save_under.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b: [u8; 4] = self.event_mask.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b: [u8; 4] = self.do_not_propogate_mask.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.colormap.to_be_bytes();
                buffer.extend_from_slice(&b);
                let b = self.cursor.to_be_bytes();
                buffer.extend_from_slice(&b);
                buffer.try_into().unwrap()
            }
            fn to_le_bytes(self) -> [u8; 41] {
                let mut buffer: Vec<u8> = Vec::new();
                let b = self.background_pixmap.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.background_pixel.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.border_pixmap.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.bit_gravity.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.win_gravity.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.backing_store.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.backing_planes.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.backing_planes.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.override_redirect.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.save_under.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b: [u8; 4] = self.event_mask.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b: [u8; 4] = self.do_not_propogate_mask.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.colormap.to_le_bytes();
                buffer.extend_from_slice(&b);
                let b = self.cursor.to_le_bytes();
                buffer.extend_from_slice(&b);
                buffer.try_into().unwrap()
            }
        }
        let values: Vec<Value> = vec![];

        let values_bytes: Vec<u8> = values
            .into_iter()
            .map(|value| {
                if self.byte_order == ByteOrder::BigEndian {
                    value.to_be_bytes()
                } else {
                    value.to_be_bytes()
                }
            })
            .fold(Vec::new(), |mut vec, bytes| {
                vec.extend_from_slice(&bytes);
                vec
            });
        // push the data and calulate the length
        // request_len = Card16(
        //     (std::mem::size_of_val(&OPCODE)
        //         + std::mem::size_of_val(&depth)
        //         + std::mem::size_of_val(&window_id)
        //         + std::mem::size_of_val(&parent)
        //         + std::mem::size_of_val(&position_x)
        //         + std::mem::size_of_val(&position_y)
        //         + std::mem::size_of_val(&width)
        //         + std::mem::size_of_val(&height)
        //         + std::mem::size_of_val(&border_width)
        //         + std::mem::size_of_val(&class)
        //         + std::mem::size_of_val(&visual_id)
        //         + std::mem::size_of_val(&value_mask)
        //         + std::mem::size_of_val(&request_len)
        //         + std::mem::size_of_val(&values_bytes)) as u16,
        // );
        request_len = Card16(32);
        match self.byte_order {
            ByteOrder::BigEndian => {
                command_buffer.extend_from_slice(&OPCODE.to_be_bytes());
                command_buffer.extend_from_slice(&depth.to_be_bytes());
                command_buffer.extend_from_slice(&request_len.to_be_bytes());
                command_buffer.extend_from_slice(&window_id.to_be_bytes());
                command_buffer.extend_from_slice(&parent.to_be_bytes());
                command_buffer.extend_from_slice(&position_x.to_be_bytes());
                command_buffer.extend_from_slice(&position_y.to_be_bytes());
                command_buffer.extend_from_slice(&width.to_be_bytes());
                command_buffer.extend_from_slice(&height.to_be_bytes());
                command_buffer.extend_from_slice(&border_width.to_be_bytes());
                command_buffer.extend_from_slice(&class.to_be_bytes());
                command_buffer.extend_from_slice(&visual_id.to_be_bytes());
                command_buffer.extend_from_slice(&value_mask.to_be_bytes());
                command_buffer.extend_from_slice(&values_bytes);


            }
            ByteOrder::LittleEndian => {
                unimplemented!()
            }
        }

        self.stream.write_all(&command_buffer)?;
        self.stream.flush()?;
        let mut response_buffer = [0_u8; 512];
        let result = self.stream.read(&mut response_buffer);
        println!("Command Buffer: {command_buffer:?}");
        println!("response {response_buffer:?}");
        Ok(())
    }
}

pub trait X11Stream: std::io::Read + std::io::Write {}
impl X11Stream for std::os::unix::net::UnixStream {}
impl X11Stream for std::net::TcpStream {}

#[cfg(unix)]
pub fn connect_unix_socket(
    path: &str,
) -> Result<std::os::unix::net::UnixStream, Box<dyn std::error::Error>> {
    let stream = std::os::unix::net::UnixStream::connect(&path)?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    Ok(stream)
}
pub fn connect_tcp_ip<SocketAddress>(
    address: SocketAddress,
) -> Result<std::net::TcpStream, Box<dyn std::error::Error>>
where
    SocketAddress: Sized + std::net::ToSocketAddrs,
{
    let stream: TcpStream = TcpStream::connect(address)?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    Ok(stream)
}
pub trait TryFromIterBytesBe {
    fn try_from_iter_bytes_be(
        iterator: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

pub struct X11ConnectionInfo {
    release_id_base: Card32,
    release_number: Card32,
    resource_id_mask: Card32,
    motion_buffer_size: Card32,
    max_request_len: Card16,
    image_byte_order: ImageByteOrder,
    bitmap_format_bit_order: BitmapFormatBitOrder,
    bitmap_format_scanline_pad: Card8,
    bitmap_format_scanline_unit: Card8,
    min_keycode: KeyCode,
    max_keycode: KEYCODE,
    formats: Vec<PIXFORMAT>,
    roots: Vec<Screen>,
    protocol_major_version: Card16,
    protocol_minor_version: Card16,
    vendor_name: String8,
}

pub fn XOpenDisplay(display_str: &str) -> Result<Display, Box<dyn std::error::Error>> {
    Display::connect(display_str)
}
pub fn XCreateWindow(display: &mut Display,parent: WINDOW,x:Card16,y:Card16,width:Card16,height:Card16,border_width:Card16) {
    display.create_window()
}

pub fn XDefaultRootWindow(display: &mut Display) -> Result<WINDOW,Box<dyn std::error::Error>> {
    let screen = display.connection_info.roots.first().ok_or("Unable to get the first Screen inside the roots structure from the connection info. The default root window is not known")?;
    Ok(screen.root)
}

#[test]
pub fn test_create_window()  -> Result<(), Box<dyn std::error::Error>> {
    let display = std::env::var("DISPLAY").unwrap_or(":0".to_string());
    let mut connection = Display::connect(&display)?;
    let root_window = XDefaultRootWindow(display)?;
    let _ = XCreateWindow(&mut connection, parent, x, y, width, height, border_width);
    Ok(())
}

#[test]
pub fn test_initiate_connection() -> Result<(), Box<dyn std::error::Error>> {
    let display = std::env::var("DISPLAY").unwrap_or(":0".to_string());
    let connection = Display::connect(&display)?;
    Ok(())
}
#[test]
pub fn test_x_default_root_window() {
    let display = std::env::var("DISPLAY").unwrap_or(":0".to_string());
    let mut display = XOpenDisplay(&display).unwrap();
    let root_window = XDefaultRootWindow(&mut display)?;
    dbg!(root_window)

}