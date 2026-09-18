use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row_len, col_len) = (heights.len(), heights[0].len());
        let mut result = Vec::new();

        let mut atlantic = HashSet::new();
        let mut pacific = HashSet::new();

        for col in 0..col_len {
            Self::pacific_atlantic_dfs(&heights, 0, col as isize, &mut pacific, heights[0][col]);
            Self::pacific_atlantic_dfs(
                &heights,
                (row_len - 1) as isize,
                col as isize,
                &mut atlantic,
                heights[row_len - 1][col],
            );
        }
        for row in 0..row_len {
            Self::pacific_atlantic_dfs(&heights, row as isize, 0, &mut pacific, heights[row][0]);
            Self::pacific_atlantic_dfs(
                &heights,
                row as isize,
                (col_len - 1) as isize,
                &mut atlantic,
                heights[row][col_len - 1],
            );
        }
        for r in 0..row_len {
            for c in 0..col_len {
                if atlantic.contains(&(r, c)) && pacific.contains(&(r, c)) {
                    result.push(vec![r as i32, c as i32]);
                }
            }
        }
        result
    }

    fn pacific_atlantic_dfs(
        heights: &[Vec<i32>],
        row: isize,
        col: isize,
        visited: &mut HashSet<(usize, usize)>,
        prev_height: i32,
    ) {
        let (row_len, col_len) = (heights.len() as isize, heights[0].len() as isize);

        if row < 0 || row >= row_len || col < 0 || col >= col_len {
            return;
        }
        let (ur, uc) = (row as usize, col as usize);
        let curr_height = heights[ur][uc];
        if visited.contains(&(ur, uc)) || curr_height < prev_height {
            return;
        }
        visited.insert((ur, uc));
        let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for &(dr, dc) in &directions {
            let next_row = row + dr;
            let next_col = col + dc;

            Self::pacific_atlantic_dfs(heights, next_row, next_col, visited, curr_height);
        }
    }
}
