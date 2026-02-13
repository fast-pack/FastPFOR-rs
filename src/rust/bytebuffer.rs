use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug)]
pub struct ByteBuffer {
    pub buffer: BytesMut,
}

impl ByteBuffer {
    #[must_use]
    pub fn new(capacity: u32) -> Self {
        ByteBuffer {
            buffer: BytesMut::with_capacity(capacity as usize),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    #[must_use]
    pub fn position(&self) -> u32 {
        self.buffer.len() as u32
    }

    pub fn put(&mut self, byte: u8) {
        self.buffer.put_u8(byte);
    }

    #[must_use]
    pub fn get(&mut self) -> u8 {
        // move read cursor
        self.buffer.get_u8()
    }

    #[must_use]
    pub fn get_u32_le(&mut self) -> u32 {
        self.buffer.get_u32_le()
    }

    pub fn put_u32_le(&mut self, value: u32) {
        self.buffer.put_u32_le(value);
    }
}
