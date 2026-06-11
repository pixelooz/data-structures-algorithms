/// Shorter version basically maintains that the brackets are pushed in a certain
/// order, so when we encounter the closing brackets, they should pop in the same
/// order of insertion.
/// # Example
/// - Example -> "(\[{}\])"
/// - Pushing -> The closed versions are pushed in order \[")", "]", "}"\] for "([{".
/// - Popping -> So when iterating over the closed variants, the order should be the
/// same for "}])".
pub fn is_valid(braces: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for brace in braces.chars() {
        match brace {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {
                if stack.pop() != Some(brace) {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

pub fn _is_valid(braces: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for brace in braces.chars() {
        match brace {
            '(' | '[' | '{' => stack.push(brace),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
