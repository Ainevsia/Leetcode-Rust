fn main() {
    let mat = vec![
        vec![1,1,0,0,0],
        vec![1,1,1,1,0],
        vec![1,0,0,0,0],
        vec![1,1,0,0,0],
        vec![1,1,1,1,1]
    ];
    Solution::k_weakest_rows(mat, 3);
}

struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut x: Vec<(i32, usize)> = mat.iter().enumerate().map(|(i, x)| (x.iter().sum(), i)).collect();
        x.sort();
        let mut x: Vec<i32> = x.into_iter().map(|(_, i)| i as i32).collect();
        while x.len() > k as usize {
            x.pop();
        }
        // println!("x = {:#?}", x);
        x
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let mat = vec![
            vec![1,1,0,0,0],
            vec![1,1,1,1,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,0],
            vec![1,1,1,1,1]
        ];
        assert_eq!(Solution::k_weakest_rows(mat, 3), vec![2,0,3]);
    }
}
