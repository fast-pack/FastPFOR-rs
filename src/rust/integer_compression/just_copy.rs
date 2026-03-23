use crate::FastPForResult;
use crate::codec::AnyLenCodec;
use crate::helpers::AsUsize;

/// A no-op codec that copies data without compression.
///
/// Useful as a baseline for benchmarking or when a codec interface is required.
#[derive(Debug)]
pub struct JustCopy;

impl JustCopy {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        JustCopy
    }
}

impl Default for JustCopy {
    fn default() -> Self {
        JustCopy::new()
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
    use crate::FastPForError;
    use crate::test_utils::{decompress, roundtrip};

    #[test]
    fn justcopy_default_and_roundtrip() {
        roundtrip::<JustCopy>(&[1u32, 2, 3]);
    }

    #[test]
    fn justcopy_decode_with_expected_len_ok() {
        let data = vec![1u32, 2, 3];
        let out = decompress::<JustCopy>(&data, Some(3));
        assert_eq!(out, data);
    }

    #[test]
    #[expect(clippy::default_constructed_unit_structs)]
    fn justcopy_decode_expected_len_mismatch_errors() {
        let data = vec![1u32, 2, 3];
        let err = JustCopy::default()
            .decode(&data, &mut Vec::new(), Some(2))
            .unwrap_err();
        assert!(matches!(
            err,
            FastPForError::DecodedCountMismatch {
                actual: 3,
                expected: 2
            }
        ));
    }
}
