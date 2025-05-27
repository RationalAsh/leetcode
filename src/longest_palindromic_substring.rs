use std::cmp::{max, min};
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        longest_palindromic_substring(s)
    }
}

fn longest_palindromic_substring(s: String) -> String {
    // Convert the string into a byte array for easy
    // iteration / working
    let mut charmap: HashMap<char, Vec<usize>> = HashMap::new();

    // Insert locations into map.
    s.chars().enumerate().fold(&mut charmap, |map, (idx, ch)| {
        let entry = map.entry(ch).or_insert(Vec::new());
        entry.push(idx);

        map
    });

    // Find the relevant min idx and max idx.
    let (min_idx, max_idx) = charmap
        .iter()
        .fold((0 as usize, s.len()), |(l, r), (_ch, locs)| {
            if locs.len() > 1 {
                let min_entry = locs.iter().min_by_key(|&&v| v).unwrap();
                let max_entry = locs.iter().max_by_key(|&&v| v).unwrap();

                (min(l, *min_entry), max(r, *max_entry))
            } else {
                (l, r)
            }
        });

    todo!()
}

// ---------- tests ----------
#[cfg(test)]
mod tests {
    use super::Solution;

    // Helper: confirms the returned string is
    //   • actually a substring,
    //   • a palindrome,
    //   • and at least as long as any known correct answer(s) we supply.
    fn validate(s: &str, cand: &str, refs: &[&str]) {
        // 1) substring
        assert!(
            s.contains(cand),
            "result '{cand}' is not a substring of input '{s}'"
        );
        // 2) palindrome
        assert!(
            cand.chars().eq(cand.chars().rev()),
            "result '{cand}' is not a palindrome"
        );
        // 3) longest
        let best_len = refs.iter().map(|r| r.len()).max().unwrap();
        assert!(
            cand.len() == best_len,
            "result '{cand}' len {} is not longest (expected {})",
            cand.len(),
            best_len
        );
    }

    #[test]
    fn example_odd_vs_even() {
        let s = "babad";
        let out = Solution::longest_palindrome(s.into());
        validate(s, &out, &["bab", "aba"]);
    }

    #[test]
    fn example_even() {
        let s = "cbbd";
        let out = Solution::longest_palindrome(s.into());
        assert_eq!(out, "bb");
    }

    #[test]
    fn single_char() {
        let s = "z";
        let out = Solution::longest_palindrome(s.into());
        assert_eq!(out, "z");
    }

    #[test]
    fn two_different_chars() {
        let s = "ab";
        let out = Solution::longest_palindrome(s.into());
        assert!(
            out == "a" || out == "b",
            "expected length-1 palindrome, got '{out}'"
        );
    }

    #[test]
    fn all_same_chars() {
        let s = "aaaa";
        let out = Solution::longest_palindrome(s.into());
        assert_eq!(out, "aaaa");
    }

    #[test]
    fn full_string_palindrome() {
        let s = "abccba";
        let out = Solution::longest_palindrome(s.into());
        assert_eq!(out, "abccba");
    }

    #[test]
    fn palindrome_in_middle() {
        let s = "forgeeksskeegfor";
        let out = Solution::longest_palindrome(s.into());
        assert_eq!(out, "geeksskeeg");
    }

    #[test]
    fn digits_and_letters() {
        let s = "123321456";
        let out = Solution::longest_palindrome(s.into());
        assert_eq!(out, "123321");
    }

    #[test]
    fn mixed_case() {
        // Case-sensitive: 'A' ≠ 'a', so longest is length-1
        let s = "Aa";
        let out = Solution::longest_palindrome(s.into());
        assert!(out == "A" || out == "a");
    }

    #[test]
    fn max_length_input() {
        // 1000 'a's ⇒ whole string is the answer
        let s = "a".repeat(1000);
        let out = Solution::longest_palindrome(s.clone());
        assert_eq!(out.len(), 1000);
    }
}
