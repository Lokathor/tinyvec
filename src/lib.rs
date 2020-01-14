#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(
  feature = "nightly_slice_partition_dedup",
  feature(slice_partition_dedup)
)]
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
//! * Being 100% safe means that you have to have some sort of compromise
//!   compared to the versions using `unsafe`. In this case, the compromise is
//!   that the element type must implement `Default` to be usable in these vecs.
//!   This makes TinyVec structures not applicable for truly arbitrary data types.
//!   However, [quite a
//!   few](https://doc.rust-lang.org/std/default/trait.Default.html#implementors)
//!   types have a `Default` impl, including the common cases such as `u8`, `char`
//!   and even `&str`.
//! * [`ArrayVec`] is an array-backed vec-like structure with a fixed capacity.
//!   If you try to grow the length past the
//!   array's capacity it will error or panic (depending on the method used).
//!   * (Note: I am _very sorry_ that this type has the same name as the
//!     `ArrayVec` type in the `arrayvec` crate. We really couldn't think of
//!     another name for this sort of data structure. Please [contact
//!     us](https://github.com/Lokathor/tinyvec/issues) with a better name
//!     before this crate is 1.0 if you can think of one.)
//! * [`TinyVec`] is an enum that's either an "inline" `ArrayVec` or a "heap"
//!   `Vec`. If it's in array mode and you try to grow the vec beyond its
//!   capacity it'll quietly transition into heap mode for you and then continue
//!   operation. This type is naturally behind the `alloc` feature gate.
//!
//! ## Stability Goal
//!
//! The crate is still in development, but we have some very clear goals:
//!
//! 1) The crate is 100% safe code. Not just a safe API - no `unsafe` internals
//!    either. `#![forbid(unsafe_code)]`.
//! 2) No required dependencies.
//!    * We might provide optional dependencies for extra
//!      functionality (eg: `serde` compatability), but none of them will be
//!      required.
//! 3) The _intended_ API is that, as much as possible, these types are
//!    essentially a "drop-in" replacement for the standard
//!    [`Vec`](alloc::vec::Vec) type.
//!    * For `Vec` methods that are not yet Stable, they are sometimes provided
//!      via a crate feature, in which case the feature requires Nightly.
//!    * If `Vec` methods that are stable but which rely on an unstable library
//!      internals, that also requires a feature and a nightly compiler (sorry).
//!    * Some of the methods provided are **not** part of the `Vec` API but are
//!      none the less important methods to have. In this case, the method names
//!      are usually fairly long and perhaps even a little silly. It is the hope
//!      that this "convention" will prevent any potential name clash between
//!      our vec types and the standard `Vec` type.
//!    * That said, if `Vec` lands
//!      some method with the same name as something we have, we'll just bite
//!      the bullet and fix it with a breaking change.

use core::{
  borrow::{Borrow, BorrowMut},
  cmp::PartialEq,
  convert::AsMut,
  default::Default,
  fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer,
    UpperExp, UpperHex,
  },
  iter::{Extend, FromIterator, IntoIterator, Iterator},
  mem::{needs_drop, replace},
  ops::{Deref, DerefMut, Index, IndexMut, RangeBounds},
  slice::SliceIndex,
};

#[cfg(feature = "alloc")]
extern crate alloc;

mod array;
pub use array::*;

mod arrayvec;
pub use arrayvec::*;

#[cfg(feature = "alloc")]
mod tinyvec;
#[cfg(feature = "alloc")]
pub use tinyvec::*;
