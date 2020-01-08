#![allow(bad_style)]

use tinyvec::*;

#[test]
fn ArrayishVec_push_pop() {
  let mut av: ArrayishVec<[i32; 4]> = Default::default();
  assert_eq!(av.len(), 0);
  assert_eq!(av.pop(), None);

  av.push(10_i32);
  assert_eq!(av.len(), 1);
  assert_eq!(av[0], 10);
  assert_eq!(av.pop(), Some(10));
  assert_eq!(av.len(), 0);
  assert_eq!(av.pop(), None);

  av.push(10);
  av.push(11);
  av.push(12);
  av.push(13);
  assert_eq!(av[0], 10);
  assert_eq!(av[1], 11);
  assert_eq!(av[2], 12);
  assert_eq!(av[3], 13);
  assert_eq!(av.len(), 4);
  assert_eq!(av.pop(), Some(13));
  assert_eq!(av.len(), 3);
  assert_eq!(av.pop(), Some(12));
  assert_eq!(av.len(), 2);
  assert_eq!(av.pop(), Some(11));
  assert_eq!(av.len(), 1);
  assert_eq!(av.pop(), Some(10));
  assert_eq!(av.len(), 0);
  assert_eq!(av.pop(), None);
}

#[test]
#[should_panic]
fn ArrayishVec_push_overflow() {
  let mut av: ArrayishVec<[i32; 0]> = Default::default();
  av.push(7);
}

#[test]
fn ArrayishVec_formatting() {
  // check that we get the comma placement correct

  let mut av: ArrayishVec<[i32; 4]> = Default::default();
  assert_eq!(format!("{:?}", av), "[]");
  av.push(10);
  assert_eq!(format!("{:?}", av), "[10]");
  av.push(11);
  assert_eq!(format!("{:?}", av), "[10, 11]");
  av.push(12);
  assert_eq!(format!("{:?}", av), "[10, 11, 12]");

  // below here just asserts that the impls exist.

  //
  let av: ArrayishVec<[i32; 4]> = Default::default();
  assert_eq!(format!("{:b}", av), "[]");
  assert_eq!(format!("{:o}", av), "[]");
  assert_eq!(format!("{:x}", av), "[]");
  assert_eq!(format!("{:X}", av), "[]");
  assert_eq!(format!("{}", av), "[]");
  //
  let av: ArrayishVec<[f32; 4]> = Default::default();
  assert_eq!(format!("{:e}", av), "[]");
  assert_eq!(format!("{:E}", av), "[]");
  //
  let av: ArrayishVec<[&'static str; 4]> = Default::default();
  assert_eq!(format!("{:p}", av), "[]");
}

#[test]
fn ArrayishVec_iteration() {
  let mut av: ArrayishVec<[i32; 4]> = Default::default();
  av.push(10);
  av.push(11);
  av.push(12);
  av.push(13);

  let mut i = av.into_iter();
  assert_eq!(i.next(), Some(10));
  assert_eq!(i.next(), Some(11));
  assert_eq!(i.next(), Some(12));
  assert_eq!(i.next(), Some(13));
  assert_eq!(i.next(), None);

  let mut av: ArrayishVec<[i32; 4]> = Default::default();
  av.push(10);
  av.push(11);
  av.push(12);
  av.push(13);

  let av2: ArrayishVec<[i32; 4]> = av.clone().into_iter().collect();
  assert_eq!(av, av2);
}
