pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    use std::collections::HashSet;
    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut count = 0;

    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut curr_num = num;
            let mut curr_count = 1;
            loop {
                if num_set.contains(&(curr_num + 1)) {
                    curr_num += 1;
                    curr_count += 1;
                } else {
                    break;
                }
            }
            count = std::cmp::max(count, curr_count);
        }
    }
    count
}
