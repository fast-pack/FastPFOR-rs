use cxx::UniquePtr;

use crate::FastPForResult;
use crate::codec::default_max_decoded_len;
use crate::cpp::ffi;
use crate::helpers::AsUsize;

/// Pass-through to C++ `encodeArray` / `decodeArray`. No extra header is added.
///
/// Block-based C++ codecs (`FastPFor`, `PFor`, etc.) store the original data length
/// in their own wire format. Byte-oriented codecs (`VariableByte`) rely on the
/// caller passing the encoded stream length to decode, which we have via `input.len()`.
pub fn encode32_to_vec_ffi(
    codec: &UniquePtr<ffi::IntegerCODEC>,
    input: &[u32],
    out: &mut Vec<u32>,
) -> FastPForResult<()> {
    let capacity = input.len() * 2 + 1024;
    let start = out.len();
    out.resize(start + capacity, 0);
    let n = ffi::codec_encode32(codec, input, &mut out[start..])?;
    out.truncate(start + n);
    Ok(())
}

fn decode32_to_vec_ffi(
    codec: &UniquePtr<ffi::IntegerCODEC>,
    input: &[u32],
    out: &mut Vec<u32>,
    capacity: usize,
) -> FastPForResult<()> {
    if !input.is_empty() {
        let start = out.len();
        out.resize(start + capacity, 0);
        let n = ffi::codec_decode32(codec, input, &mut out[start..])?;
        out.truncate(start + n);
    }
    Ok(())
}

pub fn decode32_anylen_ffi(
    codec: &UniquePtr<ffi::IntegerCODEC>,
    input: &[u32],
    out: &mut Vec<u32>,
    expected_len: Option<u32>,
) -> FastPForResult<()> {
    let max = default_max_decoded_len(input.len());
    let capacity = if let Some(n) = expected_len {
        n.is_valid_expected(max)?
    } else {
        // C++ decodeArray needs output buffer capacity. Block codecs read count from stream;
        // variable-byte decodes until input is consumed. Simple9/16 pack up to 28 values/word.
        max
    };
    let start = out.len();
    decode32_to_vec_ffi(codec, input, out, capacity)?;
    if let Some(n) = expected_len {
        (out.len() - start).is_decoded_mismatch(n)?;
    }
    Ok(())
}

pub fn encode64_to_vec_ffi(
    codec: &UniquePtr<ffi::IntegerCODEC>,
    input: &[u64],
    out: &mut Vec<u32>,
) -> FastPForResult<()> {
    let capacity = input.len() * 3 + 1024;
    let start = out.len();
    out.resize(start + capacity, 0);
    let n = ffi::codec_encode64(codec, input, &mut out[start..])?;
    out.truncate(start + n);
    Ok(())
}

pub fn decode64_to_vec_ffi(
    codec: &UniquePtr<ffi::IntegerCODEC>,
    input: &[u32],
    out: &mut Vec<u64>,
) -> FastPForResult<()> {
    if !input.is_empty() {
        // C++ decodeArray needs output buffer. Variable-byte can pack multiple values per word.
        let capacity = input.len().saturating_mul(4);
        let start = out.len();
        out.resize(start + capacity, 0);
        let n = ffi::codec_decode64(codec, input, &mut out[start..])?;
        out.truncate(start + n);
    }
    Ok(())
}
