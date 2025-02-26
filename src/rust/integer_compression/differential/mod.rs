use std::ops::{Add, AddAssign};

pub struct Delta;

impl Delta {
    pub fn new() -> Delta {
        Delta
    }

    // C++ port as it supports any type of numeric data
    // source: https://github.com/fast-pack/FastPFor/blob/49d44d94773518ef26486f7a58f8da08e8a498bb/headers/deltautil.h#L274
    pub fn fast_inverse_delta<T>(data: &mut [T])
    where
        T: Copy + Add<Output = T> + AddAssign,
    {
        if data.is_empty() {
            return;
        }

        let sz0 = (data.len() / 4) * 4;
        let mut i = 1;

        if sz0 >= 4 {
            let mut a = data[0];
            while i < sz0 - 4 {
                a = {
                    data[i] += a;
                    data[i]
                };
                a = {
                    data[i + 1] += a;
                    data[i + 1]
                };
                a = {
                    data[i + 2] += a;
                    data[i + 2]
                };
                a = {
                    data[i + 3] += a;
                    data[i + 3]
                };
                i += 4;
            }
        }

        while i < data.len() {
            data[i] += data[i - 1];
            i += 1;
        }
    }
}

impl Default for Delta {
    fn default() -> Self {
        Delta::new()
    }
}
