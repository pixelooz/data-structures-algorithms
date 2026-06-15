use crate::{
    asteroid_collision::asteroid_collision, baseball_game::cal_points,
    reverse_polish_notation::eval_rpn, valid_paren::is_valid,
};

mod asteroid_collision;
mod baseball_game;
mod reverse_polish_notation;
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

    let arr = [
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let result = eval_rpn(arr);
    println!("{result}");

    let asteroids = vec![8, -8];
    let result = asteroid_collision(asteroids);
    println!("{result:?}");
}
