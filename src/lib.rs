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
//! This crate has two main types
//!
//! * `ArrayishVec`: Like the `ArrayVec` from the
//!   [arrayvec](https://docs.rs/arrayvec) crate. It's an array backed linear
//!   data store. If you push too much data it will panic.
//! * `TinyVec`: **NOT YET IMPLEMENTED. PLANNED FOR 0.2, SOON(TM)** This will be
//!   like the `SmallVec` from [smallvec](https://docs.rs/smallvec). It starts
//!   as an `ArrayishVec`, and when that _would have_ overflowed it will instead
//!   transition everything into a normal `Vec` on the heap.
//!
//! ## How Is This Different From Those Other Crates?
//!
//! It's 100% safe code. Not just "we think this unsafe code is sound so we'll
//! give you a safe abstraction". This crate doesn't have a single `unsafe`
//! block in it. If you trust the standard library to not trigger UB, then you
//! can trust this crate to do the same.
//!
//! The trade off is that the item type has to implement `Default`, and then the
//! "spare space" of the vec is kept as Default instances of the type in
//! question, rather than being uninitialized memory.
//!
//! I haven't benchmarked it, but I _suspect_ that there is a performance loss
//! compared to just using `unsafe` and `MaybeUninit` and all that. I mean the
//! code probably isn't the best it could possibly by, but _also_ even if it
//! were perfectly optimal I suspect that there will still be a performance hit
//! compared to not using `MaybeUninit`. That's why we got it into the language
//! after all.
//!
//! Still, if you really want to be sure that there's no UB going on in your
//! collection, here you are.

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

#[cfg(feature = "extern_crate_alloc")]
extern crate alloc;

mod arrayish;
pub use arrayish::*;

mod arrayish_vec;
pub use arrayish_vec::*;

#[cfg(feature = "extern_crate_alloc")]
mod tiny_vec;
#[cfg(feature = "extern_crate_alloc")]
pub use tiny_vec::*;
