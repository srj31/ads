use std::cmp::max;

use rand::Rng;

/// Counting the number of elements in a collection with less than O(logn) space
/// Input: Stream of data
/// Initialization: X <- 0
/// Transition: X <- X + 1 with the probability of 2^-X
/// Output: n' = 2^X - 1, which is the estimate of count
pub fn morris_counting(iter: impl Iterator) -> i32 {
    let mut x_n = 0;
    let p = 1.0;
    let mut r_gen = rand::thread_rng();
    for _ in iter {
        match r_gen.gen_bool(p) {
            true => x_n += 1,
            _ => {}
        }
    }

    x_n
}

pub fn estimate_cardinality(iter: impl Iterator<Item = u64>, k: u32) -> f32 {
    let bucket_size: u32 = 1 << k;
    let mut bucket = vec![0; bucket_size as usize];

    for i in iter {
        let h = hash(i);
        let bucket_id = h & ((bucket_size - 1) as u64);
        let bucket_hash = h >> k;
        bucket[bucket_id as usize] =
            max(bucket[bucket_id as usize], trailing_zeroes(bucket_hash));
    }

    let sum = bucket.iter().fold(0, |acc, x| acc + x);
    (1 << (sum / bucket_size)) as f32 * bucket_size as f32 * 0.79402
}

fn trailing_zeroes(n: u64) -> u32 {
    n.trailing_zeros()
}

fn hash(val: u64) -> u64 {
    val
}
