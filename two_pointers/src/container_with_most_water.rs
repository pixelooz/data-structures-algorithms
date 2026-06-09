pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, height.len() - 1);
    let mut result = std::i32::MIN;

    while l < r {
        let h = std::cmp::min(height[l], height[r]);
        let w = l.abs_diff(r) as i32;
        let area = h * w;
        result = std::cmp::max(result, area);
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    result
}
