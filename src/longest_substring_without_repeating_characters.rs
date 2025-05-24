pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        length_of_longest_substring(s)
    }
}

fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let chars = s.as_bytes();
    let mut l: usize = 0;
    let mut seen = HashMap::new();
    let mut answer = 0;

    if s.len() < 2 {
        return s.len() as i32;
    }

    for i in 0..s.len() {
        if let Some(old_value) = seen.insert(chars[i], i) {
            // Value was already present in the set.
            // So we remove all seen characters until this set.
            for j in l..(old_value) {
                if chars[j] != chars[i] {
                    seen.remove(&chars[j]);
                }
            }
            // Update the value of l.
            l = old_value + 1;
        } else {
            // Don't do anything
        }

        // Keep track of the largest set size.
        if answer < seen.len() {
            answer = seen.len();
        }
    }

    answer as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    // fn test_duplicates() {
    //     assert!(contains_duplicates("ss".as_bytes()))
    // }

    // ─────────────────────────────
    // 0-length & 1-length inputs
    // ─────────────────────────────
    #[test]
    fn empty_string() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn single_char() {
        assert_eq!(length_of_longest_substring("x".to_string()), 1);
    }

    // ─────────────────────────────
    // All-duplicate inputs
    // ─────────────────────────────
    #[test]
    fn all_same_characters_short() {
        assert_eq!(length_of_longest_substring("aaaa".to_string()), 1);
    }

    #[test]
    fn all_same_characters_long() {
        let big = "b".repeat(50_000); // upper size bound
        assert_eq!(length_of_longest_substring(big), 1);
    }

    // ─────────────────────────────
    // Fully unique input
    // ─────────────────────────────
    #[test]
    fn all_unique_chars() {
        assert_eq!(length_of_longest_substring("abcdef".to_string()), 6);
    }

    // ─────────────────────────────
    // Duplicates at various positions
    // ─────────────────────────────
    #[test]
    fn duplicate_at_start() {
        //            ↓ reset here
        assert_eq!(length_of_longest_substring("abcadef".to_string()), 6); // "bcadef"
    }

    #[test]
    fn duplicate_at_end() {
        assert_eq!(length_of_longest_substring("abcdefa".to_string()), 6); // "abcdef"
    }

    #[test]
    fn overlapping_duplicates() {
        assert_eq!(length_of_longest_substring("abba".to_string()), 2); // "ab"
    }

    #[test]
    fn interleaved_duplicates() {
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3); // "vdf"
    }

    #[test]
    fn example_from_prompt_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn example_from_prompt_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn example_from_prompt_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }

    // ─────────────────────────────
    // Inputs with spaces, digits & symbols
    // ─────────────────────────────
    #[test]
    fn includes_space_and_symbols() {
        assert_eq!(length_of_longest_substring("a b!c d@".to_string()), 6); // "b!c d"
    }

    #[test]
    fn alpha_numeric_mix() {
        assert_eq!(length_of_longest_substring("a1b2c3d4".to_string()), 8);
    }

    // ─────────────────────────────
    // Large cyclic alphabet – checks window shrink logic
    // ─────────────────────────────
    #[test]
    fn large_repeated_alphabet() {
        // one cycle of 62 distinct ASCII chars, then repeat it 800 times
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let s = alphabet.repeat(800);
        assert_eq!(length_of_longest_substring(s), alphabet.len() as i32); // 62
    }
}
