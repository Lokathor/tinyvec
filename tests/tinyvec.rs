#![allow(bad_style)]

use tinyvec::*;
use std::iter::FromIterator;

#[test]
fn TinyVec_drain() {
  let mut tv: TinyVec<[i32; 10]> = Default::default();
  tv.push(1);
  tv.push(2);
  tv.push(3);

  assert_eq!(Vec::from_iter(tv.clone().drain(..)), vec![1, 2, 3]);

  assert_eq!(Vec::from_iter(tv.clone().drain(..2)), vec![1, 2]);
  assert_eq!(Vec::from_iter(tv.clone().drain(..3)), vec![1, 2, 3]);

  assert_eq!(Vec::from_iter(tv.clone().drain(..=1)), vec![1, 2]);
  assert_eq!(Vec::from_iter(tv.clone().drain(..=2)), vec![1, 2, 3]);

  assert_eq!(Vec::from_iter(tv.clone().drain(0..)), vec![1, 2, 3]);
  assert_eq!(Vec::from_iter(tv.clone().drain(1..)), vec![2, 3]);

  assert_eq!(Vec::from_iter(tv.clone().drain(0..2)), vec![1, 2]);
  assert_eq!(Vec::from_iter(tv.clone().drain(0..3)), vec![1, 2, 3]);
  assert_eq!(Vec::from_iter(tv.clone().drain(1..2)), vec![2]);
  assert_eq!(Vec::from_iter(tv.clone().drain(1..3)), vec![2, 3]);

  assert_eq!(Vec::from_iter(tv.clone().drain(0..=1)), vec![1, 2]);
  assert_eq!(Vec::from_iter(tv.clone().drain(0..=2)), vec![1, 2, 3]);
  assert_eq!(Vec::from_iter(tv.clone().drain(1..=1)), vec![2]);
  assert_eq!(Vec::from_iter(tv.clone().drain(1..=2)), vec![2, 3]);
}

#[test]
fn TinyVec_resize() {
  let mut tv: TinyVec<[i32; 10]> = Default::default();
  tv.resize(20, 5);
  assert_eq!(&tv[..], &[5; 20]);
}
