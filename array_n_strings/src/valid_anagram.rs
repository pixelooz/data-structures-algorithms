pub fn is_anagram(one: String, two: String) -> bool {
    if one.len() != two.len() {
        return false;
    }
    let mut arr = [0i32; 26];
    for i in 0..one.len() {
        arr[(one.as_bytes()[i] - b'a') as usize] += 1;
        arr[(two.as_bytes()[i] - b'a') as usize] -= 1;
    }
    for count in arr {
        if count != 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_valid_anagram() {
    let one = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
    let two = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba".to_string();
    let is = is_anagram(one, two);
    assert!(!is);
}
