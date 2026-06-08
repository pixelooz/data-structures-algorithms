use crate::{
    is_palindrome::is_palindrome, merge_sorted_array::merge,
    merge_strings_alter::merge_alternately, remove_duplicates_sorted_arr::remove_duplicates,
    reverse_string::reverse_string, two_sum_ii::two_sum, valid_palindrome_ii::valid_palindrome,
};

mod is_palindrome;
mod merge_sorted_array;
mod merge_strings_alter;
mod remove_duplicates_sorted_arr;
mod reverse_string;
mod two_sum_ii;
mod valid_palindrome_ii;

fn main() {
    let mut strs = ['h', 'e', 'l', 'l', 'o'].to_vec();
    reverse_string(&mut strs);
    println!("reversed_string={:?}", strs);

    let strs = " ".to_string();
    let result = is_palindrome(strs);
    println!("is_palindrome={:?}", result);

    let strs = "eceec".to_string();
    let result = valid_palindrome(strs);
    println!("valid_palindrome={:?}", result);

    let word1 = "abcd".to_string();
    let word2 = "pq".to_string();
    let result = merge_alternately(word1, word2);
    println!("merged={:?}", result);

    let mut nums1 = [0].to_vec();
    let m = 0;
    let mut nums2 = [1].to_vec();
    let n = nums2.len() as i32;
    merge(&mut nums1, m, &mut nums2, n);
    println!("merge_sorted={:?}", nums1);

    let mut arr = [1, 1, 2].to_vec();
    let result = remove_duplicates(&mut arr);
    println!("de-duplicated={:?}, {}", arr, result);

    let arr = [0].to_vec();
    let result = two_sum(arr, 9);
    println!("two_sum_ii={:?}", result);
}
