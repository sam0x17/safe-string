# safe-string

[![Crates.io](https://img.shields.io/crates/v/safe-string)](https://crates.io/crates/safe-string)
[![docs.rs](https://img.shields.io/docsrs/safe-string?label=docs)](https://docs.rs/safe-string/latest/safe-string/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/safe-string/ci.yaml)](https://github.com/sam0x17/safe-string/actions/workflows/ci.yaml?query=branch%3Amain)
[![MIT License](https://img.shields.io/github/license/sam0x17/safe-string)](https://github.com/sam0x17/safe-string/blob/main/LICENSE)

This crate provides replacement types for `String` and `&str`(`str`) that allow for safe
indexing by character to avoid panics and the usual pitfalls of working with multi-byte UTF-8
characters, namely the scenario where the _byte length_ of a string and the _character length_
of that same string are not the same.

Specifically, `IndexedString` (replaces `String`) and `IndexedSlice` (replaces `&str`) allow
for O(1) slicing and indexing by character, and they will never panic when indexing or slicing.

This is accomplished by storing the character offsets of each character in the string, along
with the original `String`, and using this information to calculate the byte offsets of each
character on the fly. Thus `IndexedString` uses ~2x the memory of a normal `String`, but
`IndexedSlice` and other types implementing `IndexedStr` have only one `usize` extra in
overhead over that of a regular `&str` slice / fat pointer. In theory this could be reduced
down to the same size as a fat pointer using unsafe rust, but this way we get to have
completely safe code and the difference is negligible.

## Examples

```rust
use safe_string::{IndexedString, IndexedStr, IndexedSlice};

let message = IndexedString::from("Hello, 世界! 👋😊");
assert_eq!(message.as_str(), "Hello, 世界! 👋😊");
assert_eq!(message, "Hello, 世界! 👋😊"); // handy PartialEq impls

// Access characters by index
assert_eq!(message.char_at(7), Some('世'));
assert_eq!(message.char_at(100), None); // Out of bounds access returns None

// Slice the IndexedString
let slice = message.slice(7..9);
assert_eq!(slice.as_str(), "世界");

// Convert slice back to IndexedString
let sliced_message = slice.to_indexed_string();
assert_eq!(sliced_message.as_str(), "世界");

// Nested slicing
let slice = message.slice(0..10);
let nested_slice = slice.slice(3..6);
assert_eq!(nested_slice.as_str(), "lo,");

// Display byte length and character length
assert_eq!(IndexedString::from_str("世界").byte_len(), 6); // "世界" is 6 bytes in UTF-8
assert_eq!(IndexedString::from_str("世界").len(), 2); // "世界" has 2 characters

// Demonstrate clamped slicing (no panic)
let clamped_slice = message.slice(20..30);
assert_eq!(clamped_slice.as_str(), "");

// Using `as_str` to interface with standard Rust string handling
let slice = message.slice(0..5);
let standard_str_slice = slice.as_str();
assert_eq!(standard_str_slice, "Hello");
```
