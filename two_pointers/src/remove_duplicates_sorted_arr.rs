pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut i, mut j) = (0, 0);
    let mut curr = std::i32::MIN;

    'outer: while i < nums.len() && j < nums.len() {
        while nums[j] == curr {
            j += 1;
            if j >= nums.len() {
                continue 'outer;
            }
        }
        nums[i] = nums[j];
        i += 1;
        curr = nums[j];
    }
    i as i32
}
