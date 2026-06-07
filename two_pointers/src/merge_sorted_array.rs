pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    while m > 0 && n > 0 {
        match nums1[m - 1].cmp(&nums2[n - 1]) {
            std::cmp::Ordering::Less => {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
            std::cmp::Ordering::Equal => {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
            std::cmp::Ordering::Greater => {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            }
        }
    }
    while n > 0 {
        nums1[m + n - 1] = nums2[n - 1];
        n -= 1;
    }
}
