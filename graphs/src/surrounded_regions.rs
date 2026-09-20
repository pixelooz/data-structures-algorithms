use crate::Solution;

impl Solution {
    pub fn solve(mut board: &mut Vec<Vec<char>>) {
        let (row_len, col_len) = (board.len(), board[0].len());

        for r in 0..row_len {
            Self::surrounded_regions_dfs(&mut board, r, 0);
            Self::surrounded_regions_dfs(&mut board, r, col_len - 1);
        }
        for c in 0..col_len {
            Self::surrounded_regions_dfs(&mut board, 0, c);
            Self::surrounded_regions_dfs(&mut board, row_len - 1, c);
        }
        for r in 0..row_len {
            for c in 0..col_len {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                }
                if board[r][c] == 'T' {
                    board[r][c] = 'O';
                }
            }
        }
    }

    fn surrounded_regions_dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
        if row >= board.len()
            || col >= board[0].len()
            || board[row][col] != 'O'
            || board[row][col] == 'T'
        {
            return;
        }
        board[row][col] = 'T';
        if row > 0 {
            Self::surrounded_regions_dfs(board, row - 1, col);
        }
        if col > 0 {
            Self::surrounded_regions_dfs(board, row, col - 1);
        }
        Self::surrounded_regions_dfs(board, row + 1, col);
        Self::surrounded_regions_dfs(board, row, col + 1);
    }
}
