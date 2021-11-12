// Copyright (c) 2019, 2021 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The hash function used in BSA files for *The Elder Scrolls III: Morrowind*.

#![no_std]
#![deny(
    warnings,
    future_incompatible,
    unused,
    unsafe_code,
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    rustdoc::all
)]
#![allow(clippy::cast_lossless, clippy::must_use_candidate)]

#[inline]
/// Computes the hash of a given byte sequence, expressed as a 64-bit integer.
pub fn calculate(input: &[u8]) -> u64 {
    let (left, right) = calculate_tuple(input);
    (right as u64) << 32 | left as u64
}

#[inline]
/// Computes the hash of a given byte sequence, expressed as a tuple of two 32-bit integers.
pub fn calculate_tuple(input: &[u8]) -> (u32, u32) {
    const MASK: u32 = 0b1_1111;
    let (left, right) = input.split_at(input.len() >> 1);
    let (mut a, mut b, mut shift) = (0, 0, 0);
    for &n in left {
        a ^= u32::from(n) << shift;
        shift = (shift + 8) & MASK;
    }
    shift = 0;
    for &n in right {
        let temp = u32::from(n) << shift;
        b = (b ^ temp).rotate_right(temp & MASK);
        shift = (shift + 8) & MASK;
    }
    (a, b)
}

#[cfg(test)]
mod tests {
    include!("../bsa_lists/morrowind.rs");
    include!("../bsa_lists/tribunal.rs");
    include!("../bsa_lists/bloodmoon.rs");

    #[inline]
    fn test_hashes(list: &[(&str, u32, u32)]) {
        for &(filename, left_hash, right_hash) in list {
            let number = super::calculate(filename.as_bytes());
            let tuple = super::calculate_tuple(filename.as_bytes());
            assert_eq!(number, (tuple.1 as u64) << 32 | tuple.0 as u64);
            assert_eq!(tuple, (left_hash, right_hash));
        }
    }

    #[test]
    fn morrowind_bsa() {
        test_hashes(MORROWIND_BSA);
    }

    #[test]
    fn tribunal_bsa() {
        test_hashes(TRIBUNAL_BSA);
    }

    #[test]
    fn bloodmoon_bsa() {
        test_hashes(BLOODMOON_BSA);
    }
}
