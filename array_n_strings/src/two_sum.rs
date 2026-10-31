pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::with_capacity(nums.len());
    for (idx, num) in nums.into_iter().enumerate() {
        if let Some(value) = map.get(&(target - num)) {
            return vec![idx as i32, *value];
        }
        map.insert(num, idx as i32);
    }
    vec![]
}
