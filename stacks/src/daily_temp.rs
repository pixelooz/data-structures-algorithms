pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = vec![];
    let mut result = vec![0; temperatures.len()];

    for (idx, tmp) in temperatures.into_iter().enumerate() {
        while stack.last().is_some() && tmp > stack.last().unwrap().1 {
            let (i, _) = stack.pop().unwrap();
            result[i] = (idx - i) as i32;
        }
        stack.push((idx, tmp));
    }
    result
}
