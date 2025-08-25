#[derive(Debug)]
pub struct ByteBuffer {
    pub buffer: Vec<u8>,
    position: u32,
    limit: u32,
    capacity: u32,
}

#[derive(Debug)]
pub struct IntBuffer {
    pub buffer: Vec<u32>,
}

impl ByteBuffer {
    /// Create a new [`ByteBuffer`] with the specified capacity
    pub fn new(capacity: u32) -> Self {
        ByteBuffer {
            buffer: vec![0; capacity as usize],
            position: 0,
            limit: capacity,
            capacity,
        }
    }

    /// Clear the buffer, resetting position and limit
    pub fn clear(&mut self) {
        self.position = 0;
        self.limit = self.capacity;
    }

    /// Get the current position
    pub fn position(&self) -> u32 {
        self.position
    }

    /// Set a new position
    pub fn set_position(&mut self, pos: u32) {
        assert!(pos <= self.limit, "Position exceeds limit");
        self.position = pos;
    }

    // Write a single byte to the buffer
    pub fn put(&mut self, byte: u8) {
        assert!(self.position < self.limit, "Buffer overflow");

        if self.position as usize >= self.buffer.len() {
            // Extend the buffer with zeros to fill the gap
            self.buffer.resize(self.position as usize + 1, 0);
        }

        self.buffer[self.position as usize] = byte; // Write the byte at the position
        self.position += 1; // Increment the position
    }

    // Read a single byte from the buffer
    pub fn get(&mut self) -> u8 {
        assert!(self.position < self.limit, "Buffer underflow");
        let byte = self.buffer[self.position as usize];
        self.position += 1;
        byte
    }

    /// Get the internal buffer as a slice
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }

    /// Flip the buffer, setting the limit to the current position
    pub fn flip(&mut self) {
        self.limit = self.position;
        self.position = 0;
    }

    /// Get the buffer as a slice of integers
    /// <https://www.dotnetperls.com/convert-bytes-integer-rust>
    pub fn as_int_buffer(&self) -> IntBuffer {
        // Ensure the buffer length is a multiple of 4
        assert_eq!(
            self.buffer.len() % 4,
            0,
            "Buffer length is not a multiple of 4"
        );

        IntBuffer {
            buffer: {
                let mut result = vec![];
                for chunk in self.buffer.chunks(4) {
                    let mut bytes = [0; 4];
                    bytes.copy_from_slice(chunk);
                    result.push(u32::from_le_bytes(bytes));
                }
                result
            },
        }
    }
}

impl IntBuffer {
    /// Create a new [`IntBuffer`] with the specified capacity
    pub fn get(&self, dst: &mut [u32], offset: usize, length: usize) {
        assert!(offset + length <= dst.len(), "Buffer overflow");
        dst[offset..offset + length].copy_from_slice(&self.buffer[..length]);
    }

    #[expect(clippy::unused_self)]
    pub fn put(&mut self, src: &[u32], offset: usize, length: u32) -> Vec<u8> {
        // FIXME: why is self not used here??
        assert!(offset + length as usize <= src.len(), "Buffer underflow");
        let mut result: Vec<u8> = vec![];
        for value in &src[offset..offset + (length as usize)] {
            result.extend_from_slice(&value.to_le_bytes());
        }
        result
    }
}
