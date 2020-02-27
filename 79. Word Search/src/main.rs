fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.len() == 0 || board.len() == 0 || board[0].len() == 0 { return false }
        let mut used = vec![vec![false; board[0].len()]; board.len()];
        let word = word.chars().collect::<Vec<char>>();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == word[0] {
                    used[i][j] = true;
                    if Self::search(&board, &word[1..], i, j, &mut used) { return true }
                    used[i][j] = false;
                }
            }
        }
        false
    }

    pub fn search(board: & Vec<Vec<char>>, word: &[char], i: usize, j: usize, used: &mut Vec<Vec<bool>>) -> bool {
        if word.len() == 0 { return true }
        if i > 0 && !used[i-1][j] && board[i-1][j] == word[0] {
            used[i-1][j] = true;
            if Self::search(board, &word[1..], i-1, j, used) { return true }
            used[i-1][j] = false;
        }
        if j > 0 && !used[i][j-1] && board[i][j-1] == word[0] {
            used[i][j-1] = true;
            if Self::search(board, &word[1..], i, j-1, used) { return true }
            used[i][j-1] = false;
        }
        if i < board.len() - 1 && !used[i+1][j] && board[i+1][j] == word[0] {
            used[i+1][j] = true;
            if Self::search(board, &word[1..], i+1, j, used) { return true }
            used[i+1][j] = false;
        }
        if j < board[0].len() - 1 && !used[i][j+1] && board[i][j+1] == word[0] {
            used[i][j+1] = true;
            if Self::search(board, &word[1..], i, j+1, used) { return true }
            used[i][j+1] = false;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::exist(vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ], "ABCCED".to_string()), true);
    }
}