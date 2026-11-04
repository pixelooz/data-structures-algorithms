pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = std::collections::HashMap::new();
    for word in strs {
        let mut arr = [0; 26];
        for ch in word.as_bytes() {
            arr[(*ch - b'a') as usize] += 1;
        }
        groups.entry(arr).or_insert(vec![]).push(word);
    }
    let mut result = Vec::with_capacity(groups.len());
    for group in groups.into_values() {
        result.push(group);
    }
    result
}
