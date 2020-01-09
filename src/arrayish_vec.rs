use super::*;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ArrayishVec<A: Arrayish> {
  len: usize,
  data: A,
}

impl<A: Arrayish> Deref for ArrayishVec<A> {
  type Target = [A::Item];
  #[inline(always)]
  #[must_use]
  fn deref(&self) -> &Self::Target {
    &self.data.slice()[..self.len]
  }
}

impl<A: Arrayish> DerefMut for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data.slice_mut()[..self.len]
  }
}

impl<A: Arrayish, I: SliceIndex<[A::Item]>> Index<I> for ArrayishVec<A> {
  type Output = <I as SliceIndex<[A::Item]>>::Output;
  #[inline(always)]
  #[must_use]
  fn index(&self, index: I) -> &Self::Output {
    &self.deref()[index]
  }
}

impl<A: Arrayish, I: SliceIndex<[A::Item]>> IndexMut<I> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    &mut self.deref_mut()[index]
  }
}

impl<A: Arrayish> ArrayishVec<A> {
  #[inline]
  pub fn append(&mut self, other: &mut Self) {
    let final_len = self.len + other.len;
    if final_len > A::CAPACITY {
      panic!("ArrayishVec: overflow!");
    }
    let target_slice = &mut self.data.slice_mut()[self.len..final_len];
    for (target_mut, app_mut) in target_slice.iter_mut().zip(other.deref_mut())
    {
      replace(target_mut, replace(app_mut, A::Item::default()));
    }
    self.len = final_len;
    other.len = 0;
  }

  #[inline(always)]
  #[must_use]
  pub fn as_mut_ptr(&mut self) -> *mut A::Item {
    self.data.slice_mut().as_mut_ptr()
  }

  #[inline(always)]
  #[must_use]
  pub fn as_mut_slice(&mut self) -> &mut [A::Item] {
    self.deref_mut()
  }

  #[inline(always)]
  #[must_use]
  pub fn as_ptr(&self) -> *const A::Item {
    self.data.slice().as_ptr()
  }

  #[inline(always)]
  #[must_use]
  pub fn as_slice(&self) -> &[A::Item] {
    self.deref()
  }

  #[inline(always)]
  #[must_use]
  pub fn capacity(&self) -> usize {
    A::CAPACITY
  }

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

  // TODO(Vec): drain
  // TODO(Vec): drain_filter #nightly

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
        panic!("ArrayishVec: length {} exceeds capacity {}!", len, A::CAPACITY)
      }
    }
  }

  // TODO(Vec): insert

  #[inline(always)]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  #[inline(always)]
  #[must_use]
  pub fn len(&self) -> usize {
    self.len
  }

  #[inline(always)]
  #[must_use]
  pub fn new() -> Self
  where
    A: Default,
  {
    Self::default()
  }

  #[inline]
  pub fn pop(&mut self) -> Option<A::Item> {
    if self.len > 0 {
      self.len -= 1;
      let out =
        replace(&mut self.data.slice_mut()[self.len], A::Item::default());
      Some(out)
    } else {
      None
    }
  }

  #[inline(always)]
  pub fn push(&mut self, val: A::Item) {
    if self.try_push(val).is_err() {
      panic!("ArrayishVec: overflow!")
    }
  }

  // TODO(Vec): remove
  // TODO(Vec): remove_item
  // TODO(Vec): resize (val that's clone)
  // TODO(Vec): resize_with (fn that generates a new one each time)
  // TODO(Vec): retain
  // TODO(Vec): splice
  // TODO(Vec): split_off
  // TODO(Vec): swap_remove

  #[inline]
  pub fn truncate(&mut self, new_len: usize) {
    if needs_drop::<A::Item>() {
      while self.len > new_len {
        self.pop();
      }
    } else {
      self.len = new_len;
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
    if len <= A::CAPACITY {
      Ok(Self { data, len })
    } else {
      Err(data)
    }
  }

  /// Pushes an item if there's room.
  ///
  /// ## Failure
  ///
  /// If there's no more capacity the vec is unchanged, and you get the item
  /// back in the `Err`.
  #[inline]
  pub fn try_push(&mut self, val: A::Item) -> Result<(), A::Item> {
    if self.len < A::CAPACITY {
      replace(&mut self.data.slice_mut()[self.len], val);
      self.len += 1;
      Ok(())
    } else {
      Err(val)
    }
  }
}

impl<A: Arrayish> AsMut<[A::Item]> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  fn as_mut(&mut self) -> &mut [A::Item] {
    &mut *self
  }
}

impl<A: Arrayish> AsRef<[A::Item]> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  fn as_ref(&self) -> &[A::Item] {
    &*self
  }
}

impl<A: Arrayish> Borrow<[A::Item]> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  fn borrow(&self) -> &[A::Item] {
    &*self
  }
}

impl<A: Arrayish> BorrowMut<[A::Item]> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  fn borrow_mut(&mut self) -> &mut [A::Item] {
    &mut *self
  }
}

impl<A: Arrayish> Extend<A::Item> for ArrayishVec<A> {
  fn extend<T: IntoIterator<Item = A::Item>>(&mut self, iter: T) {
    for t in iter {
      self.push(t)
    }
  }
}

impl<A: Arrayish> From<A> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  /// The output has a length equal to the full array.
  ///
  /// If you want to select a length, use [`from_array_len`]
  fn from(data: A) -> Self {
    Self { len: data.slice().len(), data }
  }
}

impl<A: Arrayish + Default> FromIterator<A::Item> for ArrayishVec<A> {
  #[must_use]
  fn from_iter<T: IntoIterator<Item = A::Item>>(iter: T) -> Self {
    let mut av = Self::default();
    for i in iter {
      av.push(i)
    }
    av
  }
}

pub struct ArrayishVecIterator<A: Arrayish> {
  base: usize,
  len: usize,
  data: A,
}
impl<A: Arrayish> Iterator for ArrayishVecIterator<A> {
  type Item = A::Item;
  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    if self.base < self.len {
      let out =
        replace(&mut self.data.slice_mut()[self.base], A::Item::default());
      self.base += 1;
      Some(out)
    } else {
      None
    }
  }
  #[inline(always)]
  #[must_use]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let s = self.len - self.base;
    (s, Some(s))
  }
  #[inline(always)]
  fn count(self) -> usize {
    self.len - self.base
  }
  #[inline]
  fn last(mut self) -> Option<Self::Item> {
    Some(replace(&mut self.data.slice_mut()[self.len], A::Item::default()))
  }
  #[inline]
  fn nth(&mut self, n: usize) -> Option<A::Item> {
    let i = self.base + (n - 1);
    if i < self.len {
      let out = replace(&mut self.data.slice_mut()[i], A::Item::default());
      self.base = i + 1;
      Some(out)
    } else {
      None
    }
  }
}

impl<A: Arrayish> IntoIterator for ArrayishVec<A> {
  type Item = A::Item;
  type IntoIter = ArrayishVecIterator<A>;
  #[inline(always)]
  #[must_use]
  fn into_iter(self) -> Self::IntoIter {
    ArrayishVecIterator { base: 0, len: self.len, data: self.data }
  }
}

impl<A: Arrayish> PartialEq for ArrayishVec<A>
where
  A::Item: PartialEq,
{
  #[inline]
  #[must_use]
  fn eq(&self, other: &Self) -> bool {
    self.deref().eq(other.deref())
  }
}
impl<A: Arrayish> Eq for ArrayishVec<A> where A::Item: Eq {}

impl<A: Arrayish> PartialOrd for ArrayishVec<A>
where
  A::Item: PartialOrd,
{
  #[inline]
  #[must_use]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    self.deref().partial_cmp(other.deref())
  }
}
impl<A: Arrayish> Ord for ArrayishVec<A>
where
  A::Item: Ord,
{
  #[inline]
  #[must_use]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.deref().cmp(other.deref())
  }
}

impl<A: Arrayish> PartialEq<&A> for ArrayishVec<A>
where
  A::Item: PartialEq,
{
  #[inline]
  #[must_use]
  fn eq(&self, other: &&A) -> bool {
    self.deref() == other.slice()
  }
}

impl<A: Arrayish> PartialEq<&[A::Item]> for ArrayishVec<A>
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

impl<A: Arrayish> PartialEq<&mut [A::Item]> for ArrayishVec<A>
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

impl<A: Arrayish> Binary for ArrayishVec<A>
where
  A::Item: Binary,
{
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

impl<A: Arrayish> Debug for ArrayishVec<A>
where
  A::Item: Debug,
{
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

impl<A: Arrayish> Display for ArrayishVec<A>
where
  A::Item: Display,
{
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

impl<A: Arrayish> LowerExp for ArrayishVec<A>
where
  A::Item: LowerExp,
{
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

impl<A: Arrayish> LowerHex for ArrayishVec<A>
where
  A::Item: LowerHex,
{
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

impl<A: Arrayish> Octal for ArrayishVec<A>
where
  A::Item: Octal,
{
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

impl<A: Arrayish> Pointer for ArrayishVec<A>
where
  A::Item: Pointer,
{
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

impl<A: Arrayish> UpperExp for ArrayishVec<A>
where
  A::Item: UpperExp,
{
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

impl<A: Arrayish> UpperHex for ArrayishVec<A>
where
  A::Item: UpperHex,
{
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
