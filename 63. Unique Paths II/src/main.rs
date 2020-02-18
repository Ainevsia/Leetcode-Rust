fn main() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![
        vec![0,0,0],
        vec![0,1,0],
        vec![0,0,0]
    ]), 2);
}

struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len() + 1, obstacle_grid[0].len() + 1);
        if m == 2 || n == 2 { return if obstacle_grid.iter().any(|x| x.contains(&1)) { 0 } else { 1 } }
        let mut grid = vec![vec![0; n]; m];
        grid[0][1] = 1;
        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i-1][j-1] == 0 {
                    grid[i][j] = grid[i-1][j] + grid[i][j-1];
                }
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
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0,0,0],
            vec![0,1,0],
            vec![0,0,0]
        ]), 2);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0,0,0],
            vec![0,1,1],
            vec![0,0,0]
        ]), 1);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0,0,0],
            vec![0,1,1],
            vec![0,1,0]
        ]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0,0,0],
            vec![1,1,1],
            vec![0,1,0]
        ]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0,0,0],
            vec![0,0,0],
            vec![0,0,1]
        ]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0,1,0],
            vec![1,0,0],
            vec![0,0,0]
        ]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0,0,0],
            vec![1,1,0],
            vec![0,0,0]
        ]), 1);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![1],
        ]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![0],
        ]), 1);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![
            vec![1,0,0],
            vec![1,1,0],
            vec![0,0,0]
        ]), 0);
    }
}