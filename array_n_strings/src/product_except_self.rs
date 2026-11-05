pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    let mut prefix = 1;
    for (idx, num) in nums.iter().enumerate().take(nums.len() - 1) {
        result[idx + 1] = prefix * num;
        prefix *= num
    }
    let mut suffix = 1;
    for (idx, num) in nums.iter().enumerate().rev() {
        result[idx] *= suffix;
        suffix *= num
    }
    result
}

#[test]
fn test_product_except_self() {
    let vec = vec![1, 2, 3, 4];
    let result = product_except_self(vec);
    println!("{:?}", result)
}
