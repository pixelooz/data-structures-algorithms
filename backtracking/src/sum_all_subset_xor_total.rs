pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    explore_subsets(&nums, 0, 0)
}

fn explore_subsets(nums: &[i32], index: usize, xor: i32) -> i32 {
    if index == nums.len() {
        return xor;
    }
    let included = explore_subsets(nums, index + 1, xor ^ nums[index]);
    let excluded = explore_subsets(nums, index + 1, xor);
    included + excluded
}
