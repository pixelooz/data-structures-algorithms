pub fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split(" ").collect();
    if pattern.len() != words.len() {
        return false;
    }
    let pattern = pattern.as_bytes();
    let mut p_to_s_map = std::collections::HashMap::new();
    let mut s_to_p_map = std::collections::HashMap::new();
    for i in 0..pattern.len() {
        let letter = pattern[i];
        let word = words[i];
        if *p_to_s_map.entry(letter).or_insert(word) != word {
            return false;
        }
        if *s_to_p_map.entry(word).or_insert(letter) != letter {
            return false;
        }
    }
    true
}
