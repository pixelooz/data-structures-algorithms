use crate::Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (row_len, col_len) = (grid.len(), grid[0].len());
        let (mut time, mut fresh) = (0, 0);

        let mut queue = VecDeque::new();

        for r in 0..row_len {
            for c in 0..col_len {
                if grid[r][c] == 1 {
                    fresh += 1;
                }
                if grid[r][c] == 2 {
                    queue.push_back((r, c, time));
                }
            }
        }
        let directions = [(1, 0), (-1, 0), (0, -1), (0, 1)];

        while let Some((row, col, pop_time)) = queue.pop_front() {
            time = pop_time;
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
                if grid[ur][uc] != 1 {
                    continue;
                }
                fresh -= 1;
                grid[ur][uc] = 2;
                queue.push_back((ur, uc, pop_time + 1));
            }
        }
        if fresh == 0 { time } else { -1 }
    }
}
