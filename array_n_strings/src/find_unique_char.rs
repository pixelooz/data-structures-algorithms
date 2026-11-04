pub fn first_uniq_char(s: String) -> i32 {
    let mut freq = [0; 26];
    for key in s.as_bytes() {
        freq[(*key - b'a') as usize] += 1;
    }
    for (idx, key) in s.as_bytes().iter().enumerate() {
        if freq[(*key - b'a') as usize] == 1 {
            return idx as i32;
        }
    }
    -1
}

#[test]
fn test_first_uniq_char() {
    let word = "leetcode";
    let result = first_uniq_char(word.to_string());
    assert!(result == 0);
}
