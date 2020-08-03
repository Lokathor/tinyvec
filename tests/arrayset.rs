#![allow(bad_style)]

use core::mem::size_of;
use tinyvec::{ArraySet, InsertError};

#[test]
fn ArraySet_test_size() {
  assert_eq!(size_of::<ArraySet<[i8; 7], u8>>(), 8);
}

#[test]
fn ArraySet_test() {
  let mut set: ArraySet<[i8; 7], u8> = ArraySet::new();
  assert_eq!(set.capacity(), 7);

  assert_eq!(set.try_insert(1), Ok(true));
  assert_eq!(set.try_insert(5), Ok(true));
  assert_eq!(set.try_insert(6), Ok(true));
  assert_eq!(set.len(), 3);

  assert_eq!(set.try_insert(5), Ok(false));
  assert_eq!(set.len(), 3);

  assert_eq!(set.try_replace(1), Ok(Some(1)));
  assert_eq!(set.try_replace(2), Ok(None));
  assert_eq!(set.len(), 4);

  assert_eq!(set.try_insert(3), Ok(true));
  assert_eq!(set.try_insert(4), Ok(true));
  assert_eq!(set.try_insert(7), Ok(true));
  assert_eq!(set.try_insert(8), Err(InsertError));
  assert_eq!(set.len(), 7);

  assert_eq!(set.try_replace(9), Err(InsertError));

  assert_eq!(set.remove(&3), Some(3));
  assert_eq!(set.len(), 6);

  set.retain(|x| *x == 3 || *x == 6);
  assert_eq!(set.len(), 1);
  assert!(!set.contains(&3));
  assert!(set.contains(&6));
}
