use crate::Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let col = grid[0].len();
        let row = grid.len();

        let mut max_area = 0;
        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == 1 {
                    let area = Self::max_area_dfs(&mut grid, r as isize, c as isize);
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }

    fn max_area_dfs(grid: &mut Vec<Vec<i32>>, row: isize, col: isize) -> i32 {
        if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
            return 0;
        }
        let (r, c) = (row as usize, col as usize);
        if grid[r][c] == -1 || grid[r][c] == 0 {
            return 0;
        }
        grid[r][c] = -1;
        let mut area = 1;
        area += Self::max_area_dfs(grid, row - 1, col);
        area += Self::max_area_dfs(grid, row + 1, col);
        area += Self::max_area_dfs(grid, row, col - 1);
        area += Self::max_area_dfs(grid, row, col + 1);
        area
    }
}
