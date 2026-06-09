pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    // we reverse the entire thing before hand because this is the only way to ensure we
    // are rotating the array k times meaning targeting the last k elements.
    // [1, 2, 3, 4, 5, 6, 7], k = 3, meaning we need to reverse 5-7 and to target just
    // them we need to rotate. [7, 6, 5, 4, 3, 2, 1], and 0-(k-1) will give just those
    // elements.
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}
