#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(
  feature = "nightly_slice_partition_dedup",
  feature(slice_partition_dedup)
)]

//! Just, really the littlest Vec you could need. So smol.

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
  ops::{Deref, DerefMut, Index, IndexMut},
  slice::SliceIndex,
};

#[cfg(feature = "extern_crate_alloc")]
extern crate alloc;

mod arrayish;
pub use arrayish::*;

mod arrayish_vec;
pub use arrayish_vec::*;
