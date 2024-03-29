// Copyright (c) 2019, 2021 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The hash function used in BSA files for *The Elder Scrolls III: Morrowind*.
//!
//! # Example
//!
//! ```no_run
//! assert_eq!(
//!     bsa3_hash::calculate(r"meshes\m\probe_journeyman_01.nif".as_bytes()),
//!     (0x0002_0336, 0xBB50_0695)
//! );
//! ```

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
/// Computes the hash of a given byte sequence, expressed as a tuple of two 32-bit integers.
///
/// # Example
///
/// ```no_run
/// println!("{:?}", bsa3_hash::calculate(b"foo"));
/// ```
pub fn calculate(input: &[u8]) -> (u32, u32) {
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
            assert_eq!(
                crate::calculate(filename.as_bytes()),
                (left_hash, right_hash)
            );
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
