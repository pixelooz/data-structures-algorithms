use crate::Solution;

impl Solution {
    pub fn partition(strs: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        let str_bytes = strs.as_bytes();
        Self::partition_dfs(str_bytes, 0, &mut path, &mut result);
        result
    }

    fn partition_dfs<'a>(
        bytes: &'a [u8],
        start: usize,
        path: &mut Vec<&'a str>,
        result: &mut Vec<Vec<String>>,
    ) {
        if start == bytes.len() {
            result.push(path.iter().map(|&s| s.to_string()).collect());
            return;
        }
        for end in start + 1..=bytes.len() {
            if Self::check_palindrome_partition(&bytes[start..end]) {
                let valid_str = unsafe { str::from_utf8_unchecked(&bytes[start..end]) };
                path.push(valid_str);
                Self::partition_dfs(bytes, end, path, result);
                path.pop();
            }
        }
    }

    fn check_palindrome_partition(strs: &[u8]) -> bool {
        strs.iter()
            .zip(strs.iter().rev())
            .take(strs.len() / 2)
            .all(|(a, b)| a == b)
    }
}
