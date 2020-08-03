#![cfg(feature = "experimental_array_set")]

// This was contributed by user `dhardy`! Big thanks.

use super::{Array, ArrayVec};

/// An array-backed set
///
/// This set supports `O(n)` operations and has a fixed size, thus may fail to
/// insert items. The potential advantage is a *really* small size.
///
/// The item type must support `Default`.
#[derive(Clone, Default)]
pub struct ArraySet<A: Array>
where
  A::Item: Default + Eq,
{
  /// The underlying ArrayVec
  pub arr: ArrayVec<A>,
}

impl<A: Array> ArraySet<A>
where
  A::Item: Default + Eq,
{
  /// Returns the fixed capacity of the set
  #[inline]
  pub fn capacity(&self) -> usize {
    A::CAPACITY
  }

  /// Returns the number of elements in the set
  #[inline]
  pub fn len(&self) -> usize {
    self.arr.len()
  }

  /// Returns true when the set contains no elements
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Removes all elements
  #[inline]
  pub fn clear(&mut self) {
    self.arr.clear()
  }

  /// Iterate over all contents
  #[inline]
  pub fn iter(&self) -> core::slice::Iter<'_, A::Item> {
    self.arr.iter()
  }

  /// Check whether the set contains `elt`
  #[inline]
  pub fn contains<Q>(&self, elt: &Q) -> bool
  where
    A::Item: PartialEq<Q>,
  {
    self.get(elt).is_some()
  }

  /// Get a reference to a contained item matching `elt`
  pub fn get<Q>(&self, elt: &Q) -> Option<&A::Item>
  where
    A::Item: PartialEq<Q>,
  {
    self.iter().find(|&x| x == elt)
  }

  /// Remove an item matching `elt`, if any
  pub fn remove<Q>(&mut self, elt: &Q) -> Option<A::Item>
  where
    A::Item: PartialEq<Q>,
  {
    let (n, _) = self.iter().enumerate().find(|(_, x)| *x == elt)?;
    Some(self.arr.remove(n))
  }

  /// Remove any items for which `f(item) == false`
  pub fn retain<F>(&mut self, f: F)
  where
    F: FnMut(&A::Item) -> bool,
  {
    self.arr.retain(f)
  }

  /// Insert an item
  ///
  /// Due to the fixed size of the backing array, insertion may fail.
  #[inline]
  pub fn try_insert(&mut self, elt: A::Item) -> Result<bool, A::Item> {
    if self.contains(&elt) {
      return Ok(false);
    }

    return match self.arr.try_push(elt) {
      Some(x) => Err(x),
      None => Ok(true),
    };
  }

  /* Hits borrow checker
  pub fn get_or_insert(&mut self, elt: A::Item) -> Result<&A::Item, InsertError> {
      if let Some(r) = self.get(&elt) {
          return Ok(r);
      }
      self.insert(elt)?;
      let len: usize = self.len.into();
      Ok(&self.arr.as_slice()[len - 1])
  }
  */

  /// Replace an item matching `elt` with `elt`, or insert `elt`
  ///
  /// Returns the replaced item, if any. Fails when there is no matching item
  /// and the backing array is full, preventing insertion.
  pub fn try_replace(
    &mut self,
    elt: A::Item,
  ) -> Result<Option<A::Item>, A::Item> {
    if let Some(x) = self.arr.iter_mut().find(|x| *x == &elt) {
      return Ok(Some(core::mem::replace(x, elt)));
    }

    return match self.arr.try_push(elt) {
      Some(x) => Err(x),
      None => Ok(None),
    };
  }

  /// Same as `try_insert`, but unwraps for you
  pub fn insert(&mut self, elt: A::Item) -> bool {
    match self.try_insert(elt) {
      Err(_) => {
        panic!("ArraySet::insert> tried to insert, but capacity is exhausted")
      }
      Ok(x) => x,
    }
  }
  /// Same as `try_replace`, but unwraps for you
  pub fn replace(&mut self, elt: A::Item) -> Option<A::Item> {
    match self.try_replace(elt) {
      Err(_) => {
        panic!("ArraySet::replace> tried to replace, but capacity is exhausted")
      }
      Ok(x) => x,
    }
  }
}
