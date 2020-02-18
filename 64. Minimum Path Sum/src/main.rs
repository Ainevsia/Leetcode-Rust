fn main() {
    assert_eq!(Solution::min_path_sum(vec![
        vec![1,3,1],
        vec![1,5,1],
        vec![4,2,1]
    ]), 7);
}

struct Solution {}

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 1..n { grid[0][i] += grid[0][i-1]; }
        for i in 1..m {
            grid[i][0] += grid[i-1][0];
            for j in 1..n {
                grid[i][j] += std::cmp::min(grid[i-1][j], grid[i][j-1]);
            }
        }
        grid[m-1][n-1]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::min_path_sum(vec![
            vec![1,3,1],
            vec![1,5,1],
            vec![4,2,1]
        ]), 7);
        assert_eq!(Solution::min_path_sum(vec![
            vec![1],
        ]), 1);
    }
}