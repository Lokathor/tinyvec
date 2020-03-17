#![allow(bad_style)]
#![allow(clippy::redundant_clone)]

use std::iter::FromIterator;
use tinyvec::*;

#[test]
fn TinyVec_swap_remove() {
  let mut tv: TinyVec<[i32; 10]> = Default::default();
  tv.push(1);
  tv.push(2);
  tv.push(3);
  tv.push(4);
  assert_eq!(tv.swap_remove(3), 4);
  assert_eq!(&tv[..], &[1, 2, 3][..]);
  assert_eq!(tv.swap_remove(0), 1);
  assert_eq!(&tv[..], &[3, 2][..]);
  assert_eq!(tv.swap_remove(0), 3);
  assert_eq!(&tv[..], &[2][..]);
  assert_eq!(tv.swap_remove(0), 2);
  assert_eq!(&tv[..], &[][..]);
}

#[test]
fn TinyVec_capacity() {
  let mut tv: TinyVec<[i32; 1]> = Default::default();
  assert_eq!(tv.capacity(), 1);
  tv.move_to_the_heap();
  tv.extend_from_slice(&[1, 2, 3, 4]);
  assert_eq!(tv.capacity(), 4);
}

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

#[test]
fn TinyVec_from_slice_impl() {
  let bigger_slice: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let tinyvec: TinyVec<[u8; 10]> = TinyVec::Heap((&bigger_slice[..]).into());
  assert_eq!(TinyVec::from(&bigger_slice[..]), tinyvec);

  let smaller_slice: [u8; 5] = [0, 1, 2, 3, 4];
  let tinyvec: TinyVec<[u8; 10]> = TinyVec::Inline(ArrayVec::from_array_len(
    [0, 1, 2, 3, 4, 0, 0, 0, 0, 0],
    5,
  ));
  assert_eq!(TinyVec::from(&smaller_slice[..]), tinyvec);

  let same_size: [u8; 4] = [0, 1, 2, 3];
  let tinyvec: TinyVec<[u8; 4]> =
    TinyVec::Inline(ArrayVec::from_array_len(same_size, 4));
  assert_eq!(TinyVec::from(&same_size[..]), tinyvec);
}

#[test]
fn TinyVec_from_array() {
  let array = [9, 8, 7, 6, 5, 4, 3, 2, 1];
  let tv = TinyVec::from(array);
  assert_eq!(&array, &tv[..]);
}
