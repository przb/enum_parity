[<img alt="docs.rs" src="https://img.shields.io/docsrs/enum_parity" height="20">](https://docs.rs/enum_parity)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/przb/enum_parity/rust.yml" height="20">](https://github.com/przb/enum_parity/actions)

# Enum Bit Parity

This crate exposes a macro to enforce enum discriminants with a given bit parity.
Using even or odd bit parity enforces a
[Hamming weight](https://en.wikipedia.org/wiki/Hamming_weight) of two.

## Example and Usage
How to use with Cargo:
```toml
[dependencies]
enum_parity = "0.1.0"
```

To use in your crate:
```rust
use parity_enum::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
pub enum EvenParitySample {
    Foo,
    Bar,
    Baz,
    Quo,
}

#[bit_parity(odd)]
#[repr(u8)]
pub enum OddParitySample {
    Lorem,
    Ipsum,
    Dolor,
    Sit,
}
```

This gets expanded to have the given bit parity as follows:
```rust
#[repr(u8)]
pub enum EvenParitySample {
    Foo = 0u8,
    Bar = 3u8,
    Baz = 5u8,
    Quo = 6u8,
}

#[repr(u8)]
pub enum OddParitySample {
    Lorem = 1u8,
    Ipsum = 2u8,
    Dolor = 4u8,
    Sit = 7u8,
}
```


## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
https://www.apache.org/licenses/LICENSE-2.0 or the MIT license
https://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.

