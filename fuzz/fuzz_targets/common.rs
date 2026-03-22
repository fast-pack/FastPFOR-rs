use fastpfor::{AnyLenCodec, cpp, rust};

pub type BoxedCppCodec = Box<dyn AnyLenCodec>;

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
        match codec {
            CppCodec::BP32 => Box::new(cpp::BP32Codec::default()),
            CppCodec::Copy => Box::new(cpp::CopyCodec::default()),
            CppCodec::FastBinaryPacking8 => Box::new(cpp::FastBinaryPacking8Codec::default()),
            CppCodec::FastPFor128 => Box::new(cpp::FastPFor128Codec::default()),
            CppCodec::FastPFor256 => Box::new(cpp::FastPFor256Codec::default()),
            CppCodec::FastBinaryPacking16 => Box::new(cpp::FastBinaryPacking16Codec::default()),
            CppCodec::FastBinaryPacking32 => Box::new(cpp::FastBinaryPacking32Codec::default()),
            CppCodec::MaskedVByte => Box::new(cpp::MaskedVByteCodec::default()),
            CppCodec::NewPFor => Box::new(cpp::NewPForCodec::default()),
            CppCodec::OptPFor => Box::new(cpp::OptPForCodec::default()),
            CppCodec::PFor2008 => Box::new(cpp::PFor2008Codec::default()),
            CppCodec::PFor => Box::new(cpp::PForCodec::default()),
            CppCodec::SimdBinaryPacking => Box::new(cpp::SimdBinaryPackingCodec::default()),
            CppCodec::SimdFastPFor128 => Box::new(cpp::SimdFastPFor128Codec::default()),
            CppCodec::SimdFastPFor256 => Box::new(cpp::SimdFastPFor256Codec::default()),
            CppCodec::SimdGroupSimple => Box::new(cpp::SimdGroupSimpleCodec::default()),
            CppCodec::SimdGroupSimpleRingBuf => {
                Box::new(cpp::SimdGroupSimpleRingBufCodec::default())
            }
            CppCodec::SimdNewPFor => Box::new(cpp::SimdNewPForCodec::default()),
            CppCodec::SimdOptPFor => Box::new(cpp::SimdOptPForCodec::default()),
            CppCodec::SimdPFor => Box::new(cpp::SimdPForCodec::default()),
            CppCodec::SimdSimplePFor => Box::new(cpp::SimdSimplePForCodec::default()),
            // CppCodec::Simple16 => Box::new(cpp::Simple16Codec::default()),
            // CppCodec::Simple8b => Box::new(cpp::Simple8bCodec::default()),
            // CppCodec::Simple8bRle => Box::new(cpp::Simple8bRleCodec::default()),
            // CppCodec::Simple9 => Box::new(cpp::Simple9Codec::default()),
            // CppCodec::Simple9Rle => Box::new(cpp::Simple9RleCodec::default()),
            // CppCodec::SimplePFor => Box::new(cpp::SimplePForCodec::default()),
            // CppCodec::Snappy => Box::new(cpp::SnappyCodec::default()),
            CppCodec::StreamVByte => Box::new(cpp::StreamVByteCodec::default()),
            CppCodec::VByte => Box::new(cpp::VByteCodec::default()),
            CppCodec::VarInt => Box::new(cpp::VarIntCodec::default()),
            // CppCodec::VarIntG8iu => Box::new(cpp::VarIntG8iuCodec::default()),
            CppCodec::VarIntGb => Box::new(cpp::VarIntGbCodec::default()),
            // CppCodec::VsEncoding => Box::new(cpp::VsEncodingCodec::default()),
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
