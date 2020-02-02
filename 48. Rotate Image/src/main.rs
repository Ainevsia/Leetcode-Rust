fn main() {
    let mut input = vec![
        vec![ 5, 1, 9,11],
        vec![ 2, 4, 8,10],
        vec![13, 3, 6, 7],
        vec![15,14,12,16]
    ];
    let mut input = vec![
        vec![ 5, 1, 9,11,12],
        vec![ 2, 4, 8,10,11],
        vec![13, 3, 6, 7,13],
        vec![15,14,12,16,14],
        vec![ 5, 2, 1, 9,11],
    ];
    Solution::rotate(&mut input);
}

struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut dimension = matrix.len();
        let mut offset = 0;
        while dimension > 1 {
            let buflen = matrix.len() - 1 - offset * 2;
            let mut buf: Vec<i32> = Vec::with_capacity(buflen);
            for i in 0..buflen {
                buf.push(matrix[offset][offset + i]);
            }
            println!("buf = {:#?}", buf);
            // rotate
            for i in 0..buflen {
                matrix[offset + 0]         [offset + i]         = matrix[offset + buflen - i][offset + 0];
                matrix[offset + buflen - i][offset + 0]         = matrix[offset + buflen]    [offset + buflen - i];
                matrix[offset + buflen]    [offset + buflen - i]= matrix[offset + i]         [offset + buflen];
                matrix[offset + i]         [offset + buflen]    = buf[i];
            }
            dimension -= 2;
            offset += 1;
            println!("matrix = {:?}", matrix);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let mut input = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ];
        let output = vec![
            vec![7,4,1],
            vec![8,5,2],
            vec![9,6,3]
        ];
        Solution::rotate(&mut input);
        assert_eq!(input, output);

        let mut input = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16]
        ];
        let output = vec![
            vec![15,13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7,10,11]
        ];
        Solution::rotate(&mut input);
        assert_eq!(input, output);
    }
}