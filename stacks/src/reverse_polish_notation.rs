pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];

    for tok in tokens {
        match tok.as_str() {
            "+" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b + a);
            }
            "-" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            "*" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b * a);
            }
            "/" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            }
            _ => stack.push(tok.parse::<i32>().unwrap()),
        }
    }
    stack[0]
}
