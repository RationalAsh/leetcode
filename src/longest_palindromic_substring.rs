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

    // Insert locations of each unique character into a hahsmap.
    s.chars().enumerate().fold(&mut charmap, |map, (idx, ch)| {
        let entry = map.entry(ch).or_insert(Vec::new());
        entry.push(idx);

        map
    });

    // Find the relevant min idx and max idx.
    let (min_idx, max_idx) =
        charmap
            .iter()
            .fold((0 as usize, 1 as usize), |(l, r), (_ch, locs)| {
                if locs.len() > 1 {
                    let ranges = get_ranges(locs);

                    ranges
                        .iter()
                        .fold((l, r), |(ml, mr), (_range_s, _range_e)| {
                            if is_palindrom(&s, (_range_s, _range_e)) {
                                if (mr - ml) < (*_range_e - *_range_s) {
                                    (*_range_s, *_range_e)
                                } else {
                                    (ml, mr)
                                }
                            } else {
                                (ml, mr)
                            }
                        })
                } else {
                    (l, r)
                }
            });

    todo!()
}

fn get_ranges(locs: &Vec<usize>) -> Vec<(usize, usize)> {
    // We iterate over the indixes
    locs.iter()
        .enumerate()
        .fold(Vec::new(), |mut ranges, (idx, &loc)| {
            let _ranges = locs[idx..(locs.len() - 1)]
                .iter()
                .fold(Vec::new(), |mut v, &l| {
                    v.push((loc, l));
                    v
                });
            for r in _ranges {
                ranges.push(r);
            }
            ranges
        })
}

fn is_palindrom(s: &String, range: (&usize, &usize)) -> bool {
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
