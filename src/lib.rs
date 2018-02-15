#![warn(missing_docs)]
//! A fairly trivial crate to allow a view into a specified number of columns of a slice.
//!
//! A lot like `std::slice::Chunks`, but transposed in how it iterates over the data.
//!
//! Mostly a note to self about how to do trait extensions.
use std::cmp;

/// An iterator over stripes of a Vec.
///
/// Analogous to the rust std module's `std::slice::Chunks`, but whereas `std::slice::Chunks` splits a Vec into n-length
/// `std::slice::Chunk` iterators, a `Stripes` is an iterator over n `Stripe` iterators of n-th values (offset
/// by the stripe index).
///
/// In other words, both `Chunks` and `Stripes` view a slice or a Vec as a table of
/// n-sized chunks, but Stripes reads this table column-wise rather than row-wise.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Stripes<'a, T:'a> {
    _ct: usize, // Can't optimise by just splitting off the next n pieces for each next().
    _skip: usize,
    _v: &'a [T],
}

impl<'a, T> Iterator for Stripes<'a, T> {
    type Item = Stripe<'a, T>;
    
    #[inline]
    fn next(&mut self) -> Option<Stripe<'a, T>> {
        let x = self._skip - self._ct;
        if x == 0 || self._v.is_empty()  {
            None
        } else {
            let ns = Stripe{ _v: self._v, _skip: self._skip };
            self._ct += 1;
            let (_fst, snd) = self._v.split_at(1);
            self._v = snd;
            Some(ns)
        }
    }
}

/// Extension trait for slices allowing striped iteration.
pub trait Striped<'a, T: 'a> {
    /// return an Stripes iterator on the underlying data.
    fn stripes(&'a self, skip: usize) -> Stripes<'a, T>;
}

impl<'a,T:'a> Stripes<'a, T> {
    /// Build a `Stripes` from a slice reference.
    pub fn from(sl: &'a [T], skip: usize) -> Stripes<'a, T> {
        Stripes {
            _v: sl,
            _skip: skip,
            _ct: 0
        }
    }
}

impl<'a, T: 'a> Striped<'a, T> for [T] {
    fn stripes(&'a self, skip: usize) -> Stripes<'a, T> {
        Stripes{_v: self, _skip: skip, _ct: 0}
    }
}

/// An iterator over mutable stripes of a slice/vec
pub struct StripesMut<'a, T:'a> {
    _s_sz: usize,
    _v: &'a mut [T],
}

/// An iterator with a view into a stripe of a slice/vec, returning every n-th value.
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Stripe<'a, T:'a> {
    _skip: usize,
    _v: &'a [T],
}

impl<'a, T> Iterator for Stripe<'a, T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self._v.first() {
            Some(n) => {
                let (_head,rst) = self._v.split_at(cmp::min(self._v.len(),self._skip));
                self._v = rst;
                Some(*n)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Stripes,Striped};
    #[test]
    fn test_stripes() {
        let x = [1,2,3,4,5,6,7,8,9u8];
        let mut s = x.stripes(3);
        let first = s.next().unwrap().collect::<Vec<u8>>();
        assert_eq!(first[2],7);
        let last = s.nth(1).unwrap().collect::<Vec<u8>>();
        assert_eq!(last[0], 3);
        assert_eq!(last[1], 6);
        assert_eq!(last[2], 9);

        let mut f = Stripes::from(&x, 4);
        let mut t = f.nth(2).unwrap();
        assert_eq!(t.nth(1),Some(7));
    }
}
