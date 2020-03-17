fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            Self::into1(&mut grid, i, 0);
            Self::into1(&mut grid, i, n - 1);
        }
        for j in 0..n {
            Self::into1(&mut grid, 0, j);
            Self::into1(&mut grid, m - 1, j);
        }
        let mut res = 0;
        for i in 1..m-1 {
            for j in 1.. n-1 {
                if grid[i][j] == 0 {
                    // if is land
                    res += 1;
                    Self::into1(&mut grid, i, j);
                }
            }
        }
        res
    }

    pub fn into1(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if grid[i][j] == 1 { return }
        grid[i][j] = 1;
        let m = grid.len();
        let n = grid[0].len();
        if i >= 1 { Self::into1(grid, i - 1, j) }
        if i + 1 < m { Self::into1(grid, i + 1, j) }
        if j >= 1 { Self::into1(grid, i, j - 1) }
        if j + 1 < n { Self::into1(grid, i, j + 1) }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::closed_island(vec![
            vec![1,1,1,1,1,1,1,0],
            vec![1,0,0,0,0,1,1,0],
            vec![1,0,1,0,1,1,1,0],
            vec![1,0,0,0,0,1,0,1],
            vec![1,1,1,1,1,1,1,0]
        ]), 2);
        assert_eq!(Solution::closed_island(vec![
            vec![0,0,1,0,0],
            vec![0,1,0,1,0],
            vec![0,1,1,1,0]
        ]), 1);
        assert_eq!(Solution::closed_island(vec![
            vec![1,1,1,1,1,1,1],
            vec![1,0,0,0,0,0,1],
            vec![1,0,1,1,1,0,1],
            vec![1,0,1,0,1,0,1],
            vec![1,0,1,1,1,0,1],
            vec![1,0,0,0,0,0,1],
            vec![1,1,1,1,1,1,1]
        ]), 2);
    }
}