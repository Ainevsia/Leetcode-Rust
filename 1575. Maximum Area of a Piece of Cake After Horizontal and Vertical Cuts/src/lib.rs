pub struct Solution {}

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();    // [2,100]
        let m = fuel as usize + 1;
        let mut dist = vec![vec![0; n]; n];
        for i in 0..n { for j in i+1 ..n {
            dist[i][j] = (locations[i] - locations[j]).abs() as usize;
            dist[j][i] = dist[i][j];
        }}
        let mut dp = vec![vec![0usize; n]; m];
        dp[0][start as usize] = 1;
        for i in 1..m { for j in 0..n {
            for k in 0..n {
                let off = dist[j][k];
                if off == 0 { continue }
                if i >= off {
                    dp[i][j] += dp[i-off][k];
                    dp[i][j] %= 1000000007;
                }
            }
        }}
        dp.iter()
        .fold(0usize, |acc, x| 
            (acc + x[finish as usize]) % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_routes(
            vec![2,3,6,8,4], 1, 3, 5
        ), 4);
        assert_eq!(Solution::count_routes(
            vec![4,3,1], 1, 0, 6
        ), 5);
        assert_eq!(Solution::count_routes(
            vec![5,2,1], 0, 2, 3
        ), 0);
    }
}
