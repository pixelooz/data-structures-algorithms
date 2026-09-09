use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(count: i32) -> Vec<String> {
        let mut result = Vec::new();
        backtrack_combinations(count, 0, 0, &mut String::new(), &mut result);
        result
    }
}

fn backtrack_combinations(
    count: i32,
    open: i32,
    close: i32,
    combo: &mut String,
    result: &mut Vec<String>,
) {
    if open == close && close == count {
        result.push(combo.to_string());
        return;
    }
    if open < count {
        combo.push('(');
        backtrack_combinations(count, open + 1, close, combo, result);
        combo.pop();
    }
    if close < open {
        combo.push(')');
        backtrack_combinations(count, open, close + 1, combo, result);
        combo.pop();
    }
}
