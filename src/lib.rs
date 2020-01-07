#![no_std]
#![forbid(unsafe_code)]

//! Just, really the littlest Vec you could need. So smol.

use core::ops::{Deref, DerefMut};
use core::mem::replace;
use core::mem::needs_drop;

extern crate alloc;
use alloc::vec::Vec;

// Note(Lokathor): We just want to hide the enum details away. Rust doesn't let
// you be an enum with private variants, so instead we make this be a private
// inner field of the public `TinyVec<T>` type.
#[derive(Debug, Clone)]
enum Payload<T: Default> {
  Inline { len: usize, data: [T; 8] },
  Heap(Vec<T>),
}

/// A `TinyVec<T>` is like a `Vec<T>`, but it will store up to 8 elements
/// "inline" on the stack before transitioning into being a normal `Vec<T>`.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct TinyVec<T: Default>(Payload<T>);

// TODO: impl a better Debug

impl<T:Default> Default for TinyVec<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T:Default> Deref for TinyVec<T> {
  type Target = [T];
  fn deref(&self) -> &[T] {
    match &self.0 {
      Payload::Inline { len, data } => {
        &data[..*len]
      }
      Payload::Heap(vec) => {
        &vec
      }
    }
  }
}

impl<T:Default> DerefMut for TinyVec<T> {
  fn deref_mut(&mut self) -> &mut [T] {
    match &mut self.0 {
      Payload::Inline { len, data } => {
        &mut data[..*len]
      }
      Payload::Heap(ref mut vec) => {
        &mut vec[..]
      }
    }
  }
}

impl<T: Default> TinyVec<T> {
  pub fn new() -> Self {
    Self(Payload::Inline {
      len: 0,
      data: [
        T::default(),
        T::default(),
        T::default(),
        T::default(),
        T::default(),
        T::default(),
        T::default(),
        T::default(),
      ],
    })
  }

  pub fn push(&mut self, val: T) {
    match &mut self.0 {
      Payload::Inline { len: 8, data } => {
        let mut v = Vec::with_capacity(8 + 10);
        for data_mut in data.iter_mut() {
          v.push(replace(data_mut, T::default()));
        }
        v.push(val);
        replace(&mut self.0, Payload::Heap(v));
      }
      Payload::Inline { len, data } => {
        debug_assert!(*len < 8, "push: illegal len: {}", len);
        data[*len] = val;
        *len += 1;
      }
      Payload::Heap(ref mut vec) => {
        vec.push(val)
      }
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    match &mut self.0 {
      Payload::Inline { len: 0, .. } => {
        None
      }
      Payload::Inline { len, data } => {
        debug_assert!(*len > 0, "pop: illegal len: {}", len);
        let out = replace(&mut data[*len - 1], T::default());
        *len -= 1;
        Some(out)
      }
      Payload::Heap(ref mut vec) => {
        vec.pop()
      }
    }
  }

  pub fn truncate(&mut self, new_len: usize) {
    match &mut self.0 {
      Payload::Inline { len, data } => {
        if needs_drop::<T>() {
          while *len > new_len {
            replace(&mut data[*len - 1], T::default());
            *len -= 1;
          }
        } else {
          *len = (*len).min(new_len);
        }
      }
      Payload::Heap(ref mut vec) => {
        vec.truncate(new_len)
      }
    }
  }
}
