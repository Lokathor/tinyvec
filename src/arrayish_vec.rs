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

impl<A: Arrayish> Index<usize> for ArrayishVec<A> {
  type Output = A::Item;
  #[inline(always)]
  #[must_use]
  fn index(&self, index: usize) -> &Self::Output {
    &self.deref()[index]
  }
}

impl<A: Arrayish> IndexMut<usize> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.deref_mut()[index]
  }
}

impl<A: Arrayish> ArrayishVec<A> {
  // TODO(Vec): append

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

  // TODO(Vec): dedup
  // TODO(Vec): dedup_by
  // TODO(Vec): dedup_by_key
  // TODO(Vec): drain
  // TODO(Vec): drain_filter #nightly
  // TODO(Vec): extend_from_slice

  #[inline]
  #[must_use]
  pub fn from_array_len(data: A, len: usize) -> Self {
    if len <= A::CAPACITY {
      Self { data, len }
    } else {
      panic!(
        "ArrayishVec: len {} is greater than capacity {}!",
        len,
        A::CAPACITY
      );
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

  #[inline]
  pub fn push(&mut self, val: A::Item) {
    if self.len < A::CAPACITY {
      replace(&mut self.data.slice_mut()[self.len], val);
      self.len += 1;
    } else {
      panic!("ArrayVec: overflow!")
    }
  }

  // TODO(Vec): remove
  // TODO(Vec): remove_item
  // TODO(Vec): resize
  // TODO(Vec): resize_default
  // TODO(Vec): resize_with
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

  // TODO: try_from_array_len
  // TODO: try_push
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

// TODO(Vec): Extend<&'a T>
// TODO(Vec): Extend<T>

impl<A: Arrayish> From<A> for ArrayishVec<A> {
  #[inline(always)]
  #[must_use]
  /// The output has a length equal to the full array.
  fn from(data: A) -> Self {
    Self { len: data.slice().len(), data }
  }
}

// TODO(Vec): From<&'a [T]>
// TODO(Vec): From<&'a mut [T]>
// TODO(Vec): From<&'_ str>
// TODO(Vec): From<&'a Vec<T>>
// TODO(Vec): From<BinaryHeap<T>>
// TODO(Vec): From<Box<[T]>>
// TODO(Vec): From<Cow<'a, [T]>>
// TODO(Vec): From<String>
// TODO(Vec): From<Vec<T>>
// TODO(Vec): From<VecDeque<T>>

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

// TODO(Vec): Index<I: SliceIndex>
// TODO(Vec): IndexMut<I: SliceIndex>

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
  #[inline(always)]
  #[must_use]
  fn eq(&self, other: &Self) -> bool {
    self.deref() == other.deref()
  }
}
impl<A: Arrayish> Eq for ArrayishVec<A> where A::Item: Eq {}

impl<A: Arrayish> PartialEq<&A> for ArrayishVec<A>
where
  A::Item: PartialEq,
{
  #[inline(always)]
  #[must_use]
  fn eq(&self, other: &&A) -> bool {
    self.deref() == other.slice()
  }
}

impl<A: Arrayish> PartialEq<&[A::Item]> for ArrayishVec<A>
where
  A::Item: PartialEq,
{
  #[inline(always)]
  #[must_use]
  fn eq(&self, other: &&[A::Item]) -> bool {
    self.deref() == *other
  }
}

impl<A: Arrayish> PartialEq<&mut [A::Item]> for ArrayishVec<A>
where
  A::Item: PartialEq,
{
  #[inline(always)]
  #[must_use]
  fn eq(&self, other: &&mut [A::Item]) -> bool {
    self.deref() == *other
  }
}

// TODO: PartialOrd, Ord, Hash

// //
// Formatting impls
// //

impl<A: Arrayish> Binary for ArrayishVec<A>
where
  A::Item: Binary,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        Binary::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> Debug for ArrayishVec<A>
where
  A::Item: Debug,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        Debug::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> Display for ArrayishVec<A>
where
  A::Item: Display,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        Display::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> LowerExp for ArrayishVec<A>
where
  A::Item: LowerExp,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        LowerExp::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> LowerHex for ArrayishVec<A>
where
  A::Item: LowerHex,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        LowerHex::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> Octal for ArrayishVec<A>
where
  A::Item: Octal,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        Octal::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> Pointer for ArrayishVec<A>
where
  A::Item: Pointer,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        Pointer::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> UpperExp for ArrayishVec<A>
where
  A::Item: UpperExp,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        UpperExp::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}

impl<A: Arrayish> UpperHex for ArrayishVec<A>
where
  A::Item: UpperHex,
{
  fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
    if self.len == 0 {
      write!(f, "[]")
    } else {
      write!(f, "[")?;
      let oxford = self.len - 1;
      for (i, elem) in self.iter().enumerate() {
        UpperHex::fmt(elem, f)?;
        if i < oxford {
          write!(f, ", ")?;
        }
      }
      write!(f, "]")
    }
  }
}
