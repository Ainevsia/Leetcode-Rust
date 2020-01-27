fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];
        for &i in nums.iter() {
            Self::permute_add(&mut res, i);
        }
        res.sort();
        res
    }

    pub fn permute_add(permutation: &mut Vec<Vec<i32>>, elem: i32) {
        if permutation.len() == 0 {
            permutation.push(vec![elem]);
            return
        }
        for _ in 0..permutation.len() {
            let base = permutation.remove(0);
            for i in 0..base.len() + 1 {
                let mut new_elem = base.clone();
                new_elem.insert(i, elem);
                permutation.push(new_elem);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::permute(vec![1,2,3]), 
            vec![
                vec![1,2,3],
                vec![1,3,2],
                vec![2,1,3],
                vec![2,3,1],
                vec![3,1,2],
                vec![3,2,1]
            ]
        );
    }

    #[test]
    fn permute_add_basic() {
        let mut input = vec![vec![]];
        Solution::permute_add(&mut input, 1);
        assert_eq!(
            input,
            vec![
                vec![1],
            ]
        );
        Solution::permute_add(&mut input, 2);
        assert_eq!(
            input,
            vec![
                vec![2,1],
                vec![1,2],
            ]
        );
        Solution::permute_add(&mut input, 3);
        input.sort();
        assert_eq!(
            input,
            vec![
                vec![1,2,3],
                vec![1,3,2],
                vec![2,1,3],
                vec![2,3,1],
                vec![3,1,2],
                vec![3,2,1]
            ]
        );
    }
}
