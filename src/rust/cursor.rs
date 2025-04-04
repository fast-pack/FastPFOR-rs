use std::io::Cursor;

pub trait IncrementCursor {
    fn increment(&mut self);
    fn add(&mut self, n: u32);
}

impl IncrementCursor for Cursor<u32> {
    fn increment(&mut self) {
        self.set_position(self.position() + 1); // Position needs to be a u64
    }
    fn add(&mut self, n: u32) {
        self.set_position(self.position() + u64::from(n));
    }
}
