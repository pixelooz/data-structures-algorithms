pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    let mut smap = std::collections::HashMap::new();
    let mut tmap = std::collections::HashMap::new();
    for (&s_char, &t_char) in s_bytes.iter().zip(t_bytes) {
        if *smap.entry(s_char).or_insert(t_char) != t_char {
            return false;
        };
        if *tmap.entry(t_char).or_insert(s_char) != s_char {
            return false;
        };
    }
    true
}
