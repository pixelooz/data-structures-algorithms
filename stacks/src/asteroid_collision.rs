pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];

    for mut asteroid in asteroids {
        while !stack.is_empty() && asteroid < 0 && stack.last().unwrap() > &0 {
            use std::cmp::Ordering::*;
            match (asteroid + stack.last().unwrap()).cmp(&0) {
                Less => {
                    stack.pop();
                }
                Equal => {
                    asteroid = 0;
                    stack.pop();
                }
                Greater => {
                    asteroid = 0;
                }
            }
        }
        if asteroid != 0 {
            stack.push(asteroid);
        }
    }
    stack
}
