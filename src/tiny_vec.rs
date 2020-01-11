#![cfg(feature = "extern_crate_alloc")]

use super::*;

use alloc::vec::Vec;

pub enum TinyVec<A: Arrayish> {
  Inline(ArrayishVec<A>),
  Heap(Vec<A::Item>)
}
