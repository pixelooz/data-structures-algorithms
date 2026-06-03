pub fn is_palindrome(words: String) -> bool {
    let chars: Vec<char> = words
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect();

    if chars.len() == 0 {
        return true;
    }
    let (mut left, mut right) = (0, chars.len() - 1);

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
