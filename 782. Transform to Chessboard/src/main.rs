fn main() {
    assert_eq!(Solution::moves_to_chessboard(
        vec![
        vec![0,0,1,0,1,0,1,1],
        vec![1,1,0,1,0,1,0,0],
        vec![0,0,1,0,1,0,1,1],
        vec![1,1,0,1,0,1,0,0],
        vec![1,1,0,1,0,1,0,0],
        vec![0,0,1,0,1,0,1,1],
        vec![0,0,1,0,1,0,1,1],
        vec![1,1,0,1,0,1,0,0]]
    ),2);
}

struct Solution {}

/// https://www.cnblogs.com/h-hkai/p/10662760.html
/// 
impl Solution {
    pub fn chekc_valid(board: &Vec<Vec<i32>>) -> bool {
        let n = board.len();
        for i in 1..n {
            let eq = if board[0][0] == board[i][0] { true } else { false };
            for j in 1..n {
                if eq && board[0][j] != board[i][j] ||
                 ! eq && board[0][j] == board[i][j] { return false }
            }
        }
        for i in 0..n {
            let ones = board[i].iter().filter(|x| x.is_positive()).count();
            if ones == 0 { return false }
            if n % 2 == 0 && ones * 2 != n { return false }
            if n % 2 == 1 && (ones * 2 + 1 != n && ones * 2 - 1 != n) { return false }
        }
        for i in 0..n {
            let ones = board.iter().filter(|x| x[i].is_positive()).count();
            if ones == 0 { return false }
            if n % 2 == 0 && ones * 2 != n { return false }
            if n % 2 == 1 && (ones * 2 + 1 != n && ones * 2 - 1 != n) { return false }
        }
        true
    }

    pub fn count_odd(mut arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut cnt = 0;
        let ones = arr.iter().filter(|x| x.is_positive()).count();
        let start_from_one = if ones * 2 + 1 == n { false } else { true };
        let (even, odd) = if start_from_one { (1,0) } else { (0,1) };
        for i in 0..n {
            if i % 2 == 0 && arr[i] != even {
                for j in i+1..n {
                    if j % 2 == 1 && arr[j] != odd {
                        arr[j] = odd;
                        cnt += 1;
                        break
                    }
                }
            } else if i % 2 == 1 && arr[i] != odd {
                for j in i+1..n {
                    if j % 2 == 0 && arr[j] != even {
                        arr[j] = even;
                        cnt += 1;
                        break;
                    }
                }
            }
        }
        cnt
    }

    pub fn count_even(mut arr: Vec<i32>, start_from_one: bool) -> i32 {
        let n = arr.len();
        let mut cnt = 0;
        let (even, odd) = if start_from_one { (1,0) } else { (0,1) };
        for i in 0..n {
            if i % 2 == 0 && arr[i] != even {
                for j in i+1..n {
                    if j % 2 == 1 && arr[j] != odd {
                        arr[j] = odd;
                        cnt += 1;
                        break
                    }
                }
            } else if i % 2 == 1 && arr[i] != odd {
                for j in i+1..n {
                    if j % 2 == 0 && arr[j] != even {
                        arr[j] = even;
                        cnt += 1;
                        break;
                    }
                }
            }
        }
        cnt
    }

    pub fn deal_with_odd(board: Vec<Vec<i32>>) -> i32 {
        let arr = board.iter().fold(Vec::with_capacity(board.len()), |mut res, row| {
            res.push(row[0]); 
            res
        });
        // count row & count column
        Self::count_odd(board[0].to_owned()) + Self::count_odd(arr)
    }

    pub fn deal_with_even(board: Vec<Vec<i32>>) -> i32 {
        let mut res = i32::max_value();
        let arr = board.iter().fold(Vec::with_capacity(board.len()), |mut res, row| {
            res.push(row[0]); 
            res
        });
        let cnt = Self::count_even(board[0].to_owned(),true) + Self::count_even(arr.to_owned(),true);
        if cnt < res { res = cnt }
        let cnt = Self::count_even(board[0].to_owned(),true) + Self::count_even(arr.to_owned(),false);
        if cnt < res { res = cnt }
        let cnt = Self::count_even(board[0].to_owned(),false) + Self::count_even(arr.to_owned(),true);
        if cnt < res { res = cnt }
        let cnt = Self::count_even(board[0].to_owned(),false) + Self::count_even(arr,false);
        if cnt < res { res = cnt }
        res
    }

    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        // check valid
        if ! Self::chekc_valid(&board) { return -1 }
        // count min step
        if n % 2 == 1 {
            Self::deal_with_odd(board)
        } else {
            Self::deal_with_even(board)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::moves_to_chessboard(vec![vec![0,1,1,0],vec![0,1,1,0],vec![1,0,0,1],vec![1,0,0,1]]),2);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![1,0],vec![1,0]]),-1);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![0,1],vec![1,0]]),0);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]]),2);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![1,0,1],vec![1,0,1],vec![0,1,0]]),1);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![1,0,1],vec![0,1,0],vec![1,0,1]]),0);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![0,1,0,1],vec![1,0,1,0],vec![0,1,0,1],vec![1,0,1,0]]),0);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![0,1,0,1],vec![0,1,0,1],vec![1,0,1,0],vec![1,0,1,0]]),1);
        assert_eq!(Solution::moves_to_chessboard(vec![vec![1,1,0],vec![0,0,1],vec![0,0,1]]),2);
        assert_eq!(Solution::moves_to_chessboard(
            vec![
            vec![0,0,1,0,1,0,1,1],
            vec![1,1,0,1,0,1,0,0],
            vec![0,0,1,0,1,0,1,1],
            vec![1,1,0,1,0,1,0,0],
            vec![1,1,0,1,0,1,0,0],
            vec![0,0,1,0,1,0,1,1],
            vec![0,0,1,0,1,0,1,1],
            vec![1,1,0,1,0,1,0,0]]
        ),2);

    }
}