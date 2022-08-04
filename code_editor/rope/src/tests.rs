use {super::*, proptest::prelude::*};

fn arbitrary_string() -> impl Strategy<Value = String> {
    ".*"
}

fn arbitrary_string_and_byte_index() -> impl Strategy<Value = (String, usize)> {
    arbitrary_string()
        .prop_flat_map(|string| {
            let string_len = string.len();
            (Just(string), 0..=string_len)
        })
        .prop_map(|(string, mut index)| {
            while !string.is_char_boundary(index) {
                index -= 1;
            }
            (string, index)
        })
}

fn arbitrary_string_and_byte_range() -> impl Strategy<Value = (String, Range<usize>)> {
    arbitrary_string_and_byte_index()
        .prop_flat_map(|(string, end)| (Just(string), 0..=end, Just(end)))
        .prop_map(|(string, mut start, end)| {
            while !string.is_char_boundary(start) {
                start -= 1;
            }
            (string, start..end)
        })
}

fn arbitrary_string_and_byte_range_and_byte_index() -> impl Strategy<Value = (String, Range<usize>, usize)> {
    arbitrary_string_and_byte_range()
        .prop_flat_map(|(string, range)| {
            let range_len = range.len();
            (Just(string), Just(range), 0..=range_len)
        }).prop_map(|(string, range, mut index)| {
            let slice = &string[range.clone()];
            while !slice.is_char_boundary(index) {
                index -= 1;
            }
            (string, range, index)
        })
}

proptest! {
    #[test]
    fn is_empty(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(rope.is_empty(), string.is_empty());
    }

    #[test]
    fn byte_len(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(rope.byte_len(), string.len());
    }

    #[test]
    fn char_len(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(rope.char_len(), string.chars().count());
    }

    #[test]
    fn char_at_byte((string, byte_index) in arbitrary_string_and_byte_index()) {
        let rope = Rope::from(&string);
        assert_eq!(rope.char_at_byte(byte_index), string[..byte_index].chars().count());
    }

    #[test]
    fn chunks(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(rope.chunks().collect::<String>(), string);
    }

    #[test]
    fn chunks_rev(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(
            rope
                .chunks_rev()
                .flat_map(|chunk| chunk.chars().rev())
                .collect::<String>(),
            string.chars().rev().collect::<String>(),
        );
    }

    #[test]
    fn bytes(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(
            rope.bytes().collect::<Vec<_>>(),
            string.bytes().collect::<Vec<_>>()
        );
    }

    #[test]
    fn bytes_rev(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(
            rope.bytes_rev().collect::<Vec<_>>(),
            string.bytes().rev().collect::<Vec<_>>()
        );
    }

    #[test]
    fn chars(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(
            rope.chars().collect::<Vec<_>>(),
            string.chars().collect::<Vec<_>>()
        );
    }

    #[test]
    fn chars_rev(string in arbitrary_string()) {
        let rope = Rope::from(&string);
        assert_eq!(
            rope.chars_rev().collect::<Vec<_>>(),
            string.chars().rev().collect::<Vec<_>>()
        );
    }

    #[test]
    fn append(mut string_0 in arbitrary_string(), string_1 in arbitrary_string()) {
        let mut rope_0 = Rope::from(&string_0);
        let rope_1 = Rope::from(&string_1);
        rope_0.append(rope_1);
        string_0.push_str(&string_1);
        assert_eq!(rope_0.chunks().collect::<String>(), string_0);
    }

    #[test]
    fn split_off((mut string, byte_index) in arbitrary_string_and_byte_index()) {
        let mut rope = Rope::from(&string);
        let other_rope = rope.split_off(byte_index);
        let other_string = string.split_off(byte_index);
        assert_eq!(rope.chunks().collect::<String>(), string);
        assert_eq!(other_rope.chunks().collect::<String>(), other_string);
    }

    #[test]
    fn truncate_front((mut string, byte_start) in arbitrary_string_and_byte_index()) {
        let mut rope = Rope::from(&string);
        rope.truncate_front(byte_start);
        string.replace_range(..byte_start, "");
        assert_eq!(rope.chunks().collect::<String>(), string);
    }

    #[test]
    fn truncate_back((mut string, byte_end) in arbitrary_string_and_byte_index()) {
        let mut rope = Rope::from(&string);
        rope.truncate_back(byte_end);
        string.truncate(byte_end);
        assert_eq!(rope.chunks().collect::<String>(), string);
    }

    #[test]
    fn slice_is_empty((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(rope_slice.is_empty(), string_slice.is_empty());
    }

    #[test]
    fn slice_byte_len((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(rope_slice.byte_len(), string_slice.len());
    }
    
    #[test]
    fn slice_char_len((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(rope_slice.char_len(), string_slice.chars().count());
    }

    #[test]
    fn slice_char_at_byte((string, byte_range, byte_index) in arbitrary_string_and_byte_range_and_byte_index()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(rope_slice.char_at_byte(byte_index), string_slice[..byte_index].chars().count());
    }

    #[test]
    fn slice_chunks((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(rope_slice.chunks().collect::<String>(), string_slice);
    }

    #[test]
    fn slice_chunks_rev((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(
            rope_slice
                .chunks_rev()
                .flat_map(|chunk| chunk.chars().rev())
                .collect::<String>(),
            string_slice.chars().rev().collect::<String>(),
        );
    }

    #[test]
    fn slice_bytes((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(
            rope_slice.bytes().collect::<Vec<_>>(),
            string_slice.bytes().collect::<Vec<_>>()
        );
    }

    #[test]
    fn slice_bytes_rev((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(
            rope_slice.bytes_rev().collect::<Vec<_>>(),
            string_slice.bytes().rev().collect::<Vec<_>>()
        );
    }

    #[test]
    fn slice_chars((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(
            rope_slice.chars().collect::<Vec<_>>(),
            string_slice.chars().collect::<Vec<_>>()
        );
    }

    #[test]
    fn slice_chars_rev((string, byte_range) in arbitrary_string_and_byte_range()) {
        let string_slice = &string[byte_range.clone()];
        let rope = Rope::from(&string);
        let rope_slice = rope.slice(byte_range);
        assert_eq!(
            rope_slice.chars_rev().collect::<Vec<_>>(),
            string_slice.chars().rev().collect::<Vec<_>>()
        );
    }
}
