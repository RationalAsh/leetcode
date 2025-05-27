pub struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut v, (idx, word)| {
                if word.contains(x) {
                    v.push(idx as i32);
                }
                v
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    /// Helper to convert a vector of `&str` into `Vec<String>`
    fn vs(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_both_match() {
        let words = vs(&["leet", "code"]);
        assert_eq!(Solution::find_words_containing(words, 'e'), vec![0, 1]);
    }

    #[test]
    fn example_one_match() {
        let words = vs(&["abc", "def"]);
        assert_eq!(Solution::find_words_containing(words, 'f'), vec![1]);
    }

    #[test]
    fn no_match_returns_empty() {
        let words = vs(&["abc", "def"]);
        assert!(Solution::find_words_containing(words, 'z').is_empty());
    }

    #[test]
    fn handles_empty_input_vector() {
        let words: Vec<String> = Vec::new();
        assert!(Solution::find_words_containing(words, 'a').is_empty());
    }

    #[test]
    fn handles_empty_string_element() {
        let words = vs(&["", "alpha"]);
        assert_eq!(Solution::find_words_containing(words, 'a'), vec![1]);
    }

    #[test]
    fn multiple_occurrences_same_word() {
        // Index should appear only once even if `x` occurs many times.
        let words = vs(&["aaaaa", "b"]);
        assert_eq!(Solution::find_words_containing(words, 'a'), vec![0]);
    }

    #[test]
    fn case_sensitive_characters() {
        // Containment is case-sensitive: 'A' ≠ 'a'.
        let words = vs(&["Apple", "banana", "Apricot"]);
        assert_eq!(
            Solution::find_words_containing(words.clone(), 'A'),
            vec![0, 2]
        );
        assert_eq!(Solution::find_words_containing(words, 'a'), vec![1]);
    }
}
