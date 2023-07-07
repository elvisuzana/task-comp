

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
    
    pub fn peek_i8(&mut self) -> i8 {
        self.peek_u8() as i8
    }
    
    pub fn peek_i16(&mut self) -> i16 {
        self.peek_u16() as i16
    }
    
    pub fn peek_i32(&mut self) -> i32 {
        self.peek_u32() as i32
    }
    
    pub fn peek_i64(&mut self) -> i64 {
        self.peek_u64() as i64
    }
    
    pub fn peek_len(&self, len: usize) -> &[u8] {
        &self.inner[self.pos..(self.pos + len)]
    }
    
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