use crate::Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let mut count = 0;
        let col = grid[0].len();
        let row = grid.len();
        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == '1' {
                    count += 1;
                    Self::num_islands_dfs(&mut grid, r as isize, c as isize);
                }
            }
        }
        count
    }

    fn num_islands_dfs(grid: &mut Vec<Vec<char>>, row: isize, col: isize) {
        if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
            return;
        }
        let (r, c) = (row as usize, col as usize);
        if grid[r][c] == '0' {
            return;
        }
        grid[r][c] = '0';
        Self::num_islands_dfs(grid, row - 1, col);
        Self::num_islands_dfs(grid, row + 1, col);
        Self::num_islands_dfs(grid, row, col - 1);
        Self::num_islands_dfs(grid, row, col + 1);
    }
}
