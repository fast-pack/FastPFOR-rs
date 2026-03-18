use crate::rust::{FastPForError, FastPForResult};

/// Finds the greatest multiple of `factor` that is less than or equal to `value`.
pub fn greatest_multiple(value: u32, factor: u32) -> u32 {
    value - value % factor
}

/// Returns the number of bits needed to represent `i`.
/// Returns 0 for input 0.
pub fn bits(i: u32) -> usize {
    32 - i.leading_zeros() as usize
}

pub trait AsUsize: Eq + Copy {
    fn as_usize(self) -> usize;
}

impl AsUsize for usize {
    #[inline]
    fn as_usize(self) -> usize {
        self
    }
}

impl AsUsize for u32 {
    #[inline]
    fn as_usize(self) -> usize {
        const _: () = {
            // Some day Rust may support usize smaller than u32?
            assert!(
                size_of::<u32>() <= size_of::<usize>(),
                "usize must be able to hold all u32 values"
            );
        };

        #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        {
            self as usize
        }
    }
}

pub trait GetWithErr<T> {
    fn get_val(&self, pos: impl AsUsize) -> FastPForResult<T>;
}

impl<T: Copy> GetWithErr<T> for &[T] {
    #[inline]
    fn get_val(&self, pos: impl AsUsize) -> FastPForResult<T> {
        self.get(pos.as_usize())
            .copied()
            .ok_or(FastPForError::NotEnoughData)
    }
}

impl<T: Copy> GetWithErr<T> for Vec<T> {
    #[inline]
    fn get_val(&self, pos: impl AsUsize) -> FastPForResult<T> {
        self.as_slice().get_val(pos)
    }
}
