# bsa3-hash

![Travis CI](https://api.travis-ci.com/FaultyRAM/bsa3-hash.svg?branch=master)
![Crates.io](https://img.shields.io/crates/v/bsa3-hash.svg)
![Docs.rs](https://docs.rs/bsa3-hash/badge.svg)

bsa3-hash provides a Rust implementation of the hash function used in BSA files for
*The Elder Scrolls III: Morrowind*.

## Usage

Add bsa3-hash to your `Cargo.toml`:

```toml
[dependencies]
bsa3-hash = "^1.0.0"
```

Then call `bsa3_hash::calculate` as needed:

```rust
fn main() {
    assert_eq!(bsa3_hash::calculate(
        r"meshes\m\probe_journeyman_01.nif",
        (0x0002_0336, 0xBB50_0695)
    ));
}
```

If you're using Rust 2015, you'll need to add bsa3-hash to your crate root as well:

```rust
extern crate bsa3_hash;
```

## License

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
