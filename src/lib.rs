#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(
  feature = "nightly_slice_partition_dedup",
  feature(slice_partition_dedup)
)]
#![cfg_attr(feature = "nightly_const_generics", allow(incomplete_features))]
#![cfg_attr(feature = "nightly_const_generics", feature(const_generics))]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::must_use_candidate)]
#![warn(missing_docs)]

//! Programmers can have a little vec, as a treat.
//!
//! ## What This Is
//!
//! This crate provides 100% safe code alternatives to both
//! [arrayvec](https://docs.rs/arrayvec) and
//! [smallvec](https://docs.rs/smallvec).
//!
//! Being 100% safe means that you have to have some sort of compromise compared
//! to the versions using `unsafe`. In this case, the compromise is that the
//! element type must implement `Default` to be usable in these vecs. However,
//! that still allows you to use [quite a few
//! types](https://doc.rust-lang.org/std/default/trait.Default.html#implementors),
//! so I think that you'll find these vecs useful in many cases.
//!
//! * [`ArrayVec`](ArrayVec) is an array-backed vec-like structure with a fixed
//!   capacity. If you try to grow the length past the array's capacity it will
//!   error or panic (depending on the method used).
//! * (`alloc` feature) [`TinyVec`](TinyVec) is an enum that's either an
//!   "Inline" `ArrayVec` or a "Heap" `Vec`. If it's Inline and you try to grow
//!   the `ArrayVec` beyond its array capacity it will quietly transition into
//!   Heap mode and then continue the operation.
//!
//! ## Crate Goals
//!
//! 1) The crate is 100% safe code. Not just a safe API, there are also no
//!    `unsafe` internals. `#![forbid(unsafe_code)]`.
//! 2) No required dependencies.
//!    * We might provide optional dependencies for extra functionality (eg:
//!      `serde` compatibility).
//! 3) The intended API is that, _as much as possible_, these types are
//!    essentially a "drop-in" replacement for the standard [`Vec`](Vec::<T>)
//!    type.
//!    * Stable `Vec` methods that the vecs here also have should be the same
//!      general signature.
//!    * Unstable `Vec` methods are sometimes provided via a crate feature, but
//!      if so they also require a Nightly compiler.
//!    * Some methods are provided that _are not_ part of the `Vec` type, such
//!      as additional constructor methods. In this case, the names are rather
//!      long and whimsical in the hopes that they don't clash with any
//!      possible future methods of `Vec`.
//!    * If, in the future, `Vec` stabilizes a method that clashes with an
//!      existing extra method here then we'll simply be forced to release a
//!      2.y.z version. Not the end of the world.
//!    * Some methods of `Vec` are simply inappropriate and will not be
//!      implemented here. For example, `ArrayVec` cannot possibly implement
//!      [`from_raw_parts`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts).

use core::{
  borrow::{Borrow, BorrowMut},
  cmp::PartialEq,
  convert::AsMut,
  default::Default,
  fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer,
    UpperExp, UpperHex,
  },
  hash::{Hash, Hasher},
  iter::{Extend, FromIterator, FusedIterator, IntoIterator, Iterator},
  mem::{needs_drop, replace},
  ops::{Deref, DerefMut, Index, IndexMut, RangeBounds},
  slice::SliceIndex,
};

#[cfg(feature = "alloc")]
#[doc(hidden)] // re-export for macros
pub extern crate alloc;

mod array;
pub use array::*;

mod arrayvec;
pub use arrayvec::*;

#[cfg(feature = "alloc")]
mod tinyvec;
#[cfg(feature = "alloc")]
pub use crate::tinyvec::*;

// TODO MSRV(1.40.0): Just call the normal `core::mem::take`
#[inline(always)]
fn take<T: Default>(from: &mut T) -> T {
  replace(from, T::default())
}
