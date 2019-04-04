# secfmt

[![Build Status](https://travis-ci.org/dirkeinecke/secfmt.svg?branch=master)](https://travis-ci.org/dirkeinecke/secfmt)
[![Crate](https://img.shields.io/crates/v/secfmt.svg)](https://crates.io/crates/secfmt)
[![API](https://docs.rs/secfmt/badge.svg)](https://docs.rs/secfmt)

Converts seconds into a human readable format (struct) containing years, days, hours, minutes and seconds.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
secfmt = "0.1"
```

# Examples

```rust
let seconds = 31537529;
let seconds_human_readable = secfmt::from(seconds);

assert_eq!(1, secfmt::from(31537529).years);
assert_eq!(0, secfmt::from(31537529).days);
assert_eq!(0, secfmt::from(31537529).hours);
assert_eq!(25, secfmt::from(31537529).minutes);
assert_eq!(29, secfmt::from(31537529).seconds);
```

# License

secfmt is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.
