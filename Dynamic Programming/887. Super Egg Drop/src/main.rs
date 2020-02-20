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
                let mut min = std::cmp::max(dp[i-1][0], dp[i][j-1]);
                for idx in 0..=j-1 {
                    min = std::cmp::min(min, std::cmp::max(dp[i-1][idx], dp[i][j-1-idx]));
                }
                dp[i][j] = min + 1;
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