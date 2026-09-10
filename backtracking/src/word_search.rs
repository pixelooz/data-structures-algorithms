use crate::Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word_bytes = word.as_bytes();
        let rows = board.len();
        let cols = board[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if Self::dfs(&mut board, &word_bytes, r, c, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &mut [Vec<char>], word: &[u8], row: usize, col: usize, index: usize) -> bool {
        if index == word.len() {
            return true;
        }
        if row >= board.len() || col >= board[0].len() || board[row][col] as u8 != word[index] {
            return false;
        }
        let original = board[row][col];
        board[row][col] = '#';

        let result = (row > 0 && Self::dfs(board, word, row - 1, col, index + 1))
            || (col > 0 && Self::dfs(board, word, row, col - 1, index + 1))
            || Self::dfs(board, word, row + 1, col, index + 1)
            || Self::dfs(board, word, row, col + 1, index + 1);

        board[row][col] = original;
        result
    }
}
