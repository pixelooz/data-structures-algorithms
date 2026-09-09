use crate::Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();

        fn explore_subs(nums: &mut [i32], index: usize, results: &mut Vec<Vec<i32>>) {
            if index == nums.len() {
                results.push(nums.to_vec());
                return;
            }
            for i in index..nums.len() {
                nums.swap(i, index);
                explore_subs(nums, index + 1, results);
                nums.swap(i, index);
            }
        }
        explore_subs(&mut nums, 0, &mut results);
        results
    }
}

#[test]
fn test_permute() {
    let nums = vec![1, 2, 3];
    let results = Solution::permute(nums);
    println!("{:?}", results);
}
