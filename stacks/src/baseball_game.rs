pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut scores: Vec<i32> = vec![];

    for ops in operations {
        match ops.as_str() {
            "D" => {
                let prev = scores.last().unwrap();
                scores.push(prev * 2);
            }
            "C" => {
                let _ = scores.pop().unwrap();
            }
            "+" => {
                let n = scores.len();
                scores.push(scores[n - 1] + scores[n - 2]);
            }
            val => scores.push(val.parse().unwrap()),
        }
    }
    scores.into_iter().sum()
}
