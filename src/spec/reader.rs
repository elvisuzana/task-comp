

pub struct Reader<'a> {
    inner: &'a [u8],
    pos: usize,
}
impl<'a> Reader<'a> {
    pub fn new<T: AsRef<[u8]>>(inner: &'a T) -> Self {
        Self {
            inner: inner.as_ref(),
            pos: 0,
        }
    }
    
    pub fn peek_u8(&self) -> u8 {
        self.inner[self.pos]
    }
    
    pub fn peek_u16(&self) -> u16 {
        u16::from_be_bytes(self.inner[self.pos..(self.pos + 2)].try_into().unwrap())
    }
    
    pub fn peek_u32(&self) -> u32 {
        u32::from_be_bytes(self.inner[self.pos..(self.pos + 4)].try_into().unwrap())
    }
    
    pub fn peek_u64(&self) -> u64 {
        u64::from_be_bytes(self.inner[self.pos..(self.pos + 8)].try_into().unwrap())
    }
    
    pub fn peek_i8(&self) -> i8 {
        self.peek_u8() as i8
    }
    
    pub fn peek_i16(&self) -> i16 {
        self.peek_u16() as i16
    }
    
    pub fn peek_i32(&self) -> i32 {
        self.peek_u32() as i32
    }
    
    pub fn peek_i64(&self) -> i64 {
        self.peek_u64() as i64
    }
    
    /// Peeks `len` bytes starting from the current position.
    pub fn peek_len(&self, len: usize) -> &[u8] {
        &self.inner[self.pos..(self.pos + len)]
    }
    
    /// Peeks `len` bytes starting from the current position, and returns the bytes in reversed order.
    /// 
    /// # Example
    /// ```
    /// use tasd::spec::reader::Reader;
    /// 
    /// let mut r = Reader::new(&[0x11, 0x22, 0x33, 0x44, 0x55]);
    /// r.advance(1);
    /// assert_eq!(r.peek_len(3),     &[0x22, 0x33, 0x44]);
    /// assert_eq!(r.peek_len_rev(3), &[0x44, 0x33, 0x22]);
    /// ```
    pub fn peek_len_rev(&self, len: usize) -> Vec<u8> {
        self.peek_len(len).iter().copied().rev().collect()
    }
    
    
    pub fn read_u8(&mut self) -> u8 {
        let data = self.inner[self.pos];
        self.pos += 1;
        
        data
    }
    
    pub fn read_u16(&mut self) -> u16 {
        let data = u16::from_be_bytes(self.inner[self.pos..(self.pos + 2)].try_into().unwrap());
        self.pos += 2;
        
        data
    }
    
    pub fn read_u32(&mut self) -> u32 {
        let data = u32::from_be_bytes(self.inner[self.pos..(self.pos + 4)].try_into().unwrap());
        self.pos += 4;
        
        data
    }
    
    pub fn read_u64(&mut self) -> u64 {
        let data = u64::from_be_bytes(self.inner[self.pos..(self.pos + 8)].try_into().unwrap());
        self.pos += 8;
        
        data
    }
    
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }
    
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }
    
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }
    
    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }
    
    
    pub fn read_bool(&mut self) -> bool {
        self.read_u8() > 0
    }
    
    pub fn read_len(&mut self, len: usize) -> &[u8] {
        let data = &self.inner[self.pos..(self.pos + len)];
        self.pos += len;
        
        data
    }
    
    pub fn read_string(&mut self, len: usize) -> String {
        String::from_utf8_lossy(self.read_len(len)).to_string()
    }
    
    pub fn read_remaining(&mut self) -> &[u8] {
        let data = &self.inner[self.pos..];
        self.pos += self.remaining();
        
        data
    }
    
    /// Copies entire buffer into a Vec regardless of current position.
    /// 
    /// Use [`Self::read_remaining`] if only the remaining data is needed.
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.to_vec()
    }
    
    pub fn advance(&mut self, count: usize) {
        self.pos += count;
    }
    
    pub fn rewind(&mut self, count: usize) {
        self.pos -= count;
    }
    
    pub fn remaining(&self) -> usize {
        self.inner.len() - self.pos
    }
    
    pub fn pos(&self) -> usize {
        self.pos
    }
    
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}





#[cfg(test)]
mod tests {
    use crate::spec::reader::Reader;
    
    const TEST_DATA: [[u8; 16]; 3] = [
        [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
        [0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87, 0x78, 0x69, 0x5A, 0x4B, 0x3C, 0x2D, 0x1E, 0x0F],
        [0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0xFF, 0x00, 0xAA, 0x55, 0x00, 0x69, 0x96, 0x00, 0x01, 0x00],
    ];
    
    #[test]
    fn controls() {
        for data in TEST_DATA {
            let mut r = Reader::new(&data);
            
            assert_eq!(r.pos, 0);
            assert_eq!(r.pos, r.pos());
            assert_eq!(r.inner, &data);
            
            let mut i = 0;
            for adv in [0, 0, 1, 1, 5, 23, 1963, 9] {
                r.advance(adv);
                i += adv;
                assert_eq!(r.pos, i);
                assert_eq!(r.pos, r.pos());
            }
            
            for set in [592, 11, 50, 99, 2, 0, 8, 9102502] {
                r.set_pos(set);
                assert_eq!(r.pos, set);
                assert_eq!(r.pos, r.pos());
            }
            
            r.set_pos(usize::MAX);
            let mut i = r.pos();
            for rev in [0, 1, 53, 3, 0] {
                r.rewind(rev);
                i -= rev;
                assert_eq!(r.pos, i);
                assert_eq!(r.pos, r.pos());
            }
            
            r.set_pos(0);
            for i in 0..data.len() {
                assert_eq!(r.remaining(), data.len() - i);
                r.advance(1);
            }
            
            r.set_pos(data.len());
            for i in 0..data.len() {
                assert_eq!(r.remaining(), i);
                r.rewind(1);
            }
        }
    }
    
    #[test]
    fn peeks() {
        for data in TEST_DATA {
            let mut r = Reader::new(&data);
            
            r.set_pos(0);
            for i in 0..data.len() {
                assert_eq!(r.peek_u8(), data[i]);
                assert_eq!(r.peek_i8(), data[i] as i8);
                r.advance(1);
            }
            
            r.set_pos(0);
            for i in 0..(data.len() - 1) {
                let expected = u16::from_be_bytes(data[i..(i + 2)].try_into().unwrap());
                assert_eq!(r.peek_u16(), expected);
                assert_eq!(r.peek_i16(), expected as i16);
                r.advance(1);
            }
            
            r.set_pos(0);
            for i in 0..(data.len() - 3) {
                let expected = u32::from_be_bytes(data[i..(i + 4)].try_into().unwrap());
                assert_eq!(r.peek_u32(), expected);
                assert_eq!(r.peek_i32(), expected as i32);
                r.advance(1);
            }
            
            r.set_pos(0);
            for i in 0..(data.len() - 7) {
                let expected = u64::from_be_bytes(data[i..(i + 8)].try_into().unwrap());
                assert_eq!(r.peek_u64(), expected);
                assert_eq!(r.peek_i64(), expected as i64);
                r.advance(1);
            }
            
            r.set_pos(0);
            for i in 0..data.len() {
                assert_eq!(r.peek_len(i), &data[..i]);
                assert_eq!(r.peek_len_rev(i), data[..i].into_iter().copied().rev().collect::<Vec<u8>>());
            }
        }
    }
    
    #[test]
    fn reads() {
        for data in TEST_DATA {
            let mut r = Reader::new(&data);
            
            r.set_pos(0);
            for i in 0..data.len() {
                assert_eq!(r.read_u8(), data[i]);
                r.rewind(1);
                assert_eq!(r.read_i8(), data[i] as i8);
                r.rewind(1);
                assert_eq!(r.read_bool(), data[i] > 0);
            }
            
            r.set_pos(0);
            for i in 0..(data.len() - 1) {
                let expected = u16::from_be_bytes(data[i..(i + 2)].try_into().unwrap());
                assert_eq!(r.read_u16(), expected);
                r.rewind(2);
                
                let expected = i16::from_be_bytes(data[i..(i + 2)].try_into().unwrap());
                assert_eq!(r.read_i16(), expected);
                r.rewind(1);
            }
            
            r.set_pos(0);
            for i in 0..(data.len() - 3) {
                let expected = u32::from_be_bytes(data[i..(i + 4)].try_into().unwrap());
                assert_eq!(r.read_u32(), expected);
                r.rewind(4);
                
                let expected = i32::from_be_bytes(data[i..(i + 4)].try_into().unwrap());
                assert_eq!(r.read_i32(), expected);
                r.rewind(3);
            }
            
            r.set_pos(0);
            for i in 0..(data.len() - 7) {
                let expected = u64::from_be_bytes(data[i..(i + 8)].try_into().unwrap());
                assert_eq!(r.read_u64(), expected);
                r.rewind(8);
                
                let expected = i64::from_be_bytes(data[i..(i + 8)].try_into().unwrap());
                assert_eq!(r.read_i64(), expected);
                r.rewind(7);
            }
            
            for i in 0..data.len() {
                r.set_pos(0);
                assert_eq!(r.read_len(i), &data[..i]);
                
                r.set_pos(data.len() - i);
                assert_eq!(r.read_len(i), &data[(data.len() - i)..]);
                
                r.set_pos(i);
                assert_eq!(r.read_remaining(), &data[i..]);
                
                r.set_pos(0);
                assert_eq!(r.read_string(i), String::from_utf8_lossy(&data[..i]));
            }
        }
    }
    
    #[test]
    fn conversion() {
        for data in TEST_DATA {
            let mut r = Reader::new(&data);
            
            for i in 0..data.len() {
                r.set_pos(i);
                assert_eq!(r.read_remaining(), &data[i..]);
                assert_eq!(r.read_remaining(), &[]);
                
                assert_eq!(r.to_vec(), &data);
            }
        }
    }
}