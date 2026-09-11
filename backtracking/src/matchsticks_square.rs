use crate::Solution;

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            return false;
        }
        let sum: u64 = matchsticks.iter().map(|&x| x as u64).sum();
        if !sum.is_multiple_of(4) {
            return false;
        }
        let size = (sum / 4) as i32;
        let mut sides = [0i32; 4];

        matchsticks.sort_unstable_by(|a, b| b.cmp(a));
        if matchsticks[0] > size {
            return false;
        }
        Self::makesquare_dfs(&matchsticks, 0, size, &mut sides)
    }

    fn makesquare_dfs(matchsticks: &[i32], index: usize, size: i32, sides: &mut [i32]) -> bool {
        if index == matchsticks.len() {
            return true;
        }
        for i in 0..4 {
            if sides[i] + matchsticks[index] > size {
                continue;
            }
            if i > 0 && sides[i] == sides[i - 1] {
                continue;
            }
            sides[i] += matchsticks[index];
            if Self::makesquare_dfs(matchsticks, index + 1, size, sides) {
                return true;
            }
            sides[i] -= matchsticks[index];
        }
        false
    }
}
