use crate::Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let people = n as usize;
        let mut net_scores = vec![0; (n + 1) as usize];

        for value in &trust {
            let truster = value[0] as usize;
            let trustee = value[1] as usize;

            net_scores[truster] -= 1;
            net_scores[trustee] += 1;
        }
        let req_score = n - 1;
        for i in 1..=people {
            if net_scores[i] == req_score {
                return i as i32;
            }
        }
        -1
    }
}
