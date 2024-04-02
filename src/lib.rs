//! This crate provides replacement types for [`String`] and [`&str`](`str`) that allow for safe
//! indexing by character to avoid panics and the usual pitfalls of working with multi-byte
//! UTF-8 characters, namely the scenario where the _byte length_ of a string and the
//! _character length_ of that same string are not the same.
//!
//! Specifically, [`IndexedString`] (replaces [`String`]) and [`IndexedSlice`] (replaces
//! [`&str`](`str`)) allow for O(1) slicing and indexing by character, and they will never panic
//! when indexing or slicing.
//!
//! This is accomplished by storing the character offsets of each character in the string,
//! along with the original [`String`], and using this information to calculate the byte
//! offsets of each character on the fly. Thus [`IndexedString`] uses ~2x the memory of a
//! normal [`String`], but [`IndexedSlice`] and other types implementing [`IndexedStr`] have
//! only one [`usize`] extra in overhead over that of a regular [`&str`](`str`) slice / fat
//! pointer. In theory this could be reduced down to the same size as a fat pointer using
//! unsafe rust, but this way we get to have completely safe code and the difference is
//! negligible.
//! # Examples
//!
//! ```
//! use safe_string::{IndexedString, IndexedStr, IndexedSlice};
//!
//! let message = IndexedString::from("Hello, 世界! 👋😊");
//! assert_eq!(message.as_str(), "Hello, 世界! 👋😊");
//! assert_eq!(message, "Hello, 世界! 👋😊"); // handy PartialEq impls
//!
//! // Access characters by index
//! assert_eq!(message.char_at(7), Some('世'));
//! assert_eq!(message.char_at(100), None); // Out of bounds access returns None
//!
//! // Slice the IndexedString
//! let slice = message.slice(7..9);
//! assert_eq!(slice.as_str(), "世界");
//!
//! // Convert slice back to IndexedString
//! let sliced_message = slice.to_indexed_string();
//! assert_eq!(sliced_message.as_str(), "世界");
//!
//! // Nested slicing
//! let slice = message.slice(0..10);
//! let nested_slice = slice.slice(3..6);
//! assert_eq!(nested_slice.as_str(), "lo,");
//!
//! // Display byte length and character length
//! assert_eq!(IndexedString::from_str("世界").byte_len(), 6); // "世界" is 6 bytes in UTF-8
//! assert_eq!(IndexedString::from_str("世界").len(), 2); // "世界" has 2 characters
//!
//! // Demonstrate clamped slicing (no panic)
//! let clamped_slice = message.slice(20..30);
//! assert_eq!(clamped_slice.as_str(), "");
//!
//! // Using `as_str` to interface with standard Rust string handling
//! let slice = message.slice(0..5);
//! let standard_str_slice = slice.as_str();
//! assert_eq!(standard_str_slice, "Hello");
//! ```

use core::fmt::{Debug, Display};
use core::ops::{Bound, Index, RangeBounds};

/// A trait that facilitates safe interaction with strings that contain multi-byte characters.
///
/// [`IndexedString`] replaces [`String`], whereas [`IndexedSlice`] replaces [`&str`](`str`).
///
/// Both of these types as well as anything that implements [`IndexedStr`] are characterized by
/// the fact that their `len()` and indexing methods operate on characters, not bytes, and
/// enough information is stored to allow for O(1) slicing and indexing on a character _and_
/// byte basis as needed, but the default interface is character-centric.
///
/// This all comes at the cost of increased memory usage and some performance overhead when a
/// [`IndexedString`] is created, but the overhead is minimal when using [`IndexedSlice`] or
/// any other type implementing [`IndexedStr`].
///
/// It is also worth noting that all of these types will never panic when indexing or slicing,
/// unlike [`&str`](`str`) and [`String`], and clamped bounds are used instead.
pub trait IndexedStr:
    Display + Debug + PartialEq<IndexedString> + for<'a> PartialEq<IndexedSlice<'a>> + Index<usize>
{
    /// Returns a [`&str`](`str`) representation of this [`IndexedStr`].
    ///
    /// WARNING: Once you cast to a [`&str`](`str`), you are leaving the safety provided by
    /// [`IndexedStr`]. Only use this method when you need to interface with code that requires
    /// a [`&str`](`str`).
    fn as_str(&self) -> &str;

    /// Returns the length of this [`IndexedStr`] in characters, NOT bytes.
    fn len(&self) -> usize;

    /// Returns the byte length of this [`IndexedStr`]. This will be longer than the
    /// [`len`](`IndexedStr::len`) if the string contains multi-byte characters.
    fn byte_len(&self) -> usize;

    /// Returns the character at the given index, if it exists.
    fn char_at(&self, index: usize) -> Option<char>;

    /// Returns a sub-slice of this [`IndexedStr`] based on the given range in terms of the
    /// _characters_ in the string, not bytes.
    ///
    /// The range is automatically clamped to the bounds of the [`IndexedStr`].
    fn slice<R: RangeBounds<usize>>(&self, range: R) -> IndexedSlice;

    /// Returns a slice containing all characters of this [`IndexedStr`] in order.
    fn chars(&self) -> &[char];

    /// Converts this [`IndexedStr`] into an owned, dynamically allocated [`IndexedString`].
    fn to_indexed_string(&self) -> IndexedString;
}

/// A [`String`] replacement that allows for safe indexing and slicing of multi-byte characters.
///
/// This is the owned counterpart to [`IndexedSlice`].
#[derive(Clone, Debug, Eq, Hash)]
pub struct IndexedString {
    chars: Vec<char>,
    offsets: Vec<usize>,
    string: String,
}

impl IndexedStr for IndexedString {
    fn as_str(&self) -> &str {
        &self.string
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.chars.get(index).copied()
    }

    fn chars(&self) -> &[char] {
        &self.chars[..]
    }

    fn len(&self) -> usize {
        self.chars.len()
    }

    fn byte_len(&self) -> usize {
        self.string.len()
    }

    fn slice<R: RangeBounds<usize>>(&self, range: R) -> IndexedSlice {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.chars.len(),
        };
        let start = if start > self.chars.len() {
            self.chars.len()
        } else {
            start
        };
        let end = if end > self.chars.len() {
            self.chars.len()
        } else {
            end
        };

        IndexedSlice {
            source: self,
            start,
            end,
        }
    }

    fn to_indexed_string(&self) -> IndexedString {
        self.clone()
    }
}

impl IndexedString {
    /// Creates a new [`IndexedString`] from a `&str` or anything that implements [`AsRef<str>`].
    pub fn from_str(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        let chars: Vec<char> = s.chars().collect();
        let offsets: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
        IndexedString {
            chars,
            offsets,
            string: s.to_string(),
        }
    }

    /// Creates a new [`IndexedString`] from an iterator of [`char`]s.
    pub fn from_chars(chars: impl Iterator<Item = char>) -> Self {
        let chars: Vec<char> = chars.collect();
        let offsets: Vec<usize> = chars.iter().enumerate().map(|(i, _)| i).collect();
        let string: String = chars.iter().collect();
        IndexedString {
            chars,
            offsets,
            string,
        }
    }
}

impl AsRef<str> for IndexedString {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

impl Index<usize> for IndexedString {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl Display for IndexedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<S: AsRef<str>> PartialEq<S> for IndexedString {
    fn eq(&self, other: &S) -> bool {
        self.string == other.as_ref()
    }
}

/// A [`&str`](`str`) replacement that allows for safe indexing and slicing of multi-byte characters.
///
/// This is the borrowed counterpart to [`IndexedString`].
#[derive(Eq, Debug, Clone)]
pub struct IndexedSlice<'a> {
    source: &'a IndexedString,
    start: usize,
    end: usize,
}

impl<'a> IndexedStr for IndexedSlice<'a> {
    fn as_str(&self) -> &str {
        if self.start >= self.source.offsets.len()
            || self.end > self.source.offsets.len()
            || self.start > self.end
        {
            return "";
        }

        let start_byte = self.source.offsets[self.start];
        let end_byte = if self.end == self.source.offsets.len() {
            self.source.string.len()
        } else {
            self.source.offsets[self.end]
        };

        &self.source.string[start_byte..end_byte]
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn byte_len(&self) -> usize {
        self.source.offsets[self.end] - self.source.offsets[self.start]
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.source.char_at(self.start + index)
    }

    fn slice<R: RangeBounds<usize>>(&self, range: R) -> IndexedSlice {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };
        let start = if start > self.len() {
            self.len()
        } else {
            start
        };
        let end = if end > self.len() { self.len() } else { end };

        IndexedSlice {
            source: self.source,
            start: self.start + start,
            end: self.start + end,
        }
    }

    fn chars(&self) -> &[char] {
        &self.source.chars[self.start..self.end]
    }

    fn to_indexed_string(&self) -> IndexedString {
        IndexedString::from_chars(self.chars().into_iter().copied())
    }
}

impl<'a, S: AsRef<str>> PartialEq<S> for IndexedSlice<'a> {
    fn eq(&self, other: &S) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<'a> AsRef<str> for IndexedSlice<'a> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a IndexedString> for IndexedSlice<'a> {
    fn from(s: &'a IndexedString) -> Self {
        IndexedSlice {
            source: s,
            start: 0,
            end: s.chars.len(),
        }
    }
}

impl From<String> for IndexedString {
    fn from(s: String) -> Self {
        IndexedString::from_str(&s)
    }
}

impl From<&str> for IndexedString {
    fn from(s: &str) -> Self {
        IndexedString::from_str(s)
    }
}

impl From<&String> for IndexedString {
    fn from(s: &String) -> Self {
        IndexedString::from_str(s)
    }
}

impl<'a> Display for IndexedSlice<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Index<usize> for IndexedSlice<'_> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.source.chars[self.start + index]
    }
}
