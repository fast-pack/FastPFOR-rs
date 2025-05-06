#![cfg(all(feature = "rust", feature = "cpp"))]

use rand::rngs::StdRng;
use rand::Rng as _;
use rand::SeedableRng as _;

pub fn get_test_cases(n: usize) -> Vec<Vec<u32>> {
    let mut rng = StdRng::seed_from_u64(14);

    vec![
        // Zeroes
        vec![0u32; n],
        // Same non-zero
        vec![14u32; n],
        // Ascending values
        (0..n).map(|i| i as u32).collect::<Vec<u32>>(),
        // Descending values
        (0..n).rev().map(|i| i as u32).collect::<Vec<u32>>(),
        // Bit-flipping pattern
        (0..n)
            .map(|i| ((i as u32) * 32) ^ ((i as u32) >> 1))
            .collect::<Vec<u32>>(),
        // Alternating large and small values
        (0..n)
            .map(|i| {
                let ui = i as u32;
                if ui % 2 == 0 {
                    1 << 30
                } else {
                    3
                }
            })
            .collect::<Vec<u32>>(),
        // Random u32 values
        (0..n)
            .map(|_| rng.random_range(0..(1 << 31)))
            .collect::<Vec<u32>>(),
        // Spike in the middle
        (0..n)
            .map(|i| if i == n / 2 { u32::MAX } else { 1 })
            .collect::<Vec<u32>>(),
        // An empty vector
        Vec::new(),
    ]
}
