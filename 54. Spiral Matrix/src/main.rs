fn main() {
    let matrix = vec![
        vec![ 1, 2, 3 ],
        vec![ 4, 5, 6 ],
        vec![ 7, 8, 9 ]
    ];

    Solution::spiral_order(matrix);
}

struct Solution {}

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        if matrix.is_empty() { return vec![] }
        loop {
            Self::pop_line(&mut matrix, &mut res);
            if matrix.len() == 0 { break res }
            let width = matrix[0].len();
            matrix = matrix.into_iter().fold(vec![vec![]; width], |mut sum, x| {
                for i in 0..width {
                    sum[width - 1 - i].push(x[i])
                }
                sum
            });
        }
    }

    pub fn pop_line(martrix: &mut Vec<Vec<i32>>, res: &mut Vec<i32>) {
        res.append(&mut martrix.remove(0));
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let matrix = vec![
            vec![ 1, 2, 3 ],
            vec![ 4, 5, 6 ],
            vec![ 7, 8, 9 ]
        ];

        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1,2,3,6,9,8,7,4,5]
        );
    }

    #[test]
    fn edge() {
        assert_eq!(
            Solution::spiral_order(vec![]),
            vec![]
        );
    }
}