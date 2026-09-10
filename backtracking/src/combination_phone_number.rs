use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: &[&[u8]] = &[
            b"", b"", b"abc", b"def", b"ghi", b"jkl", b"mno", b"pqrs", b"tuv", b"wxyz",
        ];
        let mut result = Vec::new();
        let mut path = String::with_capacity(digits.len());
        Self::letter_combinations_dfs(digits.as_bytes(), 0, map, &mut path, &mut result);
        result
    }

    fn letter_combinations_dfs(
        digits: &[u8],
        index: usize,
        map: &[&[u8]],
        path: &mut String,
        result: &mut Vec<String>,
    ) {
        if index == digits.len() {
            result.push(path.clone());
            return;
        }
        let digit_idx = (digits[index] - b'0') as usize;
        let letters = map[digit_idx];

        for &byte in letters {
            path.push(byte as char);
            Self::letter_combinations_dfs(digits, index + 1, map, path, result);
            path.pop();
        }
    }
}
