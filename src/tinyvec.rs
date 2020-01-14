#![cfg(feature = "alloc")]

use super::*;

use alloc::vec::Vec;

/// Helper to make a `TinyVec`.
///
/// You specify the backing array type, and optionally give all the elements you
/// want to initially place into the array.
///
/// As an unfortunate restriction, the backing array type must support `Default`
/// for it to work with this macro.
///
/// ```rust
/// use tinyvec::*;
/// 
/// let empty_av = tiny_vec!([u8; 16]);
/// 
/// let some_ints = tiny_vec!([i32; 4], 1, 2, 3);
/// ```
#[macro_export]
macro_rules! tiny_vec {
  ($array_type:ty) => {
    {
      let mut tv: TinyVec<$array_type> = Default::default();
      tv
    }
  };
  ($array_type:ty, $($elem:expr),*) => {
    {
      let mut tv: TinyVec<$array_type> = Default::default();
      $( tv.push($elem); )*
      tv
    }
  };
}

/// A vector that starts inline, but can automatically move to the heap.
///
/// * Requires the `alloc` feature
///
/// Note: This is currently an enum but you can mostly ignore that detail if you
/// like. In future versions this **may** become an opaque struct, but for now
/// we're being a little more liberal about allowing you to "access the
/// internals" since no safety invariants are on the line. It's kinda wild how
/// much you can just let people poke at stuff without worry when it's 100% safe
/// code.
#[derive(Clone)]
pub enum TinyVec<A: Array> {
  #[allow(missing_docs)]
  Inline(ArrayVec<A>),
  #[allow(missing_docs)]
  Heap(Vec<A::Item>)
}
impl<A: Array + Default> Default for TinyVec<A> {
  #[inline]
  #[must_use]
  fn default() -> Self {
    TinyVec::Inline(ArrayVec::default())
  }
}
impl<A: Array> TinyVec<A> {
  /// Moves the content of the TinyVec to the heap, if it's inline.
  #[allow(clippy::missing_inline_in_public_items)]
  pub fn move_to_the_heap(&mut self) {
    match self {
      TinyVec::Inline(ref mut arr) => {
        let mut v = Vec::with_capacity(A::CAPACITY * 2);
        for item in arr.drain(..) {
          v.push(item);
        }
        replace(self, TinyVec::Heap(v));
      }
      TinyVec::Heap(_) => (),
    }
  }
}

impl<A: Array> Deref for TinyVec<A> {
  type Target = [A::Item];
  #[inline(always)]
  #[must_use]
  fn deref(&self) -> &Self::Target {
    match self {
      TinyVec::Inline(a) => a.deref(),
      TinyVec::Heap(v) => v.deref(),
    }
  }
}

impl<A: Array> DerefMut for TinyVec<A> {
  #[inline(always)]
  #[must_use]
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      TinyVec::Inline(a) => a.deref_mut(),
      TinyVec::Heap(v) => v.deref_mut(),
    }
  }
}

impl<A: Array, I: SliceIndex<[A::Item]>> Index<I> for TinyVec<A> {
  type Output = <I as SliceIndex<[A::Item]>>::Output;
  #[inline(always)]
  #[must_use]
  fn index(&self, index: I) -> &Self::Output {
    &self.deref()[index]
  }
}

impl<A: Array, I: SliceIndex<[A::Item]>> IndexMut<I> for TinyVec<A> {
  #[inline(always)]
  #[must_use]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    &mut self.deref_mut()[index]
  }
}

impl<A: Array> TinyVec<A> {
  /// Move all values from `other` into this vec.
  #[inline]
  pub fn append(&mut self, other: &mut Self) {
    for item in other.drain(..) {
      self.push(item)
    }
  }

  /// A mutable pointer to the backing array.
  /// 
  /// ## Safety
  /// 
  /// This pointer has provenance over the _entire_ backing array/buffer.
  #[inline(always)]
  #[must_use]
  pub fn as_mut_ptr(&mut self) -> *mut A::Item {
    match self {
      TinyVec::Inline(a) => a.as_mut_ptr(),
      TinyVec::Heap(v) => v.as_mut_ptr(),
    }
  }

  /// Helper for getting the mut slice.
  #[inline(always)]
  #[must_use]
  pub fn as_mut_slice(&mut self) -> &mut [A::Item] {
    self.deref_mut()
  }

  /// A const pointer to the backing array.
  /// 
  /// ## Safety
  /// 
  /// This pointer has provenance over the _entire_ backing array/buffer.
  #[inline(always)]
  #[must_use]
  pub fn as_ptr(&self) -> *const A::Item {
    match self {
      TinyVec::Inline(a) => a.as_ptr(),
      TinyVec::Heap(v) => v.as_ptr(),
    }
  }

  /// Helper for getting the shared slice.
  #[inline(always)]
  #[must_use]
  pub fn as_slice(&self) -> &[A::Item] {
    self.deref()
  }

  /// The capacity of the `TinyVec`.
  /// 
  /// This is fixed based on the array type.
  #[inline(always)]
  #[must_use]
  pub fn capacity(&self) -> usize {
    A::CAPACITY
  }

  /// Removes all elements from the vec.
  #[inline(always)]
  pub fn clear(&mut self) {
    self.truncate(0)
  }

  #[cfg(feature = "nightly_slice_partition_dedup")]
  #[inline(always)]
  pub fn dedup(&mut self)
  where
    A::Item: PartialEq,
  {
    self.dedup_by(|a, b| a == b)
  }

  #[cfg(feature = "nightly_slice_partition_dedup")]
  #[inline(always)]
  pub fn dedup_by<F>(&mut self, same_bucket: F)
  where
    F: FnMut(&mut A::Item, &mut A::Item) -> bool,
  {
    let len = {
      let (dedup, _) = self.as_mut_slice().partition_dedup_by(same_bucket);
      dedup.len()
    };
    self.truncate(len);
  }

  #[cfg(feature = "nightly_slice_partition_dedup")]
  #[inline(always)]
  pub fn dedup_by_key<F, K>(&mut self, mut key: F)
  where
    F: FnMut(&mut A::Item) -> K,
    K: PartialEq,
  {
    self.dedup_by(|a, b| key(a) == key(b))
  }

  /// Creates a draining iterator that removes the specified range in the vector
  /// and yields the removed items.
  ///
  /// ## Panics
  /// * If the start is greater than the end
  /// * If the end is past the edge of the vec.
  ///
  /// ## Example
  /// ```rust
  /// use tinyvec::*;
  /// let mut tv = tiny_vec!([i32; 4], 1, 2, 3);
  /// let tv2: TinyVec<[i32; 4]> = tv.drain(1..).collect();
  /// assert_eq!(tv.as_slice(), &[1][..]);
  /// assert_eq!(tv2.as_slice(), &[2, 3][..]);
  ///
  /// tv.drain(..);
  /// assert_eq!(tv.as_slice(), &[]);
  /// ```
  #[inline]
  pub fn drain<R: RangeBounds<usize>>(
    &mut self,
    range: R,
  ) -> TinyVecDrain<'_, A> {
    use core::ops::Bound;
    let start = match range.start_bound() {
      Bound::Included(x) => *x,
      Bound::Excluded(x) => x + 1,
      Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
      Bound::Included(x) => x + 1,
      Bound::Excluded(x) => *x,
      Bound::Unbounded => self.len(),
    };
    assert!(
      start <= end,
      "TinyVec::drain> Illegal range, {} to {}",
      start,
      end
    );
    assert!(
      end <= self.len(),
      "TinyVec::drain> Range ends at {} but length is only {}!",
      end,
      self.len()
    );
    TinyVecDrain {
      parent: self,
      target_index: start,
      target_count: end - start,
    }
  }

  // LATER(Vec): drain_filter #nightly https://github.com/rust-lang/rust/issues/43244

  /// Clone each element of the slice into this vec.
  #[inline]
  pub fn extend_from_slice(&mut self, sli: &[A::Item])
  where
    A::Item: Clone,
  {
    for i in sli {
      self.push(i.clone())
    }
  }

  /// Wraps up an array and uses the given length as the initial length.
  ///
  /// Note that the `From` impl for arrays assumes the full length is used.
  ///
  /// ## Panics
  ///
  /// The length must be less than or equal to the capacity of the array.
  #[inline]
  #[must_use]
  #[allow(clippy::match_wild_err_arm)]
  pub fn from_array_len(data: A, len: usize) -> Self {
    match Self::try_from_array_len(data, len) {
      Ok(out) => out,
      Err(_) => {
        panic!("TinyVec: length {} exceeds capacity {}!", len, A::CAPACITY)
      }
    }
  }

  /// Inserts an item at the position given, moving all following elements +1
  /// index.
  ///
  /// ## Panics
  /// * If `index` > `len`
  ///
  /// ## Example
  /// ```rust
  /// use tinyvec::*;
  /// let mut tv = tiny_vec!([i32; 10], 1, 2, 3);
  /// tv.insert(1, 4);
  /// assert_eq!(tv.as_slice(), &[1, 4, 2, 3]);
  /// tv.insert(4, 5);
  /// assert_eq!(tv.as_slice(), &[1, 4, 2, 3, 5]);
  /// ```
  #[inline]
  pub fn insert(&mut self, index: usize, item: A::Item) {
    match self {
      TinyVec::Inline(a) => if a.len() == A::CAPACITY {
        self.move_to_the_heap();
        self.insert(index, item)
      } else {
        a.insert(index, item);
      },
      TinyVec::Heap(v) => v.insert(index, item),
    }
  }

  /// If the vec is empty.
  #[inline(always)]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// The length of the vec (in elements).
  #[inline(always)]
  #[must_use]
  pub fn len(&self) -> usize {
    match self {
      TinyVec::Inline(a) => a.len(),
      TinyVec::Heap(v) => v.len(),
    }
  }

  /// Makes a new, empty vec.
  #[inline(always)]
  #[must_use]
  pub fn new() -> Self
  where
    A: Default,
  {
    Self::default()
  }

  /// Remove and return the last element of the vec, if there is one.
  /// 
  /// ## Failure
  /// * If the vec is empty you get `None`.
  #[inline]
  pub fn pop(&mut self) -> Option<A::Item> {
    match self {
      TinyVec::Inline(a) => a.pop(),
      TinyVec::Heap(v) => v.pop(),
    }
  }

  /// Place an element onto the end of the vec.
  /// 
  /// See also, [`try_push`](TinyVec::<A>::try_push)
  /// ## Panics
  /// * If the length of the vec would overflow the capacity.
  #[inline(always)]
  pub fn push(&mut self, val: A::Item) {
    match self {
      TinyVec::Inline(a) => if a.len() == A::CAPACITY {
        self.move_to_the_heap();
        self.push(val)
      } else {
        a.push(val);
      },
      TinyVec::Heap(v) => v.push(val),
    }
  }

  /// Removes the item at `index`, shifting all others down by one index.
  ///
  /// Returns the removed element.
  ///
  /// ## Panics
  ///
  /// If the index is out of bounds.
  ///
  /// ## Example
  ///
  /// ```rust
  /// use tinyvec::*;
  /// let mut tv = tiny_vec!([i32; 4], 1, 2, 3);
  /// assert_eq!(tv.remove(1), 2);
  /// assert_eq!(tv.as_slice(), &[1, 3][..]);
  /// ```
  #[inline]
  pub fn remove(&mut self, index: usize) -> A::Item {
    match self {
      TinyVec::Inline(a) => a.remove(index),
      TinyVec::Heap(v) => v.remove(index),
    }
  }

  // NIGHTLY: remove_item, https://github.com/rust-lang/rust/issues/40062

  /// Resize the vec to the new length.
  ///
  /// If it needs to be longer, it's filled with clones of the provided value.
  /// If it needs to be shorter, it's truncated.
  ///
  /// ## Example
  ///
  /// ```rust
  /// use tinyvec::*;
  ///
  /// let mut tv = tiny_vec!([&str; 10], "hello");
  /// tv.resize(3, "world");
  /// assert_eq!(tv.as_slice(), &["hello", "world", "world"][..]);
  ///
  /// let mut tv = tiny_vec!([i32; 10], 1, 2, 3, 4);
  /// tv.resize(2, 0);
  /// assert_eq!(tv.as_slice(), &[1, 2][..]);
  /// ```
  #[inline]
  pub fn resize(&mut self, new_len: usize, new_val: A::Item)
  where
    A::Item: Clone,
  {
    match self {
      TinyVec::Inline(a) => if new_len > A::CAPACITY {
        self.move_to_the_heap();
        self.resize(new_len, new_val);
      } else {
        a.resize(new_len, new_val);
      },
      TinyVec::Heap(v) => v.resize(new_len, new_val),
    }
  }

  /// Resize the vec to the new length.
  ///
  /// If it needs to be longer, it's filled with repeated calls to the provided
  /// function. If it needs to be shorter, it's truncated.
  ///
  /// ## Example
  ///
  /// ```rust
  /// use tinyvec::*;
  ///
  /// let mut tv = tiny_vec!([i32; 10], 1, 2, 3);
  /// tv.resize_with(5, Default::default);
  /// assert_eq!(tv.as_slice(), &[1, 2, 3, 0, 0][..]);
  ///
  /// let mut tv = tiny_vec!([i32; 10]);
  /// let mut p = 1;
  /// tv.resize_with(4, || {
  ///   p *= 2;
  ///   p
  /// });
  /// assert_eq!(tv.as_slice(), &[2, 4, 8, 16][..]);
  /// ```
  #[inline]
  pub fn resize_with<F: FnMut() -> A::Item>(
    &mut self,
    new_len: usize,
    f: F,
  ) {
    match self {
      TinyVec::Inline(a) => a.resize_with(new_len, f),
      TinyVec::Heap(v) => v.resize_with(new_len, f),
    }
  }

  /// Walk the vec and keep only the elements that pass the predicate given.
  ///
  /// ## Example
  ///
  /// ```rust
  /// use tinyvec::*;
  ///
  /// let mut tv = tiny_vec!([i32; 10], 1, 2, 3, 4);
  /// tv.retain(|&x| x % 2 == 0);
  /// assert_eq!(tv.as_slice(), &[2, 4][..]);
  /// ```
  #[inline]
  pub fn retain<F: FnMut(&A::Item) -> bool>(&mut self, acceptable: F) {
    match self {
      TinyVec::Inline(a) => a.retain(acceptable),
      TinyVec::Heap(v) => v.retain(acceptable),
    }
  }

  // LATER(Vec): splice

  /// Splits the collection at the point given.
  ///
  /// * `[0, at)` stays in this vec
  /// * `[at, len)` ends up in the new vec.
  ///
  /// ## Panics
  /// * if at > len
  ///
  /// ## Example
  ///
  /// ```rust
  /// use tinyvec::*;
  /// let mut tv = tiny_vec!([i32; 4], 1, 2, 3);
  /// let tv2 = tv.split_off(1);
  /// assert_eq!(tv.as_slice(), &[1][..]);
  /// assert_eq!(tv2.as_slice(), &[2, 3][..]);
  /// ```
  #[inline]
  pub fn split_off(&mut self, at: usize) -> Self
  where
    A: Default,
  {
    match self {
      TinyVec::Inline(a) => TinyVec::Inline(a.split_off(at)),
      TinyVec::Heap(v) => TinyVec::Heap(v.split_off(at)),
    }
  }

  /// Remove an element, swapping the end of the vec into its place.
  ///
  /// ## Panics
  /// * If the index is out of bounds.
  ///
  /// ## Example
  /// ```rust
  /// use tinyvec::*;
  /// let mut tv = tiny_vec!([&str; 4], "foo", "bar", "quack", "zap");
  ///
  /// assert_eq!(tv.swap_remove(1), "bar");
  /// assert_eq!(tv.as_slice(), &["foo", "zap", "quack"][..]);
  ///
  /// assert_eq!(tv.swap_remove(0), "foo");
  /// assert_eq!(tv.as_slice(), &["quack", "zap"][..]);
  /// ```
  #[inline]
  pub fn swap_remove(&mut self, index: usize) -> A::Item {
    match self {
      TinyVec::Inline(a) => a.swap_remove(index),
      TinyVec::Heap(v) => v.swap_remove(index),
    }
  }

  /// Reduces the vec's length to the given value.
  /// 
  /// If the vec is already shorter than the input, nothing happens.
  #[inline]
  pub fn truncate(&mut self, new_len: usize) {
    match self {
      TinyVec::Inline(a) => a.truncate(new_len),
      TinyVec::Heap(v) => v.truncate(new_len),
    }
  }

  /// Wraps an array, using the given length as the starting length.
  ///
  /// If you want to use the whole length of the array, you can just use the
  /// `From` impl.
  ///
  /// ## Failure
  ///
  /// If the given length is greater than the capacity of the array this will
  /// error, and you'll get the array back in the `Err`.
  #[inline]
  pub fn try_from_array_len(data: A, len: usize) -> Result<Self, A> {
    let arr = ArrayVec::try_from_array_len(data, len)?;
    Ok(TinyVec::Inline(arr))
  }

  // LATER: try_push ?

  // LATER: try_insert ?

  // LATER: try_remove ?
}

/// Draining iterator for `TinyVecDrain`
/// 
/// See [`TinyVecDrain::drain`](TinyVecDrain::<A>::drain)
pub struct TinyVecDrain<'p, A: Array> {
  parent: &'p mut TinyVec<A>,
  target_index: usize,
  target_count: usize,
}
// GoodFirstIssue: this entire type is correct but slow.
// NIGHTLY: vec_drain_as_slice, https://github.com/rust-lang/rust/issues/58957
impl<'p, A: Array> Iterator for TinyVecDrain<'p, A> {
  type Item = A::Item;
  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    if self.target_count > 0 {
      let out = self.parent.remove(self.target_index);
      self.target_count -= 1;
      Some(out)
    } else {
      None
    }
  }
}
impl<'p, A: Array> Drop for TinyVecDrain<'p, A> {
  #[inline]
  fn drop(&mut self) {
    for _ in self {}
  }
}

impl<A: Array> AsMut<[A::Item]> for TinyVec<A> {
  #[inline(always)]
  #[must_use]
  fn as_mut(&mut self) -> &mut [A::Item] {
    &mut *self
  }
}

impl<A: Array> AsRef<[A::Item]> for TinyVec<A> {
  #[inline(always)]
  #[must_use]
  fn as_ref(&self) -> &[A::Item] {
    &*self
  }
}

impl<A: Array> Borrow<[A::Item]> for TinyVec<A> {
  #[inline(always)]
  #[must_use]
  fn borrow(&self) -> &[A::Item] {
    &*self
  }
}

impl<A: Array> BorrowMut<[A::Item]> for TinyVec<A> {
  #[inline(always)]
  #[must_use]
  fn borrow_mut(&mut self) -> &mut [A::Item] {
    &mut *self
  }
}

impl<A: Array> Extend<A::Item> for TinyVec<A> {
  #[inline]
  fn extend<T: IntoIterator<Item = A::Item>>(&mut self, iter: T) {
    for t in iter {
      self.push(t)
    }
  }
}

impl<A: Array> From<ArrayVec<A>> for TinyVec<A> {
  #[inline(always)]
  #[must_use]
  fn from(arr: ArrayVec<A>) -> Self {
    TinyVec::Inline(arr)
  }
}

impl<A: Array + Default> FromIterator<A::Item> for TinyVec<A> {
  #[inline]
  #[must_use]
  fn from_iter<T: IntoIterator<Item = A::Item>>(iter: T) -> Self {
    let mut av = Self::default();
    for i in iter {
      av.push(i)
    }
    av
  }
}

/// Iterator for consuming an `TinyVec` and returning owned elements.
pub enum TinyVecIterator<A: Array> {
  #[allow(missing_docs)]
  Inline(ArrayVecIterator<A>),
  #[allow(missing_docs)]
  Heap(alloc::vec::IntoIter<A::Item>)
}
impl<A: Array> Iterator for TinyVecIterator<A> {
  type Item = A::Item;
  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      TinyVecIterator::Inline(a) => a.next(),
      TinyVecIterator::Heap(v) => v.next(),
    }
  }
  #[inline(always)]
  #[must_use]
  fn size_hint(&self) -> (usize, Option<usize>) {
    match self {
      TinyVecIterator::Inline(a) => a.size_hint(),
      TinyVecIterator::Heap(v) => v.size_hint(),
    }
  }
  #[inline(always)]
  fn count(self) -> usize {
    match self {
      TinyVecIterator::Inline(a) => a.count(),
      TinyVecIterator::Heap(v) => v.count(),
    }
  }
  #[inline]
  fn last(self) -> Option<Self::Item> {
    match self {
      TinyVecIterator::Inline(a) => a.last(),
      TinyVecIterator::Heap(v) => v.last(),
    }
  }
  #[inline]
  fn nth(&mut self, n: usize) -> Option<A::Item> {
    match self {
      TinyVecIterator::Inline(a) => a.nth(n),
      TinyVecIterator::Heap(v) => v.nth(n),
    }
  }
}

impl<A: Array> IntoIterator for TinyVec<A> {
  type Item = A::Item;
  type IntoIter = TinyVecIterator<A>;
  #[inline(always)]
  #[must_use]
  fn into_iter(self) -> Self::IntoIter {
    match self {
      TinyVec::Inline(a) => TinyVecIterator::Inline(a.into_iter()),
      TinyVec::Heap(v) => TinyVecIterator::Heap(v.into_iter()),
    }
  }
}

impl<A: Array> PartialEq for TinyVec<A>
where
  A::Item: PartialEq,
{
  #[inline]
  #[must_use]
  fn eq(&self, other: &Self) -> bool {
    self.deref().eq(other.deref())
  }
}
impl<A: Array> Eq for TinyVec<A> where A::Item: Eq {}

impl<A: Array> PartialOrd for TinyVec<A>
where
  A::Item: PartialOrd,
{
  #[inline]
  #[must_use]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    self.deref().partial_cmp(other.deref())
  }
}
impl<A: Array> Ord for TinyVec<A>
where
  A::Item: Ord,
{
  #[inline]
  #[must_use]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.deref().cmp(other.deref())
  }
}

impl<A: Array> PartialEq<&A> for TinyVec<A>
where
  A::Item: PartialEq,
{
  #[inline]
  #[must_use]
  fn eq(&self, other: &&A) -> bool {
    self.deref() == other.slice()
  }
}

impl<A: Array> PartialEq<&[A::Item]> for TinyVec<A>
where
  A::Item: PartialEq,
{
  #[inline]
  #[must_use]
  fn eq(&self, other: &&[A::Item]) -> bool {
    self.deref() == *other
  }
}

/*

I think, in retrospect, this is useless?

The `&mut [A::Item]` should coerce to `&[A::Item]` and use the above impl.
I'll leave it here for now though since we already had it written out..

impl<A: Array> PartialEq<&mut [A::Item]> for TinyVec<A>
where
  A::Item: PartialEq,
{
  #[inline]
  #[must_use]
  fn eq(&self, other: &&mut [A::Item]) -> bool {
    self.deref() == *other
  }
}
*/

// //
// Formatting impls
// //

impl<A: Array> Binary for TinyVec<A>
where
  A::Item: Binary,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      Binary::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> Debug for TinyVec<A>
where
  A::Item: Debug,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> Display for TinyVec<A>
where
  A::Item: Display,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      Display::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> LowerExp for TinyVec<A>
where
  A::Item: LowerExp,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      LowerExp::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> LowerHex for TinyVec<A>
where
  A::Item: LowerHex,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      LowerHex::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> Octal for TinyVec<A>
where
  A::Item: Octal,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      Octal::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> Pointer for TinyVec<A>
where
  A::Item: Pointer,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      Pointer::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> UpperExp for TinyVec<A>
where
  A::Item: UpperExp,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      UpperExp::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}

impl<A: Array> UpperHex for TinyVec<A>
where
  A::Item: UpperHex,
{
  #[allow(clippy::missing_inline_in_public_items)]
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    write!(f, "[")?;
    for (i, elem) in self.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      UpperHex::fmt(elem, f)?;
    }
    write!(f, "]")
  }
}
