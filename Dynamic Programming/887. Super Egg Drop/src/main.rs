fn main() {
    assert_eq!(Solution::super_egg_drop(1, 2), 2);
}

struct Solution {}

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; k as usize + 1];
        for i in 1..=n as usize {
            dp[1][i] = i as i32;
        }
        for i in 2..=k as usize {
            dp[i][1] = 1;
            for j in 2..=n as usize {
                // use binary search to reduce the complexity to O(KN logN)
                let (mut start, mut end) = (0, j-1);
                let mut m = (start + end) / 2;
                while start + 1 != end {  // breaks when start + 1 == end
                    if dp[i][m] < dp[i-1][j-1-m] { start = m }
                    else if dp[i][m] == dp[i-1][j-1-m] { break }
                    else if dp[i][m] > dp[i-1][j-1-m] { end = m }
                    m = (start + end) / 2;
                }
                // end = start + 1  or  dp[i-1][m] == dp[i][m]
                dp[i][j] = std::cmp::min(std::cmp::max(dp[i][m], dp[i-1][j-1-m]),
                                         std::cmp::max(dp[i][j-m-1], dp[i-1][j-1-m]) ) + 1;
            }
        }
        // println!("dp = {:#?}", dp);
        dp[k as usize][n as usize]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::super_egg_drop(1, 2), 2);
        assert_eq!(Solution::super_egg_drop(2, 6), 3);
        assert_eq!(Solution::super_egg_drop(3, 14), 4);
        assert_eq!(Solution::super_egg_drop(10, 5000), 13);
    }
}