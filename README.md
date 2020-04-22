[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.36-green.svg)
[![travis.ci](https://travis-ci.org/Lokathor/tinyvec.svg?branch=master)](https://travis-ci.org/Lokathor/tinyvec)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/wfnu1tyudka6jbk1/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/tinyvec/branch/master)
[![crates.io](https://img.shields.io/crates/v/tinyvec.svg)](https://crates.io/crates/tinyvec)
[![docs.rs](https://docs.rs/tinyvec/badge.svg)](https://docs.rs/tinyvec/)

![Unsafe-Zero-Percent](https://img.shields.io/badge/Unsafety-0%25-brightgreen.svg)

# tinyvec

A crate for vec-like types using 100% safe code.

There's two main types provided: `ArrayVec` is backed by an array, and `TinyVec`
is an enum that's either an "inline" `ArrayVec` or a "heap" `Vec`.

Also provided is a `SliceVec` for when you just want to operate on a slice.

These vecs only work with types that have the "Placeholder" trait, which is the
value that's placed into the inactive portions of a vector. This is already
implemented for all `Default` types, but if your type isn't default you'll have
to provide an impl for your type to work with these vecs.

For full details, please see [the docs.rs documentation](https://docs.rs/tinyvec/)
