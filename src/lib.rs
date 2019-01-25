//! **hilbert_curve** is a simple implementation of
//! [Hilbert curve](https://en.wikipedia.org/wiki/Hilbert_curve) mapping algorithms in Rust.
//!
//! It assumes a square space divided into n by n cells (n being a power of 2) with integral
//! coordinates: (0, 0) in the lower left corner, (n − 1, n − 1) in the upper right corner, and a
//! distance d that starts at 0 in the lower left corner and goes to n^2 − 1 in the lower-right
//! corner.

use std::mem;

/// Convert a one-dimensional distance `d` to a pair of (x, y) coordinates.
pub fn convert_1d_to_2d(d: usize, n: usize) -> (usize, usize) {
    assert!((n & (n - 1)) == 0, "n must be a power of 2");
    let mut s = 1;
    let mut t = d;
    let (mut x, mut y) = (0, 0);
    let (mut rx, mut ry);

    while s < n {
        rx = 1 & (t / 2);
        ry = 1 & (t ^ rx);
        rotate(s, &mut x, &mut y, rx, ry);
        x += s * rx;
        y += s * ry;
        t /= 4;
        s *= 2;
    }

    (x, y)
}

/// Convert a pair of (x, y) coordinates to a one-dimensional distance.
pub fn convert_2d_to_1d (x: usize, y: usize, n: usize) -> usize {
    assert!((n & (n - 1)) == 0, "n must be a power of 2");
    let mut d = 0;
    let mut s = n / 2;
    let (mut x, mut y) = (x, y);
    let (mut rx, mut ry);

    while s > 0 {
        rx = if (x & s) > 0 { 1 } else { 0 };
        ry = if (y & s) > 0 { 1 } else { 0 };
        d += s * s * ((3 * rx) ^ ry);
        rotate(s, &mut x, &mut y, rx, ry);
        s /= 2
    }

    d
}

// Rotate a quadrant
fn rotate(n: usize, x: &mut usize, y: &mut usize, rx: usize, ry: usize) {
    if ry == 0 {
        if rx == 1 {
            *x = n.wrapping_sub(1).wrapping_sub(*x);
            *y = n.wrapping_sub(1).wrapping_sub(*y);
        }

        mem::swap(x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversibility() {
        for &n in &[2, 4, 8, 16, 32, 64, 128, 256] {
            for d in 0..(n * n) {
                let (x, y) = convert_1d_to_2d(d, n);
                assert_eq!(convert_2d_to_1d(x, y, n), d);
            }
        }
    }
}
