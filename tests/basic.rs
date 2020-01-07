#![allow(bad_style)]

extern crate tinyvec;
use tinyvec::*;

extern crate core;
use core::ops::Deref;

#[test]
fn TinyVec_push_pop() {
  let mut tv = TinyVec::new();

  assert_eq!(tv.pop(), None);

  tv.push(5_i32);
  assert_eq!(tv.pop(), Some(5_i32));
  assert_eq!(tv.len(), 0);

  for i in 0..10 {
    tv.push(i);
  }
  assert_eq!(tv.len(), 10);
  assert_eq!(tv.pop(), Some(9));
  assert_eq!(tv.pop(), Some(8));
  assert_eq!(tv.pop(), Some(7));
  assert_eq!(tv.pop(), Some(6));
  assert_eq!(tv.pop(), Some(5));
  assert_eq!(tv.pop(), Some(4));
  assert_eq!(tv.pop(), Some(3));
  assert_eq!(tv.pop(), Some(2));
  assert_eq!(tv.pop(), Some(1));
  assert_eq!(tv.pop(), Some(0));
  assert_eq!(tv.pop(), None);
  assert_eq!(tv.pop(), None);
  assert_eq!(tv.pop(), None);
}

#[test]
fn TinyVec_truncate() {
  // Truncating a five element vector to two elements
  let mut tv = TinyVec::new();
  tv.push(1_i32);
  tv.push(2);
  tv.push(3);
  tv.push(4);
  tv.push(5);
  tv.truncate(2);
  assert_eq!(tv.deref(), [1, 2]);

  // No truncation occurs when len is greater than the vector's current length
  let mut tv = TinyVec::new();
  tv.push(1_i32);
  tv.push(2);
  tv.push(3);
  tv.truncate(8);
  assert_eq!(tv.deref(), [1, 2, 3]);

  // Truncating when len == 0 is equivalent to calling the clear method.
  let mut tv = TinyVec::new();
  tv.push(1_i32);
  tv.push(2);
  tv.push(3);
  tv.truncate(0);
  assert_eq!(tv.deref(), []);
}
