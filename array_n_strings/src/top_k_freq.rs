pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count = std::collections::HashMap::new();
    for num in &nums {
        *count.entry(*num).or_insert(0) += 1;
    }
    let mut buckets = vec![Vec::new(); nums.len() + 1];
    for (num, freq) in count {
        buckets[freq].push(num);
    }
    buckets
        .into_iter()
        .rev()
        .flatten()
        .take(k as usize)
        .collect()
}
