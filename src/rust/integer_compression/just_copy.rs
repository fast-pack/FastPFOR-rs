use crate::FastPForError;
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
    fn encode(&mut self, input: &[u32], out: &mut Vec<u32>) -> Result<(), FastPForError> {
        out.extend_from_slice(input);
        Ok(())
    }

    fn decode(
        &mut self,
        input: &[u32],
        out: &mut Vec<u32>,
        expected_len: Option<u32>,
    ) -> Result<(), FastPForError> {
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

    #[test]
    fn justcopy_default_and_roundtrip() {
        // Exercise the Default impl explicitly.
        let mut codec = <JustCopy as Default>::default();
        let data = vec![1u32, 2, 3];
        let mut compressed = Vec::new();
        codec.encode(&data, &mut compressed).unwrap();
        let mut decoded = Vec::new();
        codec.decode(&compressed, &mut decoded, None).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn justcopy_decode_with_expected_len_ok() {
        let data = vec![1u32, 2, 3];
        let mut out = Vec::new();
        JustCopy::new().decode(&data, &mut out, Some(3)).unwrap();
        assert_eq!(out, data);
    }

    #[test]
    fn justcopy_decode_expected_len_mismatch_errors() {
        let data = vec![1u32, 2, 3];
        let err = JustCopy::new()
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
