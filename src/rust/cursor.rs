use std::io::Cursor;

/// Extension trait for `Cursor<u32>` providing position increment operations.
pub trait IncrementCursor {
    /// Increments the cursor position by 1.
    fn increment(&mut self);
    /// Adds `n` to the cursor position.
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
