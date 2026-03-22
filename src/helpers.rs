use crate::FastPForError;

/// Finds the greatest multiple of `factor` that is less than or equal to `value`.
#[cfg_attr(feature = "cpp", allow(dead_code))]
pub fn greatest_multiple(value: u32, factor: u32) -> u32 {
    value - value % factor
}

/// Returns the number of bits needed to represent `i`.
/// Returns 0 for input 0.
#[cfg_attr(feature = "cpp", allow(dead_code))]
pub fn bits(i: u32) -> usize {
    32 - i.leading_zeros().as_usize()
}

pub trait AsUsize: Eq + Copy {
    fn as_usize(self) -> usize;

    #[inline]
    #[cfg(feature = "cpp")]
    fn is_decoded_mismatch(self, expected: impl AsUsize) -> Result<(), FastPForError> {
        let actual = self.as_usize();
        let expected = expected.as_usize();
        if self.as_usize() == expected {
            Ok(())
        } else {
            Err(FastPForError::DecodedCountMismatch { actual, expected })
        }
    }

    /// Returns an error if `expected` exceeds `max`.
    #[inline]
    #[cfg(feature = "cpp")]
    fn is_valid_expected(self, max: impl AsUsize) -> Result<usize, FastPForError> {
        let expected = self.as_usize();
        let max = max.as_usize();
        if expected > max {
            Err(FastPForError::ExpectedCountExceedsMax { expected, max })
        } else {
            Ok(expected)
        }
    }
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

#[cfg_attr(feature = "cpp", allow(dead_code))]
pub trait GetWithErr<T> {
    fn get_val(&self, pos: impl AsUsize) -> Result<T, FastPForError>;
}

impl<T: Copy> GetWithErr<T> for &[T] {
    #[inline]
    fn get_val(&self, pos: impl AsUsize) -> Result<T, FastPForError> {
        self.get(pos.as_usize())
            .copied()
            .ok_or(FastPForError::NotEnoughData)
    }
}

impl<T: Copy> GetWithErr<T> for Vec<T> {
    #[inline]
    fn get_val(&self, pos: impl AsUsize) -> Result<T, FastPForError> {
        self.as_slice().get_val(pos)
    }
}
