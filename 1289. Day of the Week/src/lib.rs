pub struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let (mi, mii) = Self::f(&matrix[0]);
        for j in 0..n { matrix[0][j] = if matrix[0][j] == mi { mii } else { mi } }
        for i in 1..n {
            for j in 0..n { matrix[i][j] += matrix[i-1][j] }
            let (mi, mii) = Self::f(&matrix[i]);
            for j in 0..n { matrix[i][j] = if matrix[i][j] == mi { mii } else { mi } }
        }
        *matrix[n-1].iter().min().unwrap()
    }

    fn f(v: &Vec<i32>) -> (i32, i32) {
        // println!("{:#?}", v);
        let mi = *v.iter().min().unwrap();
        let idx = v.iter().position(|&x| x == mi).unwrap();
        let mii = *v[..idx].iter().chain(v[idx+1..].iter()).min().unwrap_or(&mi);
        (mi, mii)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_falling_path_sum(
            vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), 13);
        assert_eq!(Solution::min_falling_path_sum(
            vec![vec![7]]), 7);
        assert_eq!(Solution::min_falling_path_sum(
            vec![
                vec![-73, 61,43,-48,-36],
                vec![  3, 30,27, 57, 10],
                vec![ 96,-76,84, 59,-15],
                vec![  5,-49,76, 31, -7],
                vec![ 97,891,61,-46, 67]
            ]), -192);
    }
}
