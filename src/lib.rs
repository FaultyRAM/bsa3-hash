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

#[inline]
/// Splits a given byte sequence in two and computes the hash for each half.
///
/// The return value is a tuple of two `u32`s: the hash of the left half, and the hash of the right
/// half, in that order.
pub fn calculate<T: AsRef<[u8]>>(input: T) -> (u32, u32) {
    const MASK: u32 = 0b1_1111;
    let bytes = input.as_ref();
    let (left, right) = bytes.split_at(bytes.len() >> 1);
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
            assert_eq!(super::calculate(filename), (left_hash, right_hash));
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
