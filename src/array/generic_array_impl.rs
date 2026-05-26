use core::default;

use super::Array;
use generic_array::{ArrayLength, GenericArray, IntoArrayLength};

impl<T: Default, N: ArrayLength> Array for GenericArray<T, N> {
  type Item = T;
  const CAPACITY: usize = N::USIZE;

  #[inline(always)]
  fn as_slice(&self) -> &[T] {
    GenericArray::as_slice(self)
  }

  #[inline(always)]
  fn as_slice_mut(&mut self) -> &mut [T] {
    GenericArray::as_mut_slice(self)
  }

  #[inline(always)]
  fn default() -> Self {
    <Self as Default>::default()
  }
}

impl <T> IntoArrayLength for crate::ArrayVec<T> where T: IntoArrayLength {
    type ArrayLength = <T as IntoArrayLength>::ArrayLength;
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{ArrayVec, array_vec};
  use generic_array::ConstGenericArray;
  type TinyGenericVec<T, const N: usize> = ArrayVec<ConstGenericArray<T, N>>;

  #[test]
  fn retain_mut_empty_vec() {
    let mut av: TinyGenericVec<i32, 4> = TinyGenericVec::<i32, 4>::new();
    av.retain_mut(|&mut x| x % 2 == 0);
    assert_eq!(av.len(), 0);
  }

  #[test]
  fn retain_mut_all_elements() {
    let mut av: TinyGenericVec<i32, 4>  = array_vec!(ConstGenericArray<i32, 4> => 2, 4, 6, 8);
    av.retain_mut(|&mut x| x % 2 == 0);
    assert_eq!(av.len(), 4);
    assert_eq!(av.as_slice(), &[2, 4, 6, 8]);
  }

  #[test]
  fn retain_mut_some_elements() {
    let mut av: TinyGenericVec<i32, 4>  = array_vec!(ConstGenericArray<i32, 4> => 1, 2, 3, 4);
    av.retain_mut(|&mut x| x % 2 == 0);
    assert_eq!(av.len(), 2);
    assert_eq!(av.as_slice(), &[2, 4]);
  }

  #[test]
  fn retain_mut_no_elements() {
    let mut av: TinyGenericVec<i32, 4>  = array_vec!(ConstGenericArray<i32, 4> => 1, 3, 5, 7);
    av.retain_mut(|&mut x| x % 2 == 0);
    assert_eq!(av.len(), 0);
  }

  #[test]
  fn retain_mut_zero_capacity() {
    let mut av: TinyGenericVec<i32, 0>  = ArrayVec::new();
    av.retain_mut(|&mut x| x % 2 == 0);
    assert_eq!(av.len(), 0);
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn array_like_debug() {
    #[derive(Debug, Default, Copy, Clone)]
    struct S {
      x: u8,
      y: u8,
    }

    use core::fmt::Write;
    use alloc::string::String;

    let mut ar: ConstGenericArray<S, 2> = ConstGenericArray::<S, 2>::from_array([S { x: 1, y: 2 }, S { x: 3, y: 4 }]);
    let mut buf_ar = String::new();
    write!(&mut buf_ar, "{:#?}", ar.as_slice()).unwrap();

    let av: TinyGenericVec<S, 2> = TinyGenericVec::<S, 2>::from_array_len(ar, 2);
    let mut buf_av = String::new();
    write!(&mut buf_av, "{av:#?}").unwrap();

    assert_eq!(buf_av, buf_ar)
  }
}
