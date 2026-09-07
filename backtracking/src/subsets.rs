pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = vec![];
    explore(&nums, 0, &mut vec![], &mut results);
    results
}

fn explore(nums: &[i32], index: usize, curr_sub: &mut Vec<i32>, subs: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        subs.push(curr_sub.clone());
        return;
    }
    curr_sub.push(nums[index]);
    explore(&nums, index + 1, curr_sub, subs);
    curr_sub.pop();
    explore(&nums, index + 1, curr_sub, subs);
}
