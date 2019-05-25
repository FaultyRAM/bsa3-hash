// Copyright (c) 2019 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

include!("../bsa_lists/morrowind.rs");
include!("../bsa_lists/tribunal.rs");
include!("../bsa_lists/bloodmoon.rs");

macro_rules! compute_hashes {
    ($c:expr, $id:expr, $list:expr) => {
        $c.bench_function($id, |b| {
            b.iter(|| {
                for &(filename, _, _) in $list {
                    bsa3_hash::calculate(black_box(filename));
                }
            })
        });
    };
}

fn morrowind_bsa(c: &mut Criterion) {
    compute_hashes!(c, "Compute Morrowind.bsa hashes", MORROWIND_BSA);
}

fn tribunal_bsa(c: &mut Criterion) {
    compute_hashes!(c, "Compute Tribunal.bsa hashes", TRIBUNAL_BSA);
}

fn bloodmoon_bsa(c: &mut Criterion) {
    compute_hashes!(c, "Compute Bloodmoon.bsa hashes", BLOODMOON_BSA);
}

criterion_group!(benches, morrowind_bsa, tribunal_bsa, bloodmoon_bsa);
criterion_main!(benches);
