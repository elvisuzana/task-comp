use std::path::PathBuf;
use crate::spec::packets::{Packet, PacketError};
use crate::spec::reader::Reader;

pub mod packets;
pub mod reader;
pub mod writer;

pub const LATEST_VERSION: [u8; 2] = [0x00, 0x01];
pub const MAGIC_NUMBER: [u8; 4] = [0x54, 0x41, 0x53, 0x44];

#[derive(Debug)]
pub enum TasdError {
    Io(std::io::Error),
    Packet(PacketError),
    MissingHeader,
    MagicNumberMismatch(Vec<u8>),
}
impl From<std::io::Error> for TasdError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<PacketError> for TasdError {
    fn from(value: PacketError) -> Self {
        Self::Packet(value)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TasdFile {
    pub version: u16,
    pub keylen: u8,
    pub packets: Vec<Packet>,
    pub path: Option<PathBuf>,
}
impl Default for TasdFile {
    fn default() -> Self { Self {
        version: u16::from_be_bytes(LATEST_VERSION),
        keylen: 2,
        packets: vec![],
        path: None
    }}
}
impl TasdFile {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn parse_file<P: Into<PathBuf>>(path: P) -> Result<Self, TasdError> {
        let path = path.into();
        let data = std::fs::read(&path)?;
        let mut file = Self::parse_slice(&data)?;
        file.path = Some(path);
        
        Ok(file)
    }
    
    pub fn parse_slice(data: &[u8]) -> Result<Self, TasdError> {
        let mut r = Reader::new(&data);
        if r.remaining() < 7 {
            return Err(TasdError::MissingHeader);
        }
        let magic = r.read_len(4);
        if magic != MAGIC_NUMBER {
            return Err(TasdError::MagicNumberMismatch(magic.to_vec()));
        }
        
        let mut file = Self {
            version: r.read_u16(),
            keylen: r.read_u8(),
            packets: vec![],
            path: None,
        };
        
        while r.remaining() > 0 {
            use PacketError::*;
            match Packet::with_reader(&mut r, file.keylen) {
                Ok(packet) => file.packets.push(packet),
                Err(err) => match err {
                    MissingKey | MismatchedKey | MissingPayloadLength | UnsupportedExponent(_) => return Err(err.into()),
                    InvalidPayload { key, payload } => println!("InvalidPayload! Skipping. ({key:02X?}, {payload:02X?}"),
                }
            }
        }
        
        Ok(file)
    }
}
