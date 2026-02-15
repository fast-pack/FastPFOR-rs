use fastpfor::cpp;
use fastpfor::rust;

pub type BoxedCppCodec = Box<dyn cpp::Codec32>;

#[derive(arbitrary::Arbitrary)]
pub struct FuzzInput<C> {
    pub data: Vec<u32>,
    pub codec: C,
}

impl<C: std::fmt::Debug> std::fmt::Debug for FuzzInput<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FuzzInput")
            .field("codec", &self.codec)
            .field("data", &HexSlice(&self.data))
            .finish()
    }
}

#[derive(arbitrary::Arbitrary, Clone, Copy, PartialEq, Eq, Debug)]
pub enum RustCodec {
    FastPFOR256,
    FastPFOR128,
    VariableByte,
    JustCopy,
}

impl From<RustCodec> for rust::Codec {
    fn from(codec: RustCodec) -> Self {
        use rust::*;
        match codec {
            RustCodec::FastPFOR256 => Codec::from(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_256)),
            RustCodec::FastPFOR128 => Codec::from(FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128)),
            RustCodec::VariableByte => Codec::from(VariableByte::new()),
            RustCodec::JustCopy => Codec::from(JustCopy::new()),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, arbitrary::Arbitrary, Debug)]
pub enum CppCodec {
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

impl From<CppCodec> for BoxedCppCodec {
    fn from(codec: CppCodec) -> Self {
        use cpp::*;
        match codec {
            CppCodec::BP32 => Box::new(BP32Codec::default()),
            CppCodec::Copy => Box::new(CopyCodec::default()),
            CppCodec::FastBinaryPacking8 => Box::new(FastBinaryPacking8Codec::default()),
            CppCodec::FastPFor128 => Box::new(FastPFor128Codec::default()),
            CppCodec::FastPFor256 => Box::new(FastPFor256Codec::default()),
            CppCodec::FastBinaryPacking16 => Box::new(FastBinaryPacking16Codec::default()),
            CppCodec::FastBinaryPacking32 => Box::new(FastBinaryPacking32Codec::default()),
            CppCodec::MaskedVByte => Box::new(MaskedVByteCodec::default()),
            CppCodec::NewPFor => Box::new(NewPForCodec::default()),
            CppCodec::OptPFor => Box::new(OptPForCodec::default()),
            CppCodec::PFor2008 => Box::new(PFor2008Codec::default()),
            CppCodec::PFor => Box::new(PForCodec::default()),
            CppCodec::SimdBinaryPacking => Box::new(SimdBinaryPackingCodec::default()),
            CppCodec::SimdFastPFor128 => Box::new(SimdFastPFor128Codec::default()),
            CppCodec::SimdFastPFor256 => Box::new(SimdFastPFor256Codec::default()),
            CppCodec::SimdGroupSimple => Box::new(SimdGroupSimpleCodec::default()),
            CppCodec::SimdGroupSimpleRingBuf => Box::new(SimdGroupSimpleRingBufCodec::default()),
            CppCodec::SimdNewPFor => Box::new(SimdNewPForCodec::default()),
            CppCodec::SimdOptPFor => Box::new(SimdOptPForCodec::default()),
            CppCodec::SimdPFor => Box::new(SimdPForCodec::default()),
            CppCodec::SimdSimplePFor => Box::new(SimdSimplePForCodec::default()),
            // CppCodec::Simple16 => Box::new(Simple16Codec::default()),
            // CppCodec::Simple8b => Box::new(Simple8bCodec::default()),
            // CppCodec::Simple8bRle => Box::new(Simple8bRleCodec::default()),
            // CppCodec::Simple9 => Box::new(Simple9Codec::default()),
            // CppCodec::Simple9Rle => Box::new(Simple9RleCodec::default()),
            // CppCodec::SimplePFor => Box::new(SimplePForCodec::default()),
            // CppCodec::Snappy => Box::new(SnappyCodec::default()),
            CppCodec::StreamVByte => Box::new(StreamVByteCodec::default()),
            CppCodec::VByte => Box::new(VByteCodec::default()),
            CppCodec::VarInt => Box::new(VarIntCodec::default()),
            // CppCodec::VarIntG8iu => Box::new(VarIntG8iuCodec::default()),
            CppCodec::VarIntGb => Box::new(VarIntGbCodec::default()),
            // CppCodec::VsEncoding => Box::new(VsEncodingCodec::default()),
        }
    }
}

pub struct HexSlice<'a>(pub &'a [u32]);

impl<'a> std::fmt::Debug for HexSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MAX: usize = 20;

        let total = self.0.len();
        let shown = total.min(MAX);

        let mut list = f.debug_list();

        for v in &self.0[..shown] {
            list.entry(&format_args!("{:#010x}", v));
        }

        if total > MAX {
            list.entry(&format_args!(".. out of {} total", total));
        }

        list.finish()
    }
}
