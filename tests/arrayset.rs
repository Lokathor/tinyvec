#![allow(bad_style)]

use tinyvec::ArraySet;

#[test]
fn ArraySet_test() {
  let mut set: ArraySet<[i8; 7]> = ArraySet::default();
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
  assert_eq!(set.try_insert(8), Err(8));
  assert_eq!(set.len(), 7);

  assert_eq!(set.try_replace(9), Err(9));

  assert_eq!(set.remove(&3), Some(3));
  assert_eq!(set.len(), 6);

  set.retain(|x| *x == 3 || *x == 6);
  assert_eq!(set.len(), 1);
  assert!(!set.contains(&3));
  assert!(set.contains(&6));
}
