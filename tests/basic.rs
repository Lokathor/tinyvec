
extern crate std;

use tinyvec::*;

#[test]
fn test_push_pop() {
  let mut tv = TinyVec::new();

  assert_eq!(tv.pop(), None);
  
  tv.push(5_i32);
  assert_eq!(tv.pop(), Some(5_i32));
  assert_eq!(tv.len(), 0);

  for i in 0 .. 10 {
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
