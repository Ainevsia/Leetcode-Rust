fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut board = vec![vec!['.'; n]; n];
        let mut ret = 0;
        Self::solve(&mut board, 0, &mut ret);
        ret
    }

    pub fn solve(board: &mut Vec<Vec<char>>, line: usize, ret: &mut i32) {
        if line >= board.len() {
            *ret += 1;
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
            Solution::total_n_queens(4),
            2
        )
    }
}