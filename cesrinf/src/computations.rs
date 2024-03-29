// Allow not using these functions for now.
#![allow(dead_code)]

/// `n` is the length of the raw binary value
pub fn compute_pad_size(n: usize) -> usize {
    (3 - (n % 3)) % 3
}

/// `n` is the length of the raw binary value
pub fn compute_minimum_code_size_scaling_factor(pad_size: usize) -> usize {
    match pad_size {
        0 => 1,
        _ => 0,
    }
}

/// https://trustoverip.github.io/tswg-cesr-specification/#example-of-pad-size-computation
pub fn compute_code_size(pad_size: usize) -> usize {
    let m = compute_minimum_code_size_scaling_factor(pad_size);
    if pad_size > 2 {
        panic!("pad_size should be 0, 1, 2, but was {}", pad_size);
    }

    4 * m + pad_size
}
