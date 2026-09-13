use rand::RngExt;

use crate::Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let target = nums.len() - k as usize;

        let mut left = 0;
        let mut right = nums.len() - 1;

        let mut rng = rand::rng();

        while left <= right {
            let pv_index = rng.random_range(left..=right);
            let pivot = nums[pv_index];

            let mut curr = left;

            let mut lt = left;
            let mut gt = right;

            while curr <= gt {
                match nums[curr].cmp(&(pivot as i32)) {
                    std::cmp::Ordering::Less => {
                        nums.swap(lt, curr);
                        curr += 1;
                        lt += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        nums.swap(curr, gt);
                        if gt == 0 {
                            break; // usize protection
                        }
                        gt -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        curr += 1;
                    }
                }
            }
            if target >= lt && target <= gt {
                return pivot as i32;
            } else if target < lt {
                right = lt - 1;
            } else {
                left = gt + 1;
            }
        }
        -1
    }
}
