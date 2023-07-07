use std::cmp::max;
use crate::util::to_bytes;

pub struct Writer {
    inner: Vec<u8>,
}
impl Writer {
    pub fn new() -> Self {
        Self {
            inner: vec![]
        }
    }
    
    pub fn write_u8(&mut self, data: u8) {
        self.inner.push(data);
    }
    
    pub fn write_u16(&mut self, data: u16) {
        self.inner.extend_from_slice(&data.to_be_bytes());
    }
    
    pub fn write_u32(&mut self, data: u32) {
        self.inner.extend_from_slice(&data.to_be_bytes());
    }
    
    pub fn write_u64(&mut self, data: u64) {
        self.inner.extend_from_slice(&data.to_be_bytes());
    }
    
    pub fn write_i8(&mut self, data: i8) {
        self.write_u8(data as u8);
    }
    
    pub fn write_i16(&mut self, data: i16) {
        self.write_u16(data as u16);
    }
    
    pub fn write_i32(&mut self, data: i32) {
        self.write_u32(data as u32);
    }
    
    pub fn write_i64(&mut self, data: i64) {
        self.write_u64(data as u64);
    }
    
    pub fn write_bool(&mut self, data: bool) {
        self.inner.push(data as u8);
    }
    
    pub fn write_str(&mut self, data: &str) {
        self.inner.extend_from_slice(data.as_bytes());
    }
    
    pub fn write_option_string(&mut self, data: &Option<String>) {
        self.inner.extend_from_slice(data.as_ref().unwrap_or(&"".into()).as_bytes());
    }
    
    pub fn write_slice(&mut self, data: &[u8]) {
        self.inner.extend_from_slice(data);
    }
    
    pub fn write_iter<I: IntoIterator<Item = u8>>(&mut self, data: I) {
        self.inner.extend(data);
    }
    
    pub fn into_packet(self, key: &[u8], keylen: u8) -> Vec<u8> {
        let key = {
            let mut resized_key = vec![0u8; max(key.len(), keylen as usize) - key.len()];
            resized_key.extend_from_slice(key);
            resized_key
        };
        
        let exp = {
            let mut tmp = self.inner.len();
            let mut exp = 0u8;
            while tmp > 0 {
                tmp >>= 8;
                exp += 1;
            }
            exp
        };
        let plen = to_bytes(self.inner.len(), exp);
        
        let mut data = Vec::with_capacity(self.inner.len() + key.len() + 1 + exp as usize);
        data.extend_from_slice(&key);
        data.push(exp);
        data.extend_from_slice(&plen);
        data.extend_from_slice(&self.inner);
        
        data
    }
}