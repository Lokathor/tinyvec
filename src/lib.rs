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

//! `tinyvec` provides 100% safe vec-like data structures.
//!
//! ## Provided Types
//! With no features enabled, this crate provides the [`ArrayVec`] type, which
//! is an array-backed storage. You can push values into the array and pop them
//! out of the array and so on. If the array is made to overflow it will panic.
//!
//! Similarly, there is also a [`SliceVec`] type available, which is a vec-like
//! that's backed by a slice you provide. You can add and remove elements, but
//! if you overflow the vec it will panic.
//!
//! With the `alloc` feature enabled, the crate also has a [`TinyVec`] type.
//! This is an enum type which is either an `Inline(ArrayVec)` or a `Heap(Vec)`.
//! If a `TinyVec` is `Inline` and would overflow it automatically transitions
//! itself into being `Heap` mode instead of a panic.
//!
//! All of this is done with no `unsafe` code within the crate. (Technically
//! the `Vec` type from the standard library uses `unsafe` internally, but *this
//! crate* introduces no new `unsafe` code into your project).
//!
//! The limitation on all of this is that the element type of a vec from this
//! crate must support the [`Default`] trait. This means that this crate isn't
//! suitable for all situations, but a very surprising number of types do
//! support `Default`.

#[allow(unused_imports)]
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

#[cfg(feature = "experimental_array_set")]
mod arrayset;
#[cfg(feature = "experimental_array_set")]
pub use arrayset::*;

mod arrayvec;
pub use arrayvec::*;

mod slicevec;
pub use slicevec::*;

#[cfg(feature = "alloc")]
mod tinyvec;
#[cfg(feature = "alloc")]
pub use crate::tinyvec::*;

// TODO MSRV(1.40.0): Just call the normal `core::mem::take`
#[inline(always)]
fn take<T: Default>(from: &mut T) -> T {
  replace(from, T::default())
}
