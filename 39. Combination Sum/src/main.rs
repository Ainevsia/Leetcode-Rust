fn main() {
    Solution::combination_sum(vec![2,3,5], 8);
}

struct Solution {}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut buf = vec![];
        let mut target_buf = vec![];
        
        Self::sum(&candidates, target, &mut buf, &mut target_buf);
        println!("buf = {:#?}", target_buf);
        target_buf
    }

    pub fn sum( candidates: &[i32],
                target: i32,
                buf: &mut Vec<i32>,
                target_buf: &mut Vec<Vec<i32>> ) {
        if target == 0 { target_buf.push(buf.clone()) }
        if target < 0 { return }
        // println!(" -> target = {:#?}", target);
        let mut e = candidates.len() - 1;
        while e > 0 && candidates[e] > target { e -= 1; }
        for i in 0..=e {
            // println!("testing candidates[i] = {:#?}", candidates[i]);
            buf.push(candidates[i]);
            Self::sum(&candidates[i..], target - candidates[i], buf, target_buf);
            buf.pop();
        }
        // println!(" <- ");
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::combination_sum(
            vec![2,3,6,7], 7),
            vec![
                vec![2,2,3],
                vec![7]
            ]
        );
        assert_eq!(Solution::combination_sum(
            vec![2,3,5], 8),
            vec![
                vec![2,2,2,2],
                vec![2,3,3],
                vec![3,5]
            ]
        );
    }
}
