use safe_string::*;

#[test]
fn test_indexed_string_equality() {
    let indexed_string = IndexedString::from_str("h₳ello");
    assert_eq!(indexed_string, "h₳ello");
    assert_eq!(indexed_string.as_str(), "h₳ello");
    assert_eq!(indexed_string.to_string(), "h₳ello");
}

#[test]
fn test_from_chars() {
    let indexed_string = IndexedString::from_chars("h₳ello".chars());
    assert_eq!(indexed_string, "h₳ello");
    assert_eq!(indexed_string.as_str(), "h₳ello");
    assert_eq!(indexed_string.to_string(), "h₳ello");
}

#[test]
fn test_indexing() {
    let indexed_string = IndexedString::from_str("h₳ello");
    assert_eq!(indexed_string.char_at(0).unwrap(), 'h');
    assert_eq!(indexed_string.slice(1..4).as_str(), "₳el");
    assert_eq!(indexed_string.slice(4..), "lo");
}

#[test]
fn test_empty_string() {
    let indexed_string: IndexedString = (&String::from("")).into();
    assert_eq!(indexed_string.as_str(), "");
    assert!(indexed_string.char_at(0).is_none());
}

#[test]
fn test_single_character() {
    let indexed_string: IndexedString = String::from("a").into();
    assert_eq!(indexed_string.char_at(0).unwrap(), 'a');
    assert_eq!(indexed_string.as_str(), "a");
    assert_eq!(indexed_string.len(), 1);
}

#[test]
fn test_multibyte_characters() {
    let indexed_string: IndexedString = "😊🌍".into();
    assert_eq!(indexed_string.char_at(0).unwrap(), '😊');
    assert_eq!(indexed_string.char_at(1).unwrap(), '🌍');
    assert_eq!(indexed_string.slice(0..1), "😊");
    assert_eq!(indexed_string.len(), 2);
}

#[test]
fn test_out_of_bounds_indexing() {
    let indexed_string = IndexedString::from_str("test");
    assert!(indexed_string.char_at(10).is_none());
}

#[test]
fn test_reverse_range() {
    let indexed_string = IndexedString::from_str("hello");
    assert_eq!(indexed_string.slice(3..1), "");
}

#[test]
fn test_full_range() {
    let indexed_string = IndexedString::from_str("hello");
    assert_eq!(indexed_string.slice(0..5), "hello");
}

#[test]
fn test_adjacent_ranges() {
    let indexed_string = IndexedString::from_str("hello world");
    assert_eq!(indexed_string.slice(0..5), "hello");
    assert_eq!(indexed_string.slice(5..6), " ");
    assert_eq!(indexed_string.slice(6..11), "world");
}

#[test]
fn test_non_ascii_ranges() {
    let indexed_string = IndexedString::from_str("🎉🌍🚀");
    assert_eq!(indexed_string.slice(0..1), "🎉");
    assert_eq!(indexed_string.slice(1..3).as_str(), "🌍🚀");
}

#[test]
fn test_edge_case_ranges() {
    let indexed_string = IndexedString::from_str("abc");
    assert_eq!(indexed_string.slice(0..0), "");
    assert_eq!(indexed_string.slice(0..1), "a");
    assert_eq!(indexed_string.slice(2..3), "c");
    assert_eq!(indexed_string.slice(3..3), "");
}

#[test]
fn test_slicing_beyond_length() {
    let indexed_string = IndexedString::from_str("hello");
    let slice = indexed_string.slice(3..8);
    assert_eq!(slice.as_str(), "lo");
}

#[test]
fn test_nested_slices() {
    let indexed_string = IndexedString::from_str("hello world");
    let slice1 = indexed_string.slice(0..11);
    let slice2 = slice1.slice(6..11);
    assert_eq!(slice2.as_str(), "world");
}

#[test]
fn test_char_at() {
    let indexed_string = IndexedString::from_str("h₳ello");
    assert_eq!(indexed_string.char_at(1), Some('₳'));
    let slice = indexed_string.slice(1..5);
    assert_eq!(slice.char_at(0), Some('₳'));
}

#[test]
fn test_conversion_to_indexed_string() {
    let indexed_string = IndexedString::from_str("hello");
    let slice = indexed_string.slice(2..5);
    let converted = slice.to_indexed_string();
    assert_eq!(converted.as_str(), "llo");
    assert_eq!(converted, "llo");
}

#[test]
fn test_multibyte_character_boundaries() {
    let indexed_string = IndexedString::from_str("a😊bc");
    let slice = indexed_string.slice(1..3); // Should include the entire "😊"
    assert_eq!(slice.as_str(), "😊b");
    assert_eq!(slice.len(), 2);
    assert_eq!(indexed_string, indexed_string.as_slice());
}

#[test]
fn test_empty_slices() {
    let indexed_string = IndexedString::from_str("hello");
    let empty_slice = indexed_string.slice(3..3);
    assert!(empty_slice.as_str().is_empty());
    assert_eq!(empty_slice.len(), 0);
}

#[test]
fn test_varied_range_bounds() {
    let indexed_string = IndexedString::from_str("hello world");
    let slice = indexed_string.slice(6..);
    assert_eq!(slice.as_str(), "world");

    let slice = indexed_string.slice(..5);
    assert_eq!(slice.as_str(), "hello");

    let slice = indexed_string.slice(..=4);
    assert_eq!(slice.as_str(), "hello");
}

#[test]
fn test_overlap_slices() {
    let indexed_string = IndexedString::from_str("hello world");
    let slice1 = indexed_string.slice(0..7); // "hello w"
    let slice2 = indexed_string.slice(5..11); // " world"
    assert_eq!(slice1.as_str(), "hello w");
    assert_eq!(slice2.as_str(), " world");
}

#[test]
fn test_boundary_conditions_multibyte() {
    let indexed_string = IndexedString::from_str("a😊bc");
    for i in 0..indexed_string.len() {
        let slice = indexed_string.slice(i..i + 1);
        assert_eq!(slice.len(), 1);
        assert!(slice.as_str().chars().count() == 1);
    }
}

#[test]
fn test_repetitive_slicing() {
    let indexed_string = IndexedString::from_str("hello world");
    let slice1 = indexed_string.slice(0..11);
    let slice2 = slice1.slice(0..11);
    let slice3 = slice2.slice(0..11);
    assert_eq!(slice3.as_str(), "hello world");
}

#[test]
fn test_empty_slice_to_indexed_string() {
    let indexed_string = IndexedString::from_str("hello");
    let slice = indexed_string.slice(3..3);
    let converted = slice.to_indexed_string();
    assert!(converted.as_str().is_empty());
}

#[test]
fn test_slicing_each_character() {
    let indexed_string = IndexedString::from_str("h₳ello");
    for i in 0..indexed_string.len() {
        let slice = indexed_string.slice(i..i + 1);
        assert_eq!(slice.len(), 1);
        assert!(slice.to_indexed_string().len() == 1);
    }
}

#[test]
fn test_starts_and_ends_with() {
    let indexed_string = IndexedString::from_str("hello world");
    assert!(indexed_string.starts_with("hello"));
    assert!(indexed_string.ends_with("world"));
    assert!(!indexed_string.starts_with("world"));
    assert!(!indexed_string.ends_with("hello"));
    assert!(indexed_string.starts_with(IndexedString::from("hello")));
    assert!(indexed_string.ends_with(IndexedString::from("world")));
}

#[test]
fn test_slice_with_empty_start() {
    let indexed_string = IndexedString::from_str("hello");
    let slice = indexed_string.slice(..); // Equivalent to 0..len
    assert_eq!(slice.as_str(), "hello");
}

#[test]
fn test_empty_string_conversion() {
    let indexed_string: IndexedString = "".into();
    let slice = indexed_string.slice(..);
    assert_eq!(slice.as_str(), "");
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_from_chars_with_multibyte_characters() {
    let chars = vec!['a', '😊', 'b', '🌍', 'c'];
    let indexed_string = IndexedString::from_chars(chars.into_iter());

    // Verify the original string
    assert_eq!(indexed_string.as_str(), "a😊b🌍c");

    // Verify character indexing
    assert_eq!(indexed_string.char_at(0), Some('a'));
    assert_eq!(indexed_string.char_at(1), Some('😊'));
    assert_eq!(indexed_string.char_at(2), Some('b'));
    assert_eq!(indexed_string.char_at(3), Some('🌍'));
    assert_eq!(indexed_string.char_at(4), Some('c'));
    assert_eq!(indexed_string.char_at(5), None); // Out of bounds

    // Verify slicing
    assert_eq!(indexed_string.slice(0..1).as_str(), "a");
    assert_eq!(indexed_string.slice(1..2).as_str(), "😊");
    assert_eq!(indexed_string.slice(2..3).as_str(), "b");
    assert_eq!(indexed_string.slice(3..4).as_str(), "🌍");
    assert_eq!(indexed_string.slice(4..5).as_str(), "c");

    // Verify full slice
    assert_eq!(indexed_string.slice(0..5).as_str(), "a😊b🌍c");

    // Verify partial slice
    assert_eq!(indexed_string.slice(1..4).as_str(), "😊b🌍");

    // Verify empty slice
    assert_eq!(indexed_string.slice(2..2).as_str(), "");

    // Verify slice out of bounds
    assert_eq!(indexed_string.slice(5..10).as_str(), "");
}

#[test]
fn test_lines_single_line() {
    let indexed_string = IndexedString::from_str("Hello, world!");
    let lines: Vec<_> = indexed_string.lines().collect();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].as_str(), "Hello, world!");
}

#[test]
fn test_lines_multiple_lines() {
    let indexed_string = IndexedString::from_str("Hello, world!\nHow are you?\nI am fine.");
    let lines: Vec<_> = indexed_string.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0].as_str(), "Hello, world!");
    assert_eq!(lines[1].as_str(), "How are you?");
    assert_eq!(lines[2].as_str(), "I am fine.");
}

#[test]
fn test_lines_with_empty_lines() {
    let indexed_string = IndexedString::from_str("Hello, world!\n\nHow are you?\n\nI am fine.\n");
    let lines: Vec<_> = indexed_string.lines().collect();
    assert_eq!(lines.len(), 6);
    assert_eq!(lines[0].as_str(), "Hello, world!");
    assert_eq!(lines[1].as_str(), "");
    assert_eq!(lines[2].as_str(), "How are you?");
    assert_eq!(lines[3].as_str(), "");
    assert_eq!(lines[4].as_str(), "I am fine.");
    assert_eq!(lines[5].as_str(), "");
}

#[test]
fn test_lines_only_newlines() {
    let indexed_string = IndexedString::from_str("\n\n\n");
    let lines: Vec<_> = indexed_string.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0].as_str(), "");
    assert_eq!(lines[1].as_str(), "");
    assert_eq!(lines[2].as_str(), "");
    assert_eq!(lines[3].as_str(), "");
}

#[test]
fn test_lines_empty_string() {
    let indexed_string = IndexedString::from_str("");
    let lines: Vec<_> = indexed_string.lines().collect();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].as_str(), "");
}

#[test]
fn test_lines_multibyte_characters() {
    let indexed_string = IndexedString::from_str("Hello, 世界!\nこんにちは世界!\n👋😊");
    let lines: Vec<_> = indexed_string.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0].as_str(), "Hello, 世界!");
    assert_eq!(lines[1].as_str(), "こんにちは世界!");
    assert_eq!(lines[2].as_str(), "👋😊");
}
