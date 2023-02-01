pub struct Solution {}

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move == 0 { return 0 }
        let m = m as usize;
        let n = n as usize;
        let max_move = max_move as usize;
        let mut dp = vec![vec![vec![0usize; n]; m]; max_move + 1];
        for i in 0..m { dp[1][i][0] += 1; dp[1][i][n-1] += 1 }
        for j in 0..n { dp[1][0][j] += 1; dp[1][m-1][j] += 1 }
        for k in 2..=max_move {for i in 0..m { for j in 0..n {
            if i != 0 { dp[k][i][j] += dp[k-1][i-1][j] }
            if i != m-1 { dp[k][i][j] += dp[k-1][i+1][j] }
            if j != 0 { dp[k][i][j] += dp[k-1][i][j-1] }
            if j != n-1 { dp[k][i][j] += dp[k-1][i][j+1] }
            dp[k][i][j] %= 1000000007;
        }}}
        dp.iter()
        .fold(0usize, |acc, x|  (acc + x[start_row as usize][start_column as usize]) % 1000000007)
         as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_paths(2,2,2,0,0), 6);
        assert_eq!(Solution::find_paths(1,3,3,0,1), 12);
        assert_eq!(Solution::find_paths(10,10,0,5,5), 0);
    }
}
