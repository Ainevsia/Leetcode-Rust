fn main() {
    Solution::combination_sum2(vec![10,1,2,7,6,1,5], 8);
}

struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        if candidates.len() == 0 { return }
        // println!(" -> target = {:#?}", target);
        let mut e = candidates.len() - 1;
        while e > 0 && candidates[e] > target { e -= 1; }
        let mut prev = -1;
        for i in 0..=e {
            if candidates[i] == prev { continue }
            // println!("testing candidates[i] = {:#?}", candidates[i]);
            buf.push(candidates[i]);
            Self::sum(&candidates[i+1..], target - candidates[i], buf, target_buf);
            prev = buf.pop().unwrap();
        }
        // println!(" <- ");
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::combination_sum2(
            vec![10,1,2,7,6,1,5], 8),
            vec![
                vec![1, 1, 6],
                vec![1, 2, 5],
                vec![1, 7],
                vec![2, 6],
            ]
        );
        assert_eq!(Solution::combination_sum2(
            vec![2,5,2,1,2], 5),
            vec![
                vec![1,2,2],
                vec![5]
            ]
        );
    }
}
