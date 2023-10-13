use std::cmp::{max, min};
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
    
    pub fn write_u8_str(&mut self, data: &str) {
        let data = data.as_bytes();
        let len = min(data.len(), 255);
        
        self.write_u8(len as u8);
        self.inner.extend_from_slice(&data[..len]);
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
    
    /// Returns a clone of this [Writer]'s internal buffer.
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }
}





#[cfg(test)]
mod tests {
    use std::array::from_fn;
    use std::cmp::min;
    use crate::spec::writer::Writer;
    
    #[test]
    fn writes() {
        fn perform<T: Copy, I: IntoIterator<Item = T>, F: Fn(&mut Writer, &mut Vec<u8>, T)>(pattern: I, func: F) {
            let mut w = Writer::new();
            let mut data = Vec::with_capacity(256);
            for i in pattern.into_iter() {
                func(&mut w, &mut data, i);
            }
            assert_eq!(w.inner, data);
        }
        
        perform(u8::MIN..=u8::MAX, |w, expected, data| {
            w.write_u8(data);
            expected.push(data);
        });
        
        perform(-128..=127, |w, expected, data| {
            w.write_i8(data);
            expected.push(data as u8);
        });
        
        const U16PAT: [u16; 8] = [0x0000, 0x55AA, 0xAA55, 0xFFFF, 0x1234, 0x6969, 0x0FA1, 0x1010];
        perform(U16PAT, |w, expected, data| {
            w.write_u16(data);
            expected.extend_from_slice(&data.to_be_bytes());
        });
        
        const I16PAT: [i16; 8] = [0x0000, 0x55AA, -0x7A55, -0x8000, 0x1234, 0x6969, 0x0FA1, 0x1010];
        perform(I16PAT, |w, expected, data| {
            w.write_i16(data);
            expected.extend_from_slice(&data.to_be_bytes());
        });
        
        const U32PAT: [u32; 8] = [0x00000000, 0x55AA33DD, 0xAA55DD33, 0xFFFFFFFF, 0x12345678, 0x69696969, 0x0FA17C15, 0x10101010];
        perform(U32PAT, |w, expected, data| {
            w.write_u32(data);
            expected.extend_from_slice(&data.to_be_bytes());
        });
        
        const I32PAT: [i32; 8] = [0x00000000, 0x55AA33DD, -0x7A55DD33, -0x80000000, 0x12345678, 0x69696969, 0x0FA17C15, 0x10101010];
        perform(I32PAT, |w, expected, data| {
            w.write_i32(data);
            expected.extend_from_slice(&data.to_be_bytes());
        });
        
        const U64PAT: [u64; 8] = [0x0000000000000000, 0x55AA55AA55AA55AA, 0xAA55AA55AA55AA55, 0xFFFFFFFFFFFFFFFF, 0x123456789ABCDEF, 0x6969696969696969, 0x0FA17C15A6B90D38, 0x1010101010101010];
        perform(U64PAT, |w, expected, data| {
            w.write_u64(data);
            expected.extend_from_slice(&data.to_be_bytes());
        });
        
        const I64PAT: [i64; 8] = [0x0000000000000000, 0x55AA55AA55AA55AA, -0x7A55AA55AA55AA55, -0x8000000000000000, 0x123456789ABCDEF, 0x6969696969696969, 0x0FA17C15A6B90D38, 0x1010101010101010];
        perform(I64PAT, |w, expected, data| {
            w.write_i64(data);
            expected.extend_from_slice(&data.to_be_bytes());
        });
        
        perform([false, true, false, false, true, true, false, false, false, true, true, true], |w, expected, data| {
            w.write_bool(data);
            expected.push(data as u8);
        });
        
        perform(["foo", "bar", "", "fish", "tasd", "hello world!", "lorem ipsum"], |w, expected, data| {
            w.write_str(data);
            expected.extend_from_slice(data.as_bytes());
        });
        
        perform(["foo", "bar", "", "fish", "tasd", "hello world!", "lorem ipsum"], |w, expected, data| {
            w.write_u8_str(data);
            expected.push(data.len() as u8);
            expected.extend_from_slice(data.as_bytes());
        });
        for len in 0..280 {
            let mut w = Writer::new();
            let s = String::from_utf8(vec![0x5A; len]).unwrap();
            w.write_u8_str(&s);
            assert_eq!(w.inner.len(), 1 + min(len, 255));
            
            let mut expected = Vec::with_capacity(512);
            expected.push(min(len, 255) as u8);
            expected.extend_from_slice(&s.as_bytes()[..min(len, 255)]);
            assert_eq!(w.inner, expected);
        }
        
        let mut w = Writer::new();
        w.write_iter(0..=255);
        assert_eq!(w.inner, (0..=255u8).into_iter().collect::<Vec<u8>>());
    }
    
    #[test]
    fn conversion() {
        let data = [0x11, 0x22, 0x33, 0xA5, 0x5A, 0x00, 0xFF];
        let mut w = Writer::new();
        
        w.write_slice(&data);
        assert_eq!(w.to_vec(), data);
        
        let mut w = Writer::new();
        let data: [u8; 0x105A5] = from_fn(|i| i as u8);
        w.write_iter(data.clone());
        
        let mut packet = vec![
            0x5A, 0xA5,
            0x03, 0x01, 0x05, 0xA5
        ];
        packet.extend_from_slice(&data);
        assert_eq!(w.into_packet(&[0x5A, 0xA5], 2), packet);
    }
}