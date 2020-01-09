[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.36-green.svg)
[![travis.ci](https://travis-ci.org/Lokathor/tinyvec.svg?branch=master)](https://travis-ci.org/Lokathor/tinyvec)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/wfnu1tyudka6jbk1/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/tinyvec/branch/master)
[![crates.io](https://img.shields.io/crates/v/tinyvec.svg)](https://crates.io/crates/tinyvec)
[![docs.rs](https://docs.rs/tinyvec/badge.svg)](https://docs.rs/tinyvec/)

![Unsafe-Zero-Percent](https://img.shields.io/badge/Unsafety-0%25-green.svg)

# tinyvec

Just, really the littlest Vec you could need. So smol.

This is one of those crates where an initial small number of elements will be
kept on the stack, and then if you overflow that it will quietly transition
everything into being a totally normal `alloc::vec::Vec`.

What sets this crate apart from others like it is that it has
`#![forbid(unsafe_code)]` right at the top, and then requires that the element
type have a `Default` impl. This is a slight restriction compared to a normal
`Vec`, but that's still [a very large number of
types](https://doc.rust-lang.org/std/default/trait.Default.html#implementors)
that you can use.
