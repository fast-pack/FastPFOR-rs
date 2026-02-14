use crate::rust::{FastPFOR, JustCopy, VariableByte};

/// Type-erased wrapper for compression codecs.
///
/// Allows different codec types to be used interchangeably through a unified interface.
pub enum Codec {
    /// [`FastPFOR`] compression codec
    FastPFor(Box<FastPFOR>),
    /// [`VariableByte`] compression codec
    VariableByte(VariableByte),
    /// Pass-through codec (no compression)
    JustCopy(JustCopy),
}

impl From<FastPFOR> for Codec {
    fn from(fastpfor: FastPFOR) -> Self {
        Codec::FastPFor(Box::new(fastpfor))
    }
}

impl From<VariableByte> for Codec {
    fn from(vb: VariableByte) -> Self {
        Codec::VariableByte(vb)
    }
}

impl From<JustCopy> for Codec {
    fn from(jc: JustCopy) -> Self {
        Codec::JustCopy(jc)
    }
}
