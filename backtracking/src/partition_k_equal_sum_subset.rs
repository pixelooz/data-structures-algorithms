use crate::Solution;

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        if k > nums.len() {
            return false;
        }
        let sum = nums.iter().sum::<i32>();
        if sum % k as i32 != 0 {
            return false;
        }
        let largest = sum / k as i32;

        nums.sort_unstable_by(|a, b| b.cmp(a));
        if nums[0] > largest {
            return false;
        }
        let mut subsets = vec![0; k as usize];
        Self::partition_k_subset_dfs(&nums, 0, &mut subsets, largest)
    }

    fn partition_k_subset_dfs(nums: &[i32], index: usize, sums: &mut [i32], largest: i32) -> bool {
        if index == nums.len() {
            return true;
        }
        for i in 0..sums.len() {
            if nums[index] + sums[i] > largest {
                continue;
            }
            if i > 0 && sums[i] == sums[i - 1] {
                continue;
            }
            sums[i] += nums[index];
            if Self::partition_k_subset_dfs(nums, index + 1, sums, largest) {
                return true;
            }
            sums[i] -= nums[index];
        }

        false
    }
}
