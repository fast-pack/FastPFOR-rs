use crate::FastPForResult;
use crate::codec::AnyLenCodec;
use crate::helpers::AsUsize;

/// A no-op codec that copies data without compression.
///
/// Useful as a baseline for benchmarking or when a codec interface is required.
#[derive(Debug, Default)]
pub struct JustCopy;

impl JustCopy {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl AnyLenCodec for JustCopy {
    fn encode(&mut self, input: &[u32], out: &mut Vec<u32>) -> FastPForResult<()> {
        out.extend_from_slice(input);
        Ok(())
    }

    fn decode(
        &mut self,
        input: &[u32],
        out: &mut Vec<u32>,
        expected_len: Option<u32>,
    ) -> FastPForResult<()> {
        if let Some(expected) = expected_len {
            let expected = expected.is_valid_expected(Self::max_decompressed_len(input.len()))?;
            input.len().is_decoded_mismatch(expected)?;
        }
        out.extend_from_slice(input);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{decompress, roundtrip, roundtrip_expected};

    #[test]
    fn justcopy_roundtrip() {
        roundtrip::<JustCopy>(&[1u32, 2, 3]);
    }

    #[test]
    fn justcopy_roundtrip_with_expected_len_none() {
        roundtrip_expected::<JustCopy>(&[1u32, 2, 3], None);
    }

    #[test]
    fn justcopy_decode_expected_len_mismatch_errors() {
        decompress::<JustCopy>(&[1u32, 2, 3], Some(2)).unwrap_err();
    }
}
