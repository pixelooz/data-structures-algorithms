pub fn reverse_string(chars: &mut Vec<char>) {
    let (mut left, mut right) = (0, chars.len() - 1);
    while left < right {
        let temp = chars[right];
        chars[right] = chars[left];
        chars[left] = temp;
        left += 1;
        right -= 1;
    }
}

pub fn _reverse_string(s: &mut Vec<char>) {
    s.reverse();
}
