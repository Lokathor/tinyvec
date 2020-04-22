use core::{marker::PhantomData, mem::replace};

/// A trait for types that can have a "placeholder" value safely constructed.
///
/// This is _similar to_ the [`Default`] trait. The difference is that while
/// some types don't `impl Default` because it might not make sense to declare
/// any particular value the default of that type, for the purposes of `tinyvec`
/// we just need _any placeholder value at all_. We're not even going to use the
/// value, we're just going to have it occupy the memory locations of the unused
/// portions of a vector. Because we really, definitely don't care what the
/// value is, it's easier to just pick something.
///
/// Further, some types don't select a default because that would imply that
/// there's a default usable value when there's not really. Using this goofy
/// custom trait helps keep that misunderstanding to a minimum if you want to
/// use your own types with this crate.
///
/// **However, to be clear:** It is still possible for a user to create values
/// via this trait and then get their hands on those value using only safe code.
/// Whatever value is generated via this trait's impl must be fully safe for
/// users to have.
pub trait Placeholder: Sized {
  /// This constructs the placeholder value.
  ///
  /// The input is a zero-sized marker type. Only the `tinyvec` crate can make
  /// this marker type so it's the only crate that will call this method
  /// directly. However, it's possible for an end user to get sentinel values
  /// anyway (eg: `ArrayVec::set_len`), so you must still ensure that your
  /// placeholder value is safe for them to have.
  fn placeholder(marker: PlaceholderMarker<Self>) -> Self;
  // Note: Thanks to https://github.com/Lej77, they came up with this neat idea
  // for using a private marker type to limit who can call this method.
}

/// A ZST marker type so that only the `tinyvec` crate can call the
/// `Placeholder::placeholder` method.
pub struct PlaceholderMarker<T> {
  marker: PhantomData<T>,
}

/// Only functions that are in this module can construct a [`SentinelMarker`]
/// and therefore call the `Sentinel` trait.
#[inline(always)]
pub(crate) fn new_placeholder<T: Placeholder>() -> T {
  T::placeholder(PlaceholderMarker { marker: PhantomData })
}

#[inline(always)]
pub(crate) fn take_placeholder<T: Placeholder>(from: &mut T) -> T {
  replace(from, new_placeholder())
}

impl<T> Placeholder for T
where
  T: Default,
{
  fn placeholder(_: PlaceholderMarker<Self>) -> Self {
    Self::default()
  }
}
