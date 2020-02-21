fn main() {
    assert_eq!(Solution::super_egg_drop(1, 2), 2);
}

struct Solution {}

impl Solution {
    /// https://leetcode.com/problems/super-egg-drop/discuss/158974/C%2B%2BJavaPython-2D-and-1D-DP-O(KlogN)
    /// Original DP definition: I stand on nth floor and give me k eggs, 
    /// the minimum times I try is dp[n][k]. This definition means the result of 
    /// this problem is dp[N][K].
    /// 
    /// This solution is somehow a reverse thinking:
    /// New DP definition: If you give me k egg, let me drop m times, 
    /// I can try out maximum dp[m][k] floors. Based on this definition, 
    /// the result is some m, which cases dp[m][K] equals N.
    /// 
    /// The transfer equation is based on the following facts:
    /// No matter which floor you try, the egg will only break or not break, 
    /// if break, go downstairs, 
    /// if not break, go upstairs.
    /// 
    /// No matter you go up or go down, the num of all the floors is always 
    /// upstairs + downstairs + the floor you try, which is 
    /// dp[m][k] = dp[m - 1][k - 1] + dp[m - 1][k] + 1.
    /// 
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let (K, N) = (k as usize, n as usize);
        let mut dp = vec![vec![0; K + 1]; N + 1];
        let mut m = 0;
        while dp[m][K] < N {
            m += 1;
            for k in 1..=K {
                dp[m][k] = dp[m-1][k-1] + dp[m-1][k] + 1
            }
        }
        m as i32
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