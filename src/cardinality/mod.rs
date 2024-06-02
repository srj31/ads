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
