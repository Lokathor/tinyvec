/// A trait for types that are an array.
///
/// An "array", for our purposes, has the following properties:
/// * Owns some number of elements.
/// * The element type can be generic, but must implement [`Default`].
/// * The capacity is fixed at compile time, based on the implementing type.
/// * You can get a shared or mutable slice to the elements.
///
/// You are generally **not** expected to need to implement this yourself. It is
/// already implemented for all the major array lengths (`0..=32` and the powers
/// of 2 up to 4,096), or for all array lengths in Rust versions 1.55 and newer.
///
/// **Additional lengths can easily be added upon request for Rust 1.54 and
/// older.**
///
/// ## Safety Reminder
///
/// Just a reminder: this trait is 100% safe, which means that `unsafe` code
/// **must not** rely on an instance of this trait being correct.
pub trait Array {
  /// The type of the items in the thing.
  type Item: Default;

  /// The number of slots in the thing.
  const CAPACITY: usize;

  /// Gives a shared slice over the whole thing.
  ///
  /// A correct implementation will return a slice with a length equal to the
  /// `CAPACITY` value.
  fn as_slice(&self) -> &[Self::Item];

  /// Gives a unique slice over the whole thing.
  ///
  /// A correct implementation will return a slice with a length equal to the
  /// `CAPACITY` value.
  fn as_slice_mut(&mut self) -> &mut [Self::Item];

  /// Create a default-initialized instance of ourself, similar to the
  /// [`Default`] trait, but implemented for the same range of sizes as
  /// [`Array`].
  fn default() -> Self;
}

// These `*_impl` modules implement `Array` for primitive arrays.
//
// NOTE(2022-07-09): The `#[rustversion::...]` conditional compilation
// attributes are placed on the individual implementation blocks rather than on
// the modules because using procedural attribute macros on non-inline modules
// is unstable.  Even if doing so becomes stable, it would be incompatible with
// `tinyvec`'s MSRV.
mod const_generic_impl;
mod generated_impl;
