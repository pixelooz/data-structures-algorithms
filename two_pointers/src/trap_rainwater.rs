pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }
    let mut result = 0;

    let (mut l, mut r) = (0, height.len() - 1);
    let mut l_max = height[l];
    let mut r_max = height[r];

    while l < r {
        match l_max.cmp(&r_max) {
            std::cmp::Ordering::Less => {
                l += 1;
                l_max = l_max.max(height[l]);
                result += l_max - height[l];
            }
            _ => {
                r -= 1;
                r_max = r_max.max(height[r]);
                result += r_max - height[r];
            }
        }
    }
    result
}
