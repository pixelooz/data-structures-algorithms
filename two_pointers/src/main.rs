use crate::{
    is_palindrome::is_palindrome, reverse_string::reverse_string,
    valid_palindrome_ii::valid_palindrome,
};

mod is_palindrome;
mod reverse_string;
mod valid_palindrome_ii;

fn main() {
    let mut arr = ['h', 'e', 'l', 'l', 'o'].to_vec();
    reverse_string(&mut arr);
    println!("reversed_string={:?}", arr);

    let arr = " ".to_string();
    let result = is_palindrome(arr);
    println!("is_palindrome={:?}", result);

    let arr = "eceec".to_string();
    let result = valid_palindrome(arr);
    println!("valid_palindrome={:?}", result);
}
