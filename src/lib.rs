// Copyright (c) 2019 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The hash function used in BSA files for *The Elder Scrolls III: Morrowind*.

#![no_std]
#![forbid(
    warnings,
    future_incompatible,
    rust_2018_idioms,
    rustdoc,
    unused,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_results,
    clippy::all
)]

#[inline]
/// Computes the left and right hashes of a given byte sequence.
///
/// The return value is a tuple of two `u32`s: the hash of the left half, and the hash of the right
/// half.
pub fn calculate<T: AsRef<[u8]>>(input: T) -> (u32, u32) {
    const MASK: u32 = 0b1_1111;
    let bytes = input.as_ref();
    let (left, right) = bytes.split_at(bytes.len() >> 1);
    let (mut a, mut b, mut off) = (0, 0, 0);
    for &n in left {
        let x = u32::from(n);
        let shift = off & MASK;
        a ^= x << shift;
        off = off.wrapping_add(8);
    }
    off = 0;
    for &n in right {
        let y = u32::from(n);
        let shift = off & MASK;
        let temp = y << shift;
        b = (b ^ temp).rotate_right(temp & MASK);
        off = off.wrapping_add(8);
    }
    (a, b)
}
