pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();

    let mut ch1 = word1.chars();
    let mut ch2 = word2.chars();

    loop {
        match (ch1.next(), ch2.next()) {
            (Some(c1), Some(c2)) => {
                result.push(c1);
                result.push(c2);
            }
            (Some(c1), None) => result.push(c1),
            (None, Some(c2)) => result.push(c2),
            (None, None) => break,
        }
    }
    result
}

// correct but could be better
pub fn _merge_alternately(word1: String, word2: String) -> String {
    let (mut left, mut right) = (0, 0);
    let mut result = String::new();
    let char1: Vec<char> = word1.chars().collect();
    let char2: Vec<char> = word2.chars().collect();

    let len = if word1.len() > word2.len() {
        word1.len()
    } else {
        word2.len()
    };
    for _ in 0..len {
        if left < char1.len() {
            result.push(char1[left]);
        }
        if right < char2.len() {
            result.push(char2[right]);
        }
        left += 1;
        right += 1;
    }
    result
}
