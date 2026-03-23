// The fuzz crate always enables both "rust" and "cpp" features of fastpfor.
// Items here may only be used by some binaries; suppress dead_code lint.
#![allow(dead_code)]

use fastpfor::cpp::*;
use fastpfor::{AnyLenCodec, FastPFor128, FastPFor256, FastPForResult, JustCopy, VariableByte};

// ── Debug helper ─────────────────────────────────────────────────────────────

pub struct HexSlice<'a>(pub &'a [u32]);

impl std::fmt::Debug for HexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MAX: usize = 20;

        let total = self.0.len();
        let shown = total.min(MAX);

        let mut list = f.debug_list();

        for v in &self.0[..shown] {
            list.entry(&format_args!("{v:#010x}"));
        }

        if total > MAX {
            list.entry(&format_args!(".. out of {total} total"));
        }

        list.finish()
    }
}

/// A fuzz input pairing arbitrary data with a codec selector.
#[derive(arbitrary::Arbitrary, Debug)]
pub struct FuzzInput<C> {
    pub data: Vec<u32>,
    pub codec: C,
}

// `AnyLenCodec` is not dyn-compatible; carry a closed enum of every fuzzed codec instead.
macro_rules! define_fuzz_any_len {
    ($( $variant:ident ( $ty:ty ) ),* $(,)?) => {
        pub enum FuzzAnyLen {
            $( $variant($ty), )*
        }

        impl FuzzAnyLen {
            pub fn encode(&mut self, input: &[u32], out: &mut Vec<u32>) -> FastPForResult<()> {
                match self {
                    $( Self::$variant(codec) => codec.encode(input, out), )*
                }
            }

            pub fn decode(
                &mut self,
                input: &[u32],
                out: &mut Vec<u32>,
                expected_len: Option<u32>,
            ) -> FastPForResult<()> {
                match self {
                    $( Self::$variant(codec) => codec.decode(input, out, expected_len), )*
                }
            }
        }
    };
}

define_fuzz_any_len!(
    FastPFor256(FastPFor256),
    FastPFor128(FastPFor128),
    VariableByte(VariableByte),
    JustCopy(JustCopy),
    CppBP32(CppBP32),
    CppCopy(CppCopy),
    CppFastBinaryPacking8(CppFastBinaryPacking8),
    CppFastPFor128(CppFastPFor128),
    CppFastPFor256(CppFastPFor256),
    CppFastBinaryPacking16(CppFastBinaryPacking16),
    CppFastBinaryPacking32(CppFastBinaryPacking32),
    CppMaskedVByte(CppMaskedVByte),
    CppNewPFor(CppNewPFor),
    CppOptPFor(CppOptPFor),
    CppPFor2008(CppPFor2008),
    CppPFor(CppPFor),
    CppSimdBinaryPacking(CppSimdBinaryPacking),
    CppSimdFastPFor128(CppSimdFastPFor128),
    CppSimdFastPFor256(CppSimdFastPFor256),
    CppSimdGroupSimple(CppSimdGroupSimple),
    CppSimdGroupSimpleRingBuf(CppSimdGroupSimpleRingBuf),
    CppSimdNewPFor(CppSimdNewPFor),
    CppSimdOptPFor(CppSimdOptPFor),
    CppSimdPFor(CppSimdPFor),
    CppSimdSimplePFor(CppSimdSimplePFor),
    CppStreamVByte(CppStreamVByte),
    CppVByte(CppVByte),
    CppVarInt(CppVarInt),
    CppVarIntGb(CppVarIntGb),
);

pub type CodecEntry = (&'static str, fn() -> FuzzAnyLen);

macro_rules! codec_ctor_fn {
    ($fn_name:ident, $variant:ident, $ty:ty) => {
        fn $fn_name() -> FuzzAnyLen {
            FuzzAnyLen::$variant(<$ty>::default())
        }
    };
}

codec_ctor_fn!(make_rust_fastpfor256, FastPFor256, FastPFor256);
codec_ctor_fn!(make_rust_fastpfor128, FastPFor128, FastPFor128);

fn make_rust_variable_byte() -> FuzzAnyLen {
    FuzzAnyLen::VariableByte(VariableByte)
}

fn make_rust_just_copy() -> FuzzAnyLen {
    FuzzAnyLen::JustCopy(JustCopy)
}

/// Rust codecs. Block codecs are wrapped in `CompositeCodec<_, VariableByte>`.
pub static RUST: &[CodecEntry] = &[
    ("FastPFor256", make_rust_fastpfor256),
    ("FastPFor128", make_rust_fastpfor128),
    ("VariableByte", make_rust_variable_byte),
    ("JustCopy", make_rust_just_copy),
];

codec_ctor_fn!(make_cpp_bp32, CppBP32, CppBP32);
codec_ctor_fn!(make_cpp_copy, CppCopy, CppCopy);
codec_ctor_fn!(
    make_cpp_fast_binary_packing8,
    CppFastBinaryPacking8,
    CppFastBinaryPacking8
);
codec_ctor_fn!(make_cpp_fastpfor128, CppFastPFor128, CppFastPFor128);
codec_ctor_fn!(make_cpp_fastpfor256, CppFastPFor256, CppFastPFor256);
codec_ctor_fn!(
    make_cpp_fast_binary_packing16,
    CppFastBinaryPacking16,
    CppFastBinaryPacking16
);
codec_ctor_fn!(
    make_cpp_fast_binary_packing32,
    CppFastBinaryPacking32,
    CppFastBinaryPacking32
);
codec_ctor_fn!(make_cpp_masked_vbyte, CppMaskedVByte, CppMaskedVByte);
codec_ctor_fn!(make_cpp_new_pfor, CppNewPFor, CppNewPFor);
codec_ctor_fn!(make_cpp_opt_pfor, CppOptPFor, CppOptPFor);
codec_ctor_fn!(make_cpp_pfor2008, CppPFor2008, CppPFor2008);
codec_ctor_fn!(make_cpp_pfor, CppPFor, CppPFor);
codec_ctor_fn!(
    make_cpp_simd_binary_packing,
    CppSimdBinaryPacking,
    CppSimdBinaryPacking
);
codec_ctor_fn!(
    make_cpp_simd_fastpfor128,
    CppSimdFastPFor128,
    CppSimdFastPFor128
);
codec_ctor_fn!(
    make_cpp_simd_fastpfor256,
    CppSimdFastPFor256,
    CppSimdFastPFor256
);
codec_ctor_fn!(
    make_cpp_simd_group_simple,
    CppSimdGroupSimple,
    CppSimdGroupSimple
);
codec_ctor_fn!(
    make_cpp_simd_group_simple_ring_buf,
    CppSimdGroupSimpleRingBuf,
    CppSimdGroupSimpleRingBuf
);
codec_ctor_fn!(make_cpp_simd_new_pfor, CppSimdNewPFor, CppSimdNewPFor);
codec_ctor_fn!(make_cpp_simd_opt_pfor, CppSimdOptPFor, CppSimdOptPFor);
codec_ctor_fn!(make_cpp_simd_pfor, CppSimdPFor, CppSimdPFor);
codec_ctor_fn!(
    make_cpp_simd_simple_pfor,
    CppSimdSimplePFor,
    CppSimdSimplePFor
);
codec_ctor_fn!(make_cpp_stream_vbyte, CppStreamVByte, CppStreamVByte);
codec_ctor_fn!(make_cpp_vbyte, CppVByte, CppVByte);
codec_ctor_fn!(make_cpp_var_int, CppVarInt, CppVarInt);
codec_ctor_fn!(make_cpp_var_int_gb, CppVarIntGb, CppVarIntGb);

/// C++ codecs (any-length; block codecs are already composites in the C++ library).
pub static CPP: &[CodecEntry] = &[
    ("CppBP32", make_cpp_bp32),
    ("CppCopy", make_cpp_copy),
    ("CppFastBinaryPacking8", make_cpp_fast_binary_packing8),
    ("CppFastPFor128", make_cpp_fastpfor128),
    ("CppFastPFor256", make_cpp_fastpfor256),
    ("CppFastBinaryPacking16", make_cpp_fast_binary_packing16),
    ("CppFastBinaryPacking32", make_cpp_fast_binary_packing32),
    ("CppMaskedVByte", make_cpp_masked_vbyte),
    ("CppNewPFor", make_cpp_new_pfor),
    ("CppOptPFor", make_cpp_opt_pfor),
    ("CppPFor2008", make_cpp_pfor2008),
    ("CppPFor", make_cpp_pfor),
    ("CppSimdBinaryPacking", make_cpp_simd_binary_packing),
    ("CppSimdFastPFor128", make_cpp_simd_fastpfor128),
    ("CppSimdFastPFor256", make_cpp_simd_fastpfor256),
    ("CppSimdGroupSimple", make_cpp_simd_group_simple),
    (
        "CppSimdGroupSimpleRingBuf",
        make_cpp_simd_group_simple_ring_buf,
    ),
    ("CppSimdNewPFor", make_cpp_simd_new_pfor),
    ("CppSimdOptPFor", make_cpp_simd_opt_pfor),
    ("CppSimdPFor", make_cpp_simd_pfor),
    ("CppSimdSimplePFor", make_cpp_simd_simple_pfor),
    ("CppStreamVByte", make_cpp_stream_vbyte),
    ("CppVByte", make_cpp_vbyte),
    ("CppVarInt", make_cpp_var_int),
    ("CppVarIntGb", make_cpp_var_int_gb),
];

// ── Codec selector (Arbitrary) ─────────────────────────────────────────────────

/// Selects a codec by index. `idx` is wrapped modulo the list length.
/// `use_cpp` switches between [`RUST`] and [`CPP`].
#[derive(arbitrary::Arbitrary, Clone, Copy, Debug)]
pub struct AnyLenSelector {
    pub idx: u8,
    pub use_cpp: bool,
}

/// Instantiate a codec, returning `(name, codec)`.
pub fn instantiate_anylen_codec(sel: AnyLenSelector) -> (&'static str, FuzzAnyLen) {
    let list = if sel.use_cpp { CPP } else { RUST };
    let (name, make) = list[sel.idx as usize % list.len()];
    (name, make())
}

// ── Encode compare: Rust vs C++ bit-identical pairs ────────────────────────────

/// A pair of codecs that should produce bit-identical compressed output.
/// `rust` = Rust implementation, `cpp` = C++ implementation
#[derive(Clone, Copy)]
pub struct CodecPair {
    pub name: &'static str,
    pub make_rust: fn() -> FuzzAnyLen,
    pub make_cpp: fn() -> FuzzAnyLen,
}

fn pair_rust_fastpfor128() -> FuzzAnyLen {
    FuzzAnyLen::FastPFor128(FastPFor128::default())
}
fn pair_cpp_fastpfor128() -> FuzzAnyLen {
    FuzzAnyLen::CppFastPFor128(CppFastPFor128::default())
}
fn pair_rust_fastpfor256() -> FuzzAnyLen {
    FuzzAnyLen::FastPFor256(FastPFor256::default())
}
fn pair_cpp_fastpfor256() -> FuzzAnyLen {
    FuzzAnyLen::CppFastPFor256(CppFastPFor256::default())
}
fn pair_rust_variable_byte() -> FuzzAnyLen {
    FuzzAnyLen::VariableByte(VariableByte)
}
fn pair_cpp_var_int() -> FuzzAnyLen {
    FuzzAnyLen::CppVarInt(CppVarInt::default())
}
fn pair_rust_just_copy() -> FuzzAnyLen {
    FuzzAnyLen::JustCopy(JustCopy)
}
fn pair_cpp_copy() -> FuzzAnyLen {
    FuzzAnyLen::CppCopy(CppCopy::default())
}

/// Pairs of Rust and C++ codecs expected to produce bit-identical output.
pub static ENCODE_COMPARE_PAIRS: &[CodecPair] = &[
    CodecPair {
        name: "FastPFor128",
        make_rust: pair_rust_fastpfor128,
        make_cpp: pair_cpp_fastpfor128,
    },
    CodecPair {
        name: "FastPFor256",
        make_rust: pair_rust_fastpfor256,
        make_cpp: pair_cpp_fastpfor256,
    },
    CodecPair {
        name: "VariableByte",
        make_rust: pair_rust_variable_byte,
        make_cpp: pair_cpp_var_int,
    },
    CodecPair {
        name: "JustCopy",
        make_rust: pair_rust_just_copy,
        make_cpp: pair_cpp_copy,
    },
];

/// Optional pair filter: if set, only the named pair is tested.
/// Set `FUZZ_PAIR=FastPFor128` or `FUZZ_ENCODE_COMPARE_PAIR=FastPFor128` to restrict.
pub fn encode_compare_pair_filter() -> Option<String> {
    std::env::var("FUZZ_PAIR")
        .ok()
        .or_else(|| std::env::var("FUZZ_ENCODE_COMPARE_PAIR").ok())
}

/// Resolve a pair index to a `CodecPair`, applying filter and alternative substitution.
pub fn resolve_encode_compare_pair(idx: u8) -> Option<CodecPair> {
    let filter = encode_compare_pair_filter();
    let pairs = ENCODE_COMPARE_PAIRS;
    let i = idx as usize % pairs.len();
    let pair = pairs[i];
    if let Some(ref f) = filter
        && !f.eq_ignore_ascii_case(pair.name)
    {
        return None;
    }
    Some(pair)
}

/// Instantiate both codecs for a pair, using the alternative C++ when requested.
pub fn instantiate_pair(pair: CodecPair) -> (FuzzAnyLen, FuzzAnyLen) {
    let rust_codec = (pair.make_rust)();
    let cpp_codec = (pair.make_cpp)();
    (rust_codec, cpp_codec)
}
