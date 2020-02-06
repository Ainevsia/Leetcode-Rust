fn main() {
    Solution::solve_n_queens(4);
}

struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 0 { return vec![vec![]] }
        let n = n as usize;
        let mut board = vec![vec!['.'; n]; n];
        let mut ret = vec![];
        Self::solve(&mut board, 0, &mut ret);
        ret
    }

    pub fn solve(board: &mut Vec<Vec<char>>, line: usize, ret: &mut Vec<Vec<String>>) {
        if line >= board.len() {
            let x: Vec<String> = board.iter().map(|x| x.iter().collect()).collect();
            ret.push(x);
            return
        }
        for i in 0..board.len() {
            if Self::valid(board, line, i) {
                board[line][i] = 'Q';
                Self::solve(board, line + 1, ret);
                board[line][i] = '.';
            }
        }
    }

    pub fn valid(board: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        // check column up
        for idx in 0..i {
            if board[idx][j] == 'Q' { return false }
        }
        // check diagonal \ up
        let (mut idx, mut jdx) = (i as i32 - 1, j as i32 - 1);
        while idx >= 0 && jdx >= 0 {
            if board[idx as usize][jdx as usize] == 'Q' { return false }
            idx -= 1;
            jdx -= 1;
        }
        // check diagonal / up
        let (mut idx, mut jdx) = (i as i32 - 1, j + 1);
        while idx >= 0 && jdx < board.len() {
            if board[idx as usize][jdx] == 'Q' { return false }
            idx -= 1;
            jdx += 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..",  // Solution 1
                     "...Q",
                     "Q...",
                     "..Q."],

                vec!["..Q.",  // Solution 2
                     "Q...",
                     "...Q",
                     ".Q.."]
            ]
        )
    }
}