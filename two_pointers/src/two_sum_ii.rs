pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    let (mut i, mut j) = (0usize, nums.len() - 1);
    let mut result = Vec::new();

    while i < j {
        if nums[i] + nums[j] > target {
            j -= 1;
        } else if nums[i] + nums[j] < target {
            i += 1;
        } else {
            result.extend(vec![(i + 1) as i32, (j + 1) as i32]);
            break;
        }
    }
    result
}
