#![no_main]

use fastpfor::cpp::*;
use libfuzzer_sys::fuzz_target;

type BoxedCodec = Box<dyn Codec32>;

fuzz_target!(|data: FuzzInput| {
    let codec = BoxedCodec::from(data.codec);
    let input = data.data;

    // Allocate output buffer with generous size
    let mut output = vec![0u32; input.len() * 2 + 1024];

    // Compress the data
    let enc_slice = codec.encode32(&input, &mut output).unwrap();

    // Now decompress
    let mut decoded = vec![0u32; input.len() * 2 + 1024];
    let dec_slice = codec.decode32(&enc_slice, &mut decoded).unwrap();

    // Verify roundtrip
    if dec_slice.len() + input.len() < 200 {
        assert_eq!(input, dec_slice, "Decompressed output mismatches");
    } else {
        assert_eq!(dec_slice.len(), input.len(), "Decompressed length mismatch");
        for (i, (&original, &decoded)) in input.iter().zip(dec_slice.iter()).enumerate() {
            assert_eq!(
                original, decoded,
                "Mismatch at position {i}: expected {original}, got {decoded}"
            );
        }
    }
    assert_eq!(dec_slice, input.len());
});

#[derive(arbitrary::Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u32>,
    codec: FuzzCodec,
}

#[derive(Clone, Copy, Eq, PartialEq, arbitrary::Arbitrary, Debug)]
enum FuzzCodec {
    BP32Codec,
    CopyCodec,
    FastBinaryPacking8Codec,
    FastPFor128Codec,
    FastPFor256Codec,
    FastBinaryPacking16Codec,
    FastBinaryPacking32Codec,
    MaskedVByteCodec,
    NewPForCodec,
    OptPForCodec,
    PFor2008Codec,
    PForCodec,
    SimdBinaryPackingCodec,
    SimdFastPFor128Codec,
    SimdFastPFor256Codec,
    SimdGroupSimpleCodec,
    SimdGroupSimpleRingBufCodec,
    SimdNewPForCodec,
    SimdOptPForCodec,
    SimdPForCodec,
    SimdSimplePForCodec,
    // Simple16Codec, // cannot encode arbitrary bytes
    // Simple8bCodec, // cannot encode arbitrary bytes
    // Simple8bRleCodec, // cannot encode arbitrary bytes
    // Simple9Codec, // cannot encode arbitrary bytes
    // Simple9RleCodec, // cannot encode arbitrary bytes
    // SimplePForCodec, // cannot encode arbitrary bytes
    // SnappyCodec,  // Conditional with #ifdef
    StreamVByteCodec,
    VByteCodec,
    VarIntCodec,
    // VarIntG8iuCodec,  // Conditional with #ifdef
    VarIntGbCodec,
    // VsEncodingCodec,  // This is leaking memory
}

impl From<FuzzCodec> for BoxedCodec {
    fn from(codec: FuzzCodec) -> Self {
        match codec {
            FuzzCodec::BP32Codec => Box::new(BP32Codec::default()),
            FuzzCodec::CopyCodec => Box::new(CopyCodec::default()),
            FuzzCodec::FastBinaryPacking8Codec => Box::new(FastBinaryPacking8Codec::default()),
            FuzzCodec::FastPFor128Codec => Box::new(FastPFor128Codec::default()),
            FuzzCodec::FastPFor256Codec => Box::new(FastPFor256Codec::default()),
            FuzzCodec::FastBinaryPacking16Codec => Box::new(FastBinaryPacking16Codec::default()),
            FuzzCodec::FastBinaryPacking32Codec => Box::new(FastBinaryPacking32Codec::default()),
            FuzzCodec::MaskedVByteCodec => Box::new(MaskedVByteCodec::default()),
            FuzzCodec::NewPForCodec => Box::new(NewPForCodec::default()),
            FuzzCodec::OptPForCodec => Box::new(OptPForCodec::default()),
            FuzzCodec::PFor2008Codec => Box::new(PFor2008Codec::default()),
            FuzzCodec::PForCodec => Box::new(PForCodec::default()),
            FuzzCodec::SimdBinaryPackingCodec => Box::new(SimdBinaryPackingCodec::default()),
            FuzzCodec::SimdFastPFor128Codec => Box::new(SimdFastPFor128Codec::default()),
            FuzzCodec::SimdFastPFor256Codec => Box::new(SimdFastPFor256Codec::default()),
            FuzzCodec::SimdGroupSimpleCodec => Box::new(SimdGroupSimpleCodec::default()),
            FuzzCodec::SimdGroupSimpleRingBufCodec => {
                Box::new(SimdGroupSimpleRingBufCodec::default())
            }
            FuzzCodec::SimdNewPForCodec => Box::new(SimdNewPForCodec::default()),
            FuzzCodec::SimdOptPForCodec => Box::new(SimdOptPForCodec::default()),
            FuzzCodec::SimdPForCodec => Box::new(SimdPForCodec::default()),
            FuzzCodec::SimdSimplePForCodec => Box::new(SimdSimplePForCodec::default()),
            // FuzzCodec::Simple16Codec => Box::new(Simple16Codec::default()),
            // FuzzCodec::Simple8bCodec => Box::new(Simple8bCodec::default()),
            // FuzzCodec::Simple8bRleCodec => Box::new(Simple8bRleCodec::default()),
            // FuzzCodec::Simple9Codec => Box::new(Simple9Codec::default()),
            // FuzzCodec::Simple9RleCodec => Box::new(Simple9RleCodec::default()),
            // FuzzCodec::SimplePForCodec => Box::new(SimplePForCodec::default()),
            // FuzzCodec::SnappyCodec => Box::new(SnappyCodec::default()),
            FuzzCodec::StreamVByteCodec => Box::new(StreamVByteCodec::default()),
            FuzzCodec::VByteCodec => Box::new(VByteCodec::default()),
            FuzzCodec::VarIntCodec => Box::new(VarIntCodec::default()),
            // FuzzCodec::VarIntG8iuCodec => Box::new(VarIntG8iuCodec::default()),
            FuzzCodec::VarIntGbCodec => Box::new(VarIntGbCodec::default()),
            // FuzzCodec::VsEncodingCodec => Box::new(VsEncodingCodec::default()),
        }
    }
}
