use crate::rust::{FastPFOR, JustCopy, VariableByte};

pub enum Codec {
    FastPFor(FastPFOR),
    VariableByte(VariableByte),
    JustCopy(JustCopy),
}

impl From<FastPFOR> for Codec {
    fn from(fastpfor: FastPFOR) -> Self {
        Codec::FastPFor(fastpfor)
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
