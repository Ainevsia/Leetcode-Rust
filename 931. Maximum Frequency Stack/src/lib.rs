pub struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        for i in 1..n { for j in 0..n {
            if j == 0 { matrix[i][j] += matrix[i-1][j].min(matrix[i-1][j+1]) }
            else if j == n - 1 { matrix[i][j] += matrix[i-1][j].min(matrix[i-1][j-1]) }
            else { matrix[i][j] += matrix[i-1][j].min(matrix[i-1][j+1]).min(matrix[i-1][j-1]) }
        }}
        *matrix[n-1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_falling_path_sum(
            vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]]), 13);
            assert_eq!(Solution::min_falling_path_sum(
                vec![vec![-19,57],vec![-40,-5]]), -59);
    }
}
