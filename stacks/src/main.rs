use crate::{baseball_game::cal_points, valid_paren::is_valid};

mod baseball_game;
mod valid_paren;

fn main() {
    let operations = ["5", "-2", "4", "C", "D", "9", "+", "+"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
    let result = cal_points(operations);
    println!("baseball score={result}");

    let braces = "]".to_string();
    let result = is_valid(braces);
    println!("valid paren={result}");
}
