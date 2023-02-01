use std::str::Chars;

pub struct Solution {}

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        let mut path = vec![vec![0usize; n + 1]; n + 1];
        for (i, s) in board.iter().enumerate() {
            for (j, c) in s.chars().enumerate() {
                if "123456789".contains(c) {
                    dp[i + 1][j + 1] = c.to_digit(10).unwrap() as i32
                } else {
                    dp[i + 1][j + 1] = -1;
                }
            }
        }
        dp[1][1] = 0;
        dp[n][n] = 0;
        path[0][0] = 1;
        for i in 1..=n { for j in 1..=n {
            if dp[i][j] < 0 { continue }    // stone
            // no path at all, no operation
            if path[i-1][j-1] + path[i-1][j] + path[i][j-1] == 0 { continue }
            // count the max sum road
            let up = if path[i][j-1] == 0 { 0 } else { dp[i][j-1] };
            let le = if path[i-1][j] == 0 { 0 } else { dp[i-1][j] };
            let lu = if path[i-1][j-1] == 0 { 0 } else { dp[i-1][j-1] };
            let m = up.max(le).max(lu);
            dp[i][j] += m;
            if up == m { path[i][j] += path[i][j-1] }
            if le == m { path[i][j] += path[i-1][j] }
            if lu == m { path[i][j] += path[i-1][j-1] }
            path[i][j] %= 1000000007;
        }}
        // println!("{:#?}", dp);
        // println!("{:#?}", path);
        vec![dp[n][n], path[n][n] as i32]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::paths_with_max_score(
            vec!["E23".into(),"2X2".into(),"12S".into()]), vec![7,1]);
        assert_eq!(Solution::paths_with_max_score(
            vec!["E12".into(),"1X1".into(),"21S".into()]), vec![4,2]);
        assert_eq!(Solution::paths_with_max_score(
            vec!["E12".into(),"XXX".into(),"21S".into()]), vec![0,0]);
        assert_eq!(Solution::paths_with_max_score(
            vec!["EX".into(),"XS".into()]), vec![0,1]);
    }
}
