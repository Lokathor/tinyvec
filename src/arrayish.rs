/// A trait for types that can be the backing store of an
/// [`ArrayishVec`](ArrayishVec::<A>).
///
/// You are not generally expected to need to implement this yourself. You can
/// if you want I guess. Impls are provided for arrays of length `0..=32`, 33,
/// and powers of 2 up to 4096. Additional lengths can probably be added upon
/// request.
///
/// ## Safety
///
/// As a reminder, the `Arrayish` trait is 100% safe so unsafe code **must not**
/// rely on an instance of the trait being correct.
pub trait Arrayish {
  /// The type of the items in the thing.
  type Item: Default;

  /// The number of slots in the thing.
  const CAPACITY: usize;

  /// Gives a shared slice over the whole thing.
  fn slice(&self) -> &[Self::Item];

  /// Gives a unique slice over the whole thing.
  fn slice_mut(&mut self) -> &mut [Self::Item];
}

macro_rules! impl_arrayish_for_len {
  ($($len:expr),+ $(,)?) => {
    $(impl<T: Default> Arrayish for [T; $len] {
      type Item = T;
      const CAPACITY: usize = $len;
      #[inline(always)]
      fn slice(&self) -> &[T] {
        &*self
      }
      #[inline(always)]
      fn slice_mut(&mut self) -> &mut [T] {
        &mut *self
      }
    })+
  }
}

impl_arrayish_for_len! {
  0, /* The oft-forgotten 0-length array! */
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
  17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
  33, /* for luck */
  64, 128, 256, 512, 1024, 2048, 4096,
}
