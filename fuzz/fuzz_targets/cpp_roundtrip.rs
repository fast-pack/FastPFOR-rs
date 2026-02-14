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
    let dec_slice = codec.decode32(enc_slice, &mut decoded).unwrap();

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
});

#[derive(arbitrary::Arbitrary)]
struct FuzzInput {
    data: Vec<u32>,
    codec: FuzzCodec,
}

impl std::fmt::Debug for FuzzInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FuzzInput")
            .field("data_length", &self.data.len())
            .field("codec", &self.codec)
            .finish()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, arbitrary::Arbitrary, Debug)]
enum FuzzCodec {
    BP32,
    Copy,
    FastBinaryPacking8,
    FastPFor128,
    FastPFor256,
    FastBinaryPacking16,
    FastBinaryPacking32,
    MaskedVByte,
    NewPFor,
    OptPFor,
    PFor2008,
    PFor,
    SimdBinaryPacking,
    SimdFastPFor128,
    SimdFastPFor256,
    SimdGroupSimple,
    SimdGroupSimpleRingBuf,
    SimdNewPFor,
    SimdOptPFor,
    SimdPFor,
    SimdSimplePFor,
    // Simple16, // cannot encode arbitrary bytes
    // Simple8b, // cannot encode arbitrary bytes
    // Simple8bRle, // cannot encode arbitrary bytes
    // Simple9, // cannot encode arbitrary bytes
    // Simple9Rle, // cannot encode arbitrary bytes
    // SimplePFor, // cannot encode arbitrary bytes
    // Snappy,  // Conditional with #ifdef
    StreamVByte,
    VByte,
    VarInt,
    // VarIntG8iu,  // Conditional with #ifdef
    VarIntGb,
    // VsEncoding,  // This is leaking memory
}

impl From<FuzzCodec> for BoxedCodec {
    fn from(codec: FuzzCodec) -> Self {
        match codec {
            FuzzCodec::BP32 => Box::new(BP32Codec::default()),
            FuzzCodec::Copy => Box::new(CopyCodec::default()),
            FuzzCodec::FastBinaryPacking8 => Box::new(FastBinaryPacking8Codec::default()),
            FuzzCodec::FastPFor128 => Box::new(FastPFor128Codec::default()),
            FuzzCodec::FastPFor256 => Box::new(FastPFor256Codec::default()),
            FuzzCodec::FastBinaryPacking16 => Box::new(FastBinaryPacking16Codec::default()),
            FuzzCodec::FastBinaryPacking32 => Box::new(FastBinaryPacking32Codec::default()),
            FuzzCodec::MaskedVByte => Box::new(MaskedVByteCodec::default()),
            FuzzCodec::NewPFor => Box::new(NewPForCodec::default()),
            FuzzCodec::OptPFor => Box::new(OptPForCodec::default()),
            FuzzCodec::PFor2008 => Box::new(PFor2008Codec::default()),
            FuzzCodec::PFor => Box::new(PForCodec::default()),
            FuzzCodec::SimdBinaryPacking => Box::new(SimdBinaryPackingCodec::default()),
            FuzzCodec::SimdFastPFor128 => Box::new(SimdFastPFor128Codec::default()),
            FuzzCodec::SimdFastPFor256 => Box::new(SimdFastPFor256Codec::default()),
            FuzzCodec::SimdGroupSimple => Box::new(SimdGroupSimpleCodec::default()),
            FuzzCodec::SimdGroupSimpleRingBuf => {
                Box::new(SimdGroupSimpleRingBufCodec::default())
            }
            FuzzCodec::SimdNewPFor => Box::new(SimdNewPForCodec::default()),
            FuzzCodec::SimdOptPFor => Box::new(SimdOptPForCodec::default()),
            FuzzCodec::SimdPFor => Box::new(SimdPForCodec::default()),
            FuzzCodec::SimdSimplePFor => Box::new(SimdSimplePForCodec::default()),
            // FuzzCodec::Simple16 => Box::new(Simple16Codec::default()),
            // FuzzCodec::Simple8b => Box::new(Simple8bCodec::default()),
            // FuzzCodec::Simple8bRle => Box::new(Simple8bRleCodec::default()),
            // FuzzCodec::Simple9 => Box::new(Simple9Codec::default()),
            // FuzzCodec::Simple9Rle => Box::new(Simple9RleCodec::default()),
            // FuzzCodec::SimplePFor => Box::new(SimplePForCodec::default()),
            // FuzzCodec::Snappy => Box::new(SnappyCodec::default()),
            FuzzCodec::StreamVByte => Box::new(StreamVByteCodec::default()),
            FuzzCodec::VByte => Box::new(VByteCodec::default()),
            FuzzCodec::VarInt => Box::new(VarIntCodec::default()),
            // FuzzCodec::VarIntG8iu => Box::new(VarIntG8iuCodec::default()),
            FuzzCodec::VarIntGb => Box::new(VarIntGbCodec::default()),
            // FuzzCodec::VsEncoding => Box::new(VsEncodingCodec::default()),
        }
    }
}
