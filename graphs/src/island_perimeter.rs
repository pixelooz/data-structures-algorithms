use crate::Solution;

impl Solution {
    pub fn island_perimeter(mut grid: Vec<Vec<i32>>) -> i32 {
        let col = grid[0].len();
        let row = grid.len();
        for r in 0..row {
            for c in 0..col {
                if grid[r][c] == 1 {
                    return Self::island_perimeter_dfs(&mut grid, r as isize, c as isize);
                }
            }
        }
        -1
    }

    fn island_perimeter_dfs(grid: &mut Vec<Vec<i32>>, row: isize, col: isize) -> i32 {
        if row < 0 || row >= grid.len() as isize || col < 0 || col >= grid[0].len() as isize {
            return 1;
        }
        let (urow, ucol) = (row as usize, col as usize);
        if grid[urow][ucol] == 0 {
            return 1;
        }
        if grid[urow][ucol] == -1 {
            return 0;
        }
        grid[urow][ucol] = -1;
        let mut perimeter = 0;
        perimeter += Self::island_perimeter_dfs(grid, row + 1, col);
        perimeter += Self::island_perimeter_dfs(grid, row - 1, col);
        perimeter += Self::island_perimeter_dfs(grid, row, col + 1);
        perimeter += Self::island_perimeter_dfs(grid, row, col - 1);
        perimeter
    }
}
