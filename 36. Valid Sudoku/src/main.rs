fn main() {
    let x = vec!["5","3",".",".","7",".",".",".","."];
    println!("x = {:#?}", x);
    let y = x.into_iter().map(|x| x.chars().nth(0).unwrap()).collect::<Vec<char>>();
    println!("x = {:#?}", y);
    for &ibase in &[0,3,6] {
            println!("ibase = {:#?}", ibase);
    }
}

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut buf = vec![0; 10];
        // line
        for i in 0..9 {
            for idx in 1..10 { buf[idx] = 0 }
            for j in 0..9 {
                if board[i][j] != '.' {
                    let idx = board[i][j].to_digit(10).unwrap() as usize;
                    if buf[idx] > 0 { return false }
                    else { buf[idx] += 1 }
                }
            }
        }
        // coloum
        for j in 0..9 {
            for idx in 1..10 { buf[idx] = 0 }
            for i in 0..9 {
                if board[i][j] != '.' {
                    let idx = board[i][j].to_digit(10).unwrap() as usize;
                    if buf[idx] > 0 { return false }
                    else { buf[idx] += 1 }
                }
            }
        }
        // square
        for &ibase in &[0,3,6] {
            for &jbase in &[0,3,6] {
                for idx in 1..10 { buf[idx] = 0 }
                for idx in 0..3 {
                    for jdx in 0..3 {
                        let i = idx + ibase;
                        let j = jdx + jbase;
                        if board[i][j] != '.' {
                            let idx = board[i][j].to_digit(10).unwrap() as usize;
                            if buf[idx] > 0 { return false }
                            else { buf[idx] += 1 }
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic1() {
        let board = 
            vec![
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
        let board = board.into_iter().map(|vec| {
            vec.into_iter().map(|s| {
                s.chars().nth(0).unwrap()
            }).collect()
        }).collect();
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }

    #[test]
    fn basic2() {
        let board = 
            vec![
                vec!["8","3",".",".","7",".",".",".","."],
                vec!["6",".",".","1","9","5",".",".","."],
                vec![".","9","8",".",".",".",".","6","."],
                vec!["8",".",".",".","6",".",".",".","3"],
                vec!["4",".",".","8",".","3",".",".","1"],
                vec!["7",".",".",".","2",".",".",".","6"],
                vec![".","6",".",".",".",".","2","8","."],
                vec![".",".",".","4","1","9",".",".","5"],
                vec![".",".",".",".","8",".",".","7","9"]
            ];
        let board = board.into_iter().map(|vec| {
            vec.into_iter().map(|s| {
                s.chars().nth(0).unwrap()
            }).collect()
        }).collect();
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }
}