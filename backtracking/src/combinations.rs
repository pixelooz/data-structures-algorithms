use crate::Solution;

impl Solution {
    pub fn combine(limit: i32, k: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();

        fn explore_subs(
            limit: i32,
            k: i32,
            curr_num: i32,
            path: &mut Vec<i32>,
            results: &mut Vec<Vec<i32>>,
        ) {
            if k <= 0 {
                results.push(path.clone());
                return;
            }
            for num in curr_num..=limit {
                path.push(num);
                explore_subs(limit, k - 1, num + 1, path, results);
                path.pop();
            }
        }
        explore_subs(limit, k, 1, &mut vec![], &mut results);
        results
    }
}

#[test]
fn test_combine() {
    let n = 4;
    let k = 2;
    let results = Solution::combine(n, k);
    println!("{:?}", results);
}
