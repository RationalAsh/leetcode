pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let symbol_to_num = HashMap::from([
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
        ]);

        let chars = s.chars();
    }
}
