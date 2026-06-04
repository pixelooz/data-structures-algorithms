pub fn valid_palindrome(word: String) -> bool {
    let bytes = word.as_bytes();
    let len = bytes.len();

    let is_palindrome = |l: usize, r: usize| (l..=r).all(|i| bytes[i] == bytes[r - (i - l)]);

    for i in 0..len / 2 {
        if bytes[i] != bytes[len - 1 - i] {
            return is_palindrome(i + 1, len - 1 - i) || is_palindrome(i, len - 2 - i);
        }
    }
    true
}

// Wrong Answer - does not fit all solutions.
pub fn _valid_palindrome(word: String) -> bool {
    let (mut left, mut right) = (0, word.len() - 1);
    let chars: Vec<char> = word.chars().collect();
    let mut count = 1;

    while left < right {
        if chars[left] != chars[right] {
            if count == 0 {
                return false;
            }
            if chars[left + 1] == chars[right] {
                left += 1;
                count -= 1;
                continue;
            } else {
                right -= 1;
                count -= 1;
                continue;
            }
        }
        left += 1;
        right -= 1;
    }
    true
}
