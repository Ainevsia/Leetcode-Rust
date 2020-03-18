fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 { return 0 }
        let n = grid[0].len();
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    res += 1;
                    Self::into_water(&mut grid , i, j);
                }
            }
        }
        res
    }
    
    pub fn into_water(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if grid[i][j] == '0' { return; }
        grid[i][j] = '0';
        let m = grid.len();
        let n = grid[0].len();
        if i >= 1 { Self::into_water(grid, i-1, j) }
        if j >= 1 { Self::into_water(grid, i, j-1) }
        if i+1 < m { Self::into_water(grid, i+1, j) }
        if j+1 < n { Self::into_water(grid, i, j+1) }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::num_islands(vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0'],
        ]), 1);
    }
}