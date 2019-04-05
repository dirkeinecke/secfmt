# secfmt

[![Build Status](https://travis-ci.org/dirkeinecke/secfmt.svg?branch=master)](https://travis-ci.org/dirkeinecke/secfmt)
[![Coverage Status](https://coveralls.io/repos/github/dirkeinecke/secfmt/badge.svg?branch=master)](https://coveralls.io/github/dirkeinecke/secfmt?branch=master)
[![Crate](https://img.shields.io/crates/v/secfmt.svg)](https://crates.io/crates/secfmt)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/secfmt.svg)](https://crates.io/crates/secfmt)
[![API](https://docs.rs/secfmt/badge.svg)](https://docs.rs/secfmt)
![License](https://img.shields.io/crates/l/secfmt.svg)
[![Gitter](https://badges.gitter.im/secfmt/community.svg)](https://gitter.im/secfmt/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

Converts seconds into a human readable format (struct) containing years, days, hours, minutes and seconds.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
secfmt = "0.1"
```

Add this to your `*.rs` file:

```rust
extern crate secfmt;
```

Now you can proceed as follows:

### Example 1

```rust
let seconds = 31537529;
let seconds_human_readable = secfmt::from(seconds);
```

### Example 2

```rust
assert_eq!(1, secfmt::from(31537529).years);
assert_eq!(0, secfmt::from(31537529).days);
assert_eq!(0, secfmt::from(31537529).hours);
assert_eq!(25, secfmt::from(31537529).minutes);
assert_eq!(29, secfmt::from(31537529).seconds);
```

### Example 3

```rust
let shr = secfmt::from(31537529);
let s = format!("{}y {}d {}h {}m {}s", shr.years, shr.days, shr.hours, shr.minutes, shr.seconds);
assert_eq!("1y 0d 0h 25m 29s", s);
```

## Getting help

If you have questions or problems with `secfmt`, then we are happy to respond to [GitHub issues](https://github.com/dirkeinecke/secfmt/issues/new) or come chat with us on our [Gitter channel](https://gitter.im/secfmt/community) - if you have any questions about the project, or just want to say hi!

## License

`secfmt` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.
