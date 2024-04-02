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
    assert_eq!(indexed_string[0], 'h');
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
    assert_eq!(indexed_string[0], 'a');
    assert_eq!(indexed_string.as_str(), "a");
    assert_eq!(indexed_string.len(), 1);
}

#[test]
fn test_multibyte_characters() {
    let indexed_string: IndexedString = "😊🌍".into();
    assert_eq!(indexed_string[0], '😊');
    assert_eq!(indexed_string[1], '🌍');
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
