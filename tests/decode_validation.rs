//! Integration tests for **untrusted** compressed input: decoding must reject malformed
//! streams with [`fastpfor::FastPForResult::Err`], not panic.
#![cfg(feature = "rust")]

use fastpfor::{AnyLenCodec, FastPFor128, FastPFor256};

/// `compressed` must not be a valid stream for `codec`. Decoding must return `Err`.
fn assert_fails<C: AnyLenCodec + Default>(compressed: &[u32]) {
    let mut codec = C::default();
    let mut out = Vec::new();
    assert!(
        codec.decode(compressed, &mut out, None).is_err(),
        "expected decode to fail with Err, but it succeeded"
    );
}

#[test]
fn test1() {
    let data: &[u32] = &[
        42_926_275,
        589_967,
        4_522_053,
        589_967,
        3_646_554_563,
        55_438,
        u32::MAX,
        36,
    ];
    assert_fails::<FastPFor128>(data);
    assert_fails::<FastPFor256>(data);
}

/// Minimal garbage: tiny slice that cannot be a well-formed composite block stream.
#[test]
fn test2() {
    let data = &[0x200, 0, 1];
    assert_fails::<FastPFor128>(data);
    assert_fails::<FastPFor256>(data);
}
