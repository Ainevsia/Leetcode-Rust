fn main() {
    Solution::generate_matrix(1);
}

struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        Self::fill(&mut matrix, 0, n - 1, 1);
        matrix
    }

    pub fn fill(matrix: &mut Vec<Vec<i32>>, offset: usize, width: usize, start: i32) {
        if width == 0 { matrix[offset][offset] = start; return }
        let (mut v1, mut v2, mut v3, mut v4) = 
            (start, start + width as i32, start + 2 * width as i32,start + 3 * width as i32);
        for i in 0..width {
            matrix[offset][offset+i] = v1;
            matrix[offset+i][offset+width] = v2;
            matrix[offset+width][offset+width-i] = v3;
            matrix[offset+width-i][offset] = v4;
            v1 += 1;v2 += 1;v3 += 1; v4 += 1;
        }
        if width >= 2 {
            Self::fill(matrix, offset + 1, width - 2, v4);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::generate_matrix(1), vec![
            vec![1],
        ]);
        assert_eq!(Solution::generate_matrix(2), vec![
            vec![ 1, 2 ],
            vec![ 4, 3 ],
        ]);
        assert_eq!(Solution::generate_matrix(3), vec![
            vec![ 1, 2, 3 ],
            vec![ 8, 9, 4 ],
            vec![ 7, 6, 5 ]
        ]);
    }
}