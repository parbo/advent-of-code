//! An implementation of a set of ascii chars
//!

// Adapted from https://github.com/contain-rs/bit-set

use core::cmp;
use core::fmt;
use core::iter::FromIterator;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;

type Set = [u64; 4];

#[derive(Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct AsciiSet {
    set: Set,
}

impl FromIterator<char> for AsciiSet {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

impl<'a> FromIterator<&'a char> for AsciiSet {
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

impl Extend<char> for AsciiSet {
    #[inline]
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }
}

impl<'a> Extend<&'a char> for AsciiSet {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a char>>(&mut self, iter: I) {
        for i in iter {
            self.insert(*i);
        }
    }
}

impl BitAnd for AsciiSet {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.intersect_with(&rhs);
        self
    }
}

impl BitOr for AsciiSet {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.union_with(&rhs);
        self
    }
}

impl BitXor for AsciiSet {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self.symmetric_difference_with(&rhs);
        self
    }
}

impl AsciiSet {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl AsciiSet {
    #[inline]
    fn other_op<F>(&mut self, other: &Self, mut f: F)
    where
        F: FnMut(u64, u64) -> u64,
    {
        for (i, w) in other.set.iter().enumerate() {
            let old = self.set[i];
            let new = f(old, *w);
            self.set[i] = new;
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter {
        Iter(AsciiIter { c: 0, s: &self.set })
    }

    #[inline]
    pub fn union(&self, other: &Self) -> Union {
        fn or(w1: u64, w2: u64) -> u64 {
            w1 | w2
        }

        Union(ByteIter::from_set(TwoBytes {
            set: self.set.into_iter(),
            other: other.set.into_iter(),
            merge: or,
        }))
    }

    #[inline]
    pub fn intersection(&self, other: &Self) -> Intersection {
        fn bitand(w1: u64, w2: u64) -> u64 {
            w1 & w2
        }

        Intersection(ByteIter::from_set(TwoBytes {
            set: self.set.into_iter(),
            other: other.set.into_iter(),
            merge: bitand,
        }))
    }

    #[inline]
    pub fn difference(&self, other: &Self) -> Difference {
        fn diff(w1: u64, w2: u64) -> u64 {
            w1 & !w2
        }

        Difference(ByteIter::from_set(TwoBytes {
            set: self.set.into_iter(),
            other: other.set.into_iter(),
            merge: diff,
        }))
    }

    #[inline]
    pub fn symmetric_difference(&self, other: &Self) -> SymmetricDifference {
        fn bitxor(w1: u64, w2: u64) -> u64 {
            w1 ^ w2
        }

        SymmetricDifference(ByteIter::from_set(TwoBytes {
            set: self.set.into_iter(),
            other: other.set.into_iter(),
            merge: bitxor,
        }))
    }

    #[inline]
    pub fn union_with(&mut self, other: &Self) {
        self.other_op(other, |w1, w2| w1 | w2);
    }

    #[inline]
    pub fn intersect_with(&mut self, other: &Self) {
        self.other_op(other, |w1, w2| w1 & w2);
    }

    #[inline]
    pub fn difference_with(&mut self, other: &Self) {
        self.other_op(other, |w1, w2| w1 & !w2);
    }

    #[inline]
    pub fn symmetric_difference_with(&mut self, other: &Self) {
        self.other_op(other, |w1, w2| w1 ^ w2);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.set
            .iter()
            .fold(0, |acc, n| acc + n.count_ones() as usize)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.set.iter().all(|x| *x == 0)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.set = Set::default();
    }

    /// Returns `true` if this set contains the specified char.
    #[inline]
    pub fn contains(&self, value: char) -> bool {
        if let Ok(c) = u8::try_from(value) {
            let mask = 1 << (c % 64);
            let v = self.set[(c / 64) as usize];
            (v & mask) != 0
        } else {
            false
        }
    }

    /// Returns `true` if the set has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).next().is_none()
    }

    /// Returns `true` if the set is a subset of another.
    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        self.set
            .iter()
            .zip(other.set.iter())
            .all(|(w1, w2)| *w1 & *w2 == *w1)
    }

    /// Returns `true` if the set is a superset of another.
    #[inline]
    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    /// Adds a value to the set. Returns `true` if the value was not already
    /// present in the set.
    pub fn insert(&mut self, value: char) -> bool {
        if let Ok(c) = u8::try_from(value) {
            let mask = 1 << (c % 64);
            let v = &mut self.set[(c / 64) as usize];
            let old = *v & mask;
            *v |= mask;
            old != (*v & mask)
        } else {
            false
        }
    }

    /// Removes a value from the set. Returns `true` if the value was
    /// present in the set.
    pub fn remove(&mut self, value: char) -> bool {
        if let Ok(c) = u8::try_from(value) {
            let mask = 1 << (c % 64);
            let v = &mut self.set[(c / 64) as usize];
            let old = *v & mask;
            *v &= !mask;
            old != (*v & mask)
        } else {
            false
        }
    }
}

impl fmt::Debug for AsciiSet {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.iter()).finish()
    }
}

#[derive(Clone)]
pub struct AsciiIter<'a> {
    c: u8,
    s: &'a Set,
}

impl<'a> Iterator for AsciiIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while self.c < 255 {
            let byte = self.c / 64;
            let mask = 1 << (self.c % 64);
            let c = self.c as char;
            self.c += 1;
            if self.s[byte as usize] & mask != 0 {
                return Some(c);
            }
        }
        None
    }
}

#[derive(Clone)]
struct ByteIter<T> {
    head: u64,
    head_offset: u8,
    tail: T,
}

impl<T> ByteIter<T>
where
    T: Iterator<Item = u64>,
{
    fn from_set(mut set: T) -> ByteIter<T> {
        let h = set.next().unwrap_or_default();
        ByteIter {
            tail: set,
            head: h,
            head_offset: 0,
        }
    }
}

/// An iterator combining two `AsciiSet` iterators.
#[derive(Clone)]
struct TwoBytes {
    set: std::array::IntoIter<u64, 4>,
    other: std::array::IntoIter<u64, 4>,
    merge: fn(u64, u64) -> u64,
}

/// An iterator for `BitSet`.
#[derive(Clone)]
pub struct Iter<'a>(AsciiIter<'a>);
#[derive(Clone)]
pub struct Union(ByteIter<TwoBytes>);
#[derive(Clone)]
pub struct Intersection(ByteIter<TwoBytes>);
#[derive(Clone)]
pub struct Difference(ByteIter<TwoBytes>);
#[derive(Clone)]
pub struct SymmetricDifference(ByteIter<TwoBytes>);

impl<T> Iterator for ByteIter<T>
where
    T: Iterator<Item = u64>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        while self.head == 0 {
            match self.tail.next() {
                Some(w) => self.head = w,
                None => return None,
            }
            self.head_offset += 64;
        }

        // from the current byte, isolate the
        // LSB and subtract 1, producing k:
        // a byte with a number of set bits
        // equal to the index of the LSB
        let k = (self.head & (!self.head + 1)) - 1;
        // update byte, removing the LSB
        self.head = self.head & (self.head - 1);
        // return offset + (index of LSB)
        Some((self.head_offset + (k.count_ones() as u8)) as char)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.tail.size_hint() {
            (_, Some(h)) => (0, Some(1 + h * 8)),
            _ => (0, None),
        }
    }
}

impl Iterator for TwoBytes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match (self.set.next(), self.other.next()) {
            (Some(a), Some(b)) => Some((self.merge)(a, b)),
            (Some(a), None) => Some((self.merge)(a, 0)),
            (None, Some(b)) => Some((self.merge)(0, b)),
            _ => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a, au) = self.set.size_hint();
        let (b, bu) = self.other.size_hint();

        let upper = match (au, bu) {
            (Some(au), Some(bu)) => Some(cmp::max(au, bu)),
            _ => None,
        };

        (cmp::max(a, b), upper)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl Iterator for Union {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl Iterator for Intersection {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl Iterator for Difference {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl Iterator for SymmetricDifference {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> IntoIterator for &'a AsciiSet {
    type Item = char;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::AsciiSet;
    use core::cmp::Ordering::{Equal, Greater, Less};
    use std::vec::Vec;

    #[test]
    fn test_bit_set_show() {
        let mut s = AsciiSet::new();
        s.insert('a');
        s.insert('k');
        s.insert('z');
        s.insert('d');
        assert_eq!("{'a', 'd', 'k', 'z'}", format!("{:?}", s));
    }

    #[test]
    fn test_ascii_set_basic() {
        let mut b = AsciiSet::new();
        assert!(b.insert('c'));
        assert!(!b.insert('c'));
        assert!(b.contains('c'));
        assert!(b.insert('d'));
        assert!(!b.insert('d'));
        assert!(b.contains('c'));
        assert!(b.insert('x'));
        assert!(!b.insert('x'));
        assert!(b.contains('x'));
        assert_eq!(b.len(), 3);
    }

    #[test]
    fn test_ascii_set_iterator() {
        let mut b = AsciiSet::new();
        assert!(b.insert('c'));
        assert!(b.insert('x'));
        assert!(b.insert('d'));
        assert_eq!(b.iter().collect::<Vec<_>>(), vec!['c', 'd', 'x']);
    }

    #[test]
    fn test_ascii_set_intersection() {
        let mut a = AsciiSet::new();
        let mut b = AsciiSet::new();
        assert!(a.insert('a'));
        assert!(a.insert('c'));
        assert!(a.insert('f'));
        assert!(a.insert('k'));
        assert!(a.insert('m'));
        assert!(a.insert('x'));
        assert!(a.insert('g'));
        assert!(a.insert('t'));
        assert!(a.insert('z'));

        assert!(b.insert('a'));
        assert!(b.insert('f'));
        assert!(b.insert('r'));
        assert!(b.insert('k'));
        assert!(b.insert('p'));

        let expected = ['a', 'f', 'k'];
        let actual: Vec<_> = a.intersection(&b).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ascii_set_difference() {
        let mut a = AsciiSet::new();
        let mut b = AsciiSet::new();
        assert!(a.insert('a'));
        assert!(a.insert('c'));
        assert!(a.insert('f'));
        assert!(a.insert('k'));
        assert!(a.insert('m'));
        assert!(a.insert('x'));
        assert!(a.insert('g'));
        assert!(a.insert('t'));
        assert!(a.insert('z'));

        assert!(b.insert('a'));
        assert!(b.insert('f'));
        assert!(b.insert('r'));
        assert!(b.insert('k'));
        assert!(b.insert('p'));

        let expected = ['c', 'g', 'm', 't', 'x', 'z'];
        let actual: Vec<_> = a.difference(&b).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ascii_set_symmetric_difference() {
        let mut a = AsciiSet::new();
        let mut b = AsciiSet::new();
        assert!(a.insert('a'));
        assert!(a.insert('c'));
        assert!(a.insert('f'));
        assert!(a.insert('k'));
        assert!(a.insert('m'));
        assert!(a.insert('x'));
        assert!(a.insert('g'));
        assert!(a.insert('t'));
        assert!(a.insert('z'));

        assert!(b.insert('a'));
        assert!(b.insert('f'));
        assert!(b.insert('r'));
        assert!(b.insert('k'));
        assert!(b.insert('p'));

        let expected = ['c', 'g', 'm', 'p', 'r', 't', 'x', 'z'];
        let actual: Vec<_> = a.symmetric_difference(&b).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ascii_set_union() {
        let mut a = AsciiSet::new();
        let mut b = AsciiSet::new();
        assert!(a.insert('a'));
        assert!(a.insert('c'));
        assert!(a.insert('f'));
        assert!(a.insert('k'));
        assert!(a.insert('m'));
        assert!(a.insert('x'));
        assert!(a.insert('g'));
        assert!(a.insert('t'));
        assert!(a.insert('z'));

        assert!(b.insert('a'));
        assert!(b.insert('f'));
        assert!(b.insert('r'));
        assert!(b.insert('k'));
        assert!(b.insert('p'));

        let expected = ['a', 'c', 'f', 'g', 'k', 'm', 'p', 'r', 't', 'x', 'z'];
        let actual: Vec<_> = a.union(&b).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ascii_set_subset() {
        let mut set1 = AsciiSet::new();
        let mut set2 = AsciiSet::new();

        assert!(set1.is_subset(&set2)); //  {}  {}
        set2.insert('a');
        assert!(set1.is_subset(&set2)); //  {}  { 'a' }
        set2.insert('b');
        assert!(set1.is_subset(&set2)); //  {}  { 'a', 'b' }
        set1.insert('b');
        assert!(set1.is_subset(&set2)); //  { 'b' }  { 'a', 'b' }
        set1.insert('c');
        assert!(!set1.is_subset(&set2)); // { 'b', 'c' }  { 'a', 'b' }
        set2.insert('c');
        assert!(set1.is_subset(&set2)); // { 'b', 'c' }  { 'a', 'b', 'c' }
        set2.insert('d');
        assert!(set1.is_subset(&set2)); // { 'b', 'c' }  { 'a', 'b', 'c', 'd' }
        set2.remove('a');
        assert!(set1.is_subset(&set2)); // { 'b', 'c' }  { 'b', 'c', 'd' }
        set2.remove('c');
        assert!(!set1.is_subset(&set2)); // { 'b', 'c' }  { 'b', 'd' }
        set1.remove('c');
        assert!(set1.is_subset(&set2)); // { 'b' }  { 'b', 'd' }
    }

    #[test]
    fn test_bit_set_is_disjoint() {
        let a = AsciiSet::from_iter(['a', 'c', 'g']);
        let b = AsciiSet::from_iter(['b']);
        let c = AsciiSet::new();
        let d = AsciiSet::from_iter(['c', 'd']);

        assert!(!a.is_disjoint(&d));
        assert!(!d.is_disjoint(&a));

        assert!(a.is_disjoint(&b));
        assert!(a.is_disjoint(&c));
        assert!(b.is_disjoint(&a));
        assert!(b.is_disjoint(&c));
        assert!(c.is_disjoint(&a));
        assert!(c.is_disjoint(&b));
    }

    #[test]
    fn test_ascii_set_union_with() {
        let mut a = AsciiSet::new();
        a.insert('a');
        let mut b = AsciiSet::new();
        b.insert('f');
        let expected = AsciiSet::from_iter(['a', 'f']);
        a.union_with(&b);
        assert_eq!(a, expected);

        let mut a = AsciiSet::from_iter(['a', 'c', 'g']);
        let mut b = AsciiSet::from_iter(['b', 'c', 'g']);
        let c = a.clone();
        a.union_with(&b);
        b.union_with(&c);
        assert_eq!(a.len(), 4);
        assert_eq!(b.len(), 4);
    }

    #[test]
    fn test_ascii_set_intersect_with() {
        let mut a = AsciiSet::from_iter(['a', 'c']);
        let mut b = AsciiSet::new();
        let c = a.clone();
        a.intersect_with(&b);
        b.intersect_with(&c);
        assert!(a.is_empty());
        assert!(b.is_empty());

        let mut a = AsciiSet::from_iter(['a', 'c', 'g']);
        let mut b = AsciiSet::from_iter(['b', 'c', 'g']);
        let c = a.clone();
        a.intersect_with(&b);
        b.intersect_with(&c);
        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 2);
    }

    #[test]
    fn test_ascii_set_difference_with() {
        let mut a = AsciiSet::new();
        let b = AsciiSet::from_iter(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
        a.difference_with(&b);
        assert!(a.is_empty());

        let mut a = AsciiSet::from_iter(['a', 'c', 'g']);
        let mut b = AsciiSet::from_iter(['b', 'c', 'g']);
        let c = a.clone();
        a.difference_with(&b);
        b.difference_with(&c);
        assert_eq!(a.len(), 1);
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn test_ascii_set_symmetric_difference_with() {
        let mut a = AsciiSet::new();
        a.insert('a');
        a.insert('b');
        let mut b = AsciiSet::new();
        b.insert('b');
        b.insert('g');
        let expected = AsciiSet::from_iter(['a', 'g']);
        a.symmetric_difference_with(&b);
        assert_eq!(a, expected);

        let mut a = AsciiSet::from_iter(['a', 'c', 'g']);
        let b = AsciiSet::new();
        let c = a.clone();
        a.symmetric_difference_with(&b);
        assert_eq!(a, c);

        let mut a = AsciiSet::from_iter(['a', 'c', 'g']);
        let mut b = AsciiSet::from_iter(['b', 'c', 'g']);
        let c = a.clone();
        a.symmetric_difference_with(&b);
        b.symmetric_difference_with(&c);
        assert_eq!(a.len(), 2);
        assert_eq!(b.len(), 2);
    }

    #[test]
    fn test_ascii_set_bitand() {
        let a = AsciiSet::from_iter(['a', 'c', 'g']);
        let b = AsciiSet::from_iter(['b', 'c', 'g']);
        let c = a & b;
        assert_eq!(c, AsciiSet::from_iter(['c', 'g']));
    }

    #[test]
    fn test_ascii_set_bitor() {
        let a = AsciiSet::from_iter(['a', 'c', 'g']);
        let b = AsciiSet::from_iter(['b', 'c', 'g']);
        let c = a | b;
        assert_eq!(c, AsciiSet::from_iter(['a', 'b', 'c', 'g']));
    }

    #[test]
    fn test_ascii_set_bitxor() {
        let a = AsciiSet::from_iter(['a', 'c', 'g']);
        let b = AsciiSet::from_iter(['b', 'c', 'g']);
        let c = a ^ b;
        assert_eq!(c, AsciiSet::from_iter(['a', 'b']));
    }

    #[test]
    fn test_ascii_set_eq() {
        let a = AsciiSet::from_iter(['a', 'c']);
        let b = AsciiSet::from_iter(&[]);
        let c = AsciiSet::new();

        assert!(a == a);
        assert!(a != b);
        assert!(a != c);
        assert!(b == b);
        assert!(b == c);
        assert!(c == c);
    }

    #[test]
    fn test_ascii_set_cmp() {
        let a = AsciiSet::from_iter(['a', 'c', 'g']);
        let b = AsciiSet::from_iter(&[]);
        let c = AsciiSet::new();

        assert_eq!(a.cmp(&b), Greater);
        assert_eq!(a.cmp(&c), Greater);
        assert_eq!(b.cmp(&a), Less);
        assert_eq!(b.cmp(&c), Equal);
        assert_eq!(c.cmp(&a), Less);
        assert_eq!(c.cmp(&b), Equal);
    }
}
