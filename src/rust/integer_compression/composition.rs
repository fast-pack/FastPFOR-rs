use std::io::Cursor;

use crate::rust::cursor::IncrementCursor;
use crate::rust::{Codec, FastPForResult, Integer};

/// Chains two codecs together, applying them sequentially.
///
/// Compresses data first with `c1`, then with `c2` on the remaining data.
pub struct Composition {
    c1: Codec,
    c2: Codec,
}

impl Composition {
    /// Creates a new instance of the composition codec.
    pub fn new<C1, C2>(c1: C1, c2: C2) -> Self
    where
        C1: Into<Codec>,
        C2: Into<Codec>,
    {
        Composition {
            c1: c1.into(),
            c2: c2.into(),
        }
    }
}

impl Integer<u32> for Composition {
    fn compress(
        &mut self,
        input: &[u32],
        mut input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let inpos_init = input_offset.position();
        let outpos_init = output_offset.position();
        self.c1
            .compress(input, input_length, input_offset, output, output_offset)?;
        if output_offset.position() == outpos_init {
            output[outpos_init as usize] = 0;
            output_offset.increment();
        }
        input_length -= input_offset.position() as u32 - inpos_init as u32;
        self.c2
            .compress(input, input_length, input_offset, output, output_offset)
    }

    fn uncompress(
        &mut self,
        input: &[u32],
        mut input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let final_init = input_offset.position() as u32;
        self.c1
            .uncompress(input, input_length, input_offset, output, output_offset)?;
        input_length -= input_offset.position() as u32 - final_init;
        self.c2
            .uncompress(input, input_length, input_offset, output, output_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::integer_compression::fastpfor::FastPFOR;
    use crate::rust::integer_compression::variable_byte::VariableByte;

    #[test]
    fn test_composition() {
        let fastpfor = FastPFOR::default();
        let vb = VariableByte::new();
        let mut comp = Composition::new(fastpfor, vb);
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut output = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);
        let input_length = 10;
        comp.compress(
            &input,
            input_length,
            &mut input_offset,
            &mut output,
            &mut output_offset,
        )
        .expect("Failed to compress");
    }
}
