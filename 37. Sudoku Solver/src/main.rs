fn main() {
    let board = vec![
        vec!["5","3",".",".","7",".",".",".","."],
        vec!["6",".",".","1","9","5",".",".","."],
        vec![".","9","8",".",".",".",".","6","."],
        vec!["8",".",".",".","6",".",".",".","3"],
        vec!["4",".",".","8",".","3",".",".","1"],
        vec!["7",".",".",".","2",".",".",".","6"],
        vec![".","6",".",".",".",".","2","8","."],
        vec![".",".",".","4","1","9",".",".","5"],
        vec![".",".",".",".","8",".",".","7","9"]
    ];
    let mut board = board.into_iter().map(|vec| {
        vec.into_iter().map(|s| {
            s.chars().nth(0).unwrap()
        }).collect()
    }).collect();
    Solution::solve_sudoku(&mut board);
    Solution::pb(&board);
}

struct Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    pub fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    for n in 1..=9 {
                        let c = std::char::from_digit(n, 10).unwrap();
                        if Self::valid(board, i, j, c) {
                            board[i][j] = c;
                            if Self::solve(board) { return true }
                        }
                    }
                    board[i][j] = '.';
                    return false
                }
            }
        }
        true
    }

    pub fn valid(board: &Vec<Vec<char>>, i: usize, j: usize, c: char) -> bool {
        let ibase = i / 3 * 3;
        let jbase = j / 3 * 3;
        for offset in 0..9 {
            if board[offset][j] == c { return false }
            if board[i][offset] == c { return false }
            if board[ibase + offset / 3][jbase + offset % 3] == c {
                return false
            }
        }
        true
    }

    pub fn pb(board: &Vec<Vec<char>>) {
        println!("{}", format!(
            "
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}
            {}{}{}{}{}{}{}{}{}",
            board[0][0],board[0][1],board[0][2],board[0][3],board[0][4],board[0][5],board[0][6],board[0][7],board[0][8],
            board[1][0],board[1][1],board[1][2],board[1][3],board[1][4],board[1][5],board[1][6],board[1][7],board[1][8],
            board[2][0],board[2][1],board[2][2],board[2][3],board[2][4],board[2][5],board[2][6],board[2][7],board[2][8],
            board[3][0],board[3][1],board[3][2],board[3][3],board[3][4],board[3][5],board[3][6],board[3][7],board[3][8],
            board[4][0],board[4][1],board[4][2],board[4][3],board[4][4],board[4][5],board[4][6],board[4][7],board[4][8],
            board[5][0],board[5][1],board[5][2],board[5][3],board[5][4],board[5][5],board[5][6],board[5][7],board[5][8],
            board[6][0],board[6][1],board[6][2],board[6][3],board[6][4],board[6][5],board[6][6],board[6][7],board[6][8],
            board[7][0],board[7][1],board[7][2],board[7][3],board[7][4],board[7][5],board[7][6],board[7][7],board[7][8],
            board[8][0],board[8][1],board[8][2],board[8][3],board[8][4],board[8][5],board[8][6],board[8][7],board[8][8],
        ));
    }
}

