use derive_arbitrary::Arbitrary;
use arbitrary_model_tests::arbitrary_stateful_operations;
use honggfuzz::fuzz;
use std::{fmt::Debug, iter::FromIterator, ops::{RangeBounds, Bound}};

use tinyvec::TinyVec;
use tinyvec_fuzz::ArbRange;

const CAPACITY: usize = 32;

arbitrary_stateful_operations! {
    model = Vec<T>,
    tested = TinyVec<[T; CAPACITY]>,

    type_parameters = <
        T: Default + Clone + Debug + Eq + Ord,
        R: RangeBounds<usize> + Clone + Debug,
    >,

    methods {
        equal {
            fn as_mut_slice(&mut self) -> &mut [T];
            fn as_slice(&self) -> &[T];
            fn clear(&mut self);
            fn dedup(&mut self);
            fn insert(&mut self, index: usize, item: T);
            fn is_empty(&self) -> bool;
            fn len(&self) -> usize;
            fn pop(&mut self) -> Option<T>;
            fn push(&mut self, item: T);
            fn remove(&mut self, index: usize) -> T;
            fn resize(&mut self, new_len: usize, new_val: T);
            fn swap_remove(&mut self, index: usize) -> T;
            fn truncate(&mut self, new_len: usize);
        }

        equal_with(Vec::from_iter) {
            fn split_off(&mut self, at: usize) -> impl IntoIterator<Item = T>;
            fn drain(&self, range: R) -> impl Iterator<Item = T>;
            fn iter(&self) -> impl Iterator<Item = &T>;
            fn iter_mut(&self) -> impl Iterator<Item = &mut T>;
        }
    }

    pre {
        match self {
            Self::insert { index, .. } | Self::split_off { at: index } if index > model.len() => {
                // TODO: Should test that these identically panic
                return;
            }
            Self::remove { index } | Self::swap_remove { index } if index >= model.len() => {
                // TODO: Should test that these identically panic
                return;
            }
            // Arbitrary limit to avoid allocating too large a buffer
            Self::resize { new_len, .. } if new_len > 4 * CAPACITY => {
                return;
            }
            Self::drain { ref range } => {
                // TODO: Should test that these identically panic
                let start = match range.start_bound() {
                    Bound::Included(&n) => n,
                    Bound::Excluded(&n) => n + 1,
                    Bound::Unbounded => 0,
                };
                let end = match range.end_bound() {
                    // If it's already usize::max, doesn't really matter about adding 1
                    Bound::Included(&n) => n.checked_add(1).unwrap_or(n),
                    Bound::Excluded(&n) => n,
                    Bound::Unbounded => model.len(),
                };
                if start > end || end > model.len() {
                    return;
                }
            }
            _ => {}
        }
    }
}

const MAX_RING_SIZE: usize = 16_384;

fn fuzz_cycle(data: &[u8]) -> Result<(), ()> {
    use arbitrary::{Arbitrary, FiniteBuffer};

    let mut ring = FiniteBuffer::new(&data, MAX_RING_SIZE).map_err(|_| ())?;

    let mut model = Vec::<u16>::default();
    let mut tested: TinyVec<[u16; 32]> = TinyVec::new();

    let mut _op_trace = String::new();
    while let Ok(op) = <op::Op<u16, ArbRange<usize>> as Arbitrary>::arbitrary(&mut ring) {
        #[cfg(fuzzing_debug)]
        _op_trace.push_str(&format!("{}\n", op.to_string()));
        op.execute_and_compare(&mut model, &mut tested);
    }

    Ok(())
}

fn main() -> Result<(), ()> {
    better_panic::install();

    loop {
        fuzz!(|data: &[u8]| {
            let _ = fuzz_cycle(data);
        });
    }
}
