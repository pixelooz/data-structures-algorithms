use crate::Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let col_len = grid[0].len();
        let row_len = grid.len();

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        for r in 0..row_len {
            for c in 0..col_len {
                if grid[r][c] == 0 {
                    visited.insert((r, c));
                    queue.push_back((r, c));
                }
            }
        }
        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((row, col)) = queue.pop_front() {
            for &(dr, dc) in &directions {
                let next_row = dr + row as isize;
                let next_col = dc + col as isize;

                if next_row < 0
                    || next_row >= row_len as isize
                    || next_col < 0
                    || next_col >= col_len as isize
                {
                    continue;
                }
                let (ur, uc) = (next_row as usize, next_col as usize);
                if grid[ur][uc] == -1 || visited.contains(&(ur, uc)) {
                    continue;
                }
                visited.insert((ur, uc));
                queue.push_back((ur, uc));
                grid[ur][uc] += grid[row][col] + 1;
            }
        }
    }
}
