use std::panic;

 pub struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 1 { return nums }
        nums.sort();
        let mut f = vec![0; n];
        let mut g = vec![0; n];
        f[0] = 1;
        g[0] = usize::MAX;
        for i in 1..n {
            let mut t = vec![0; i];
            for j in 0..i { if nums[i] % nums[j] == 0 { t[j] = f[j]; } }
            if let Some((idx, &cnt)) = t.iter().enumerate().max_by_key(|&(_, &f)| f) {
                // must match
                if cnt == 0 {
                    f[i] = 1;
                    g[i] = usize::MAX;
                    continue
                }
                // append to longest
                f[i] = cnt + 1;
                g[i] = idx;
            }
        }
        // println!("[!] what ?");
        // dbg!(&f);
        // dbg!(&g);
        // panic!("sss");
        if let Some((idx, _)) = f.iter().enumerate().max_by_key(|&(_, &y)| y) {
            let mut res = vec![nums[idx]];
            let mut i = g[idx];
            while i != usize::MAX {
                res.push(nums[i]);
                i = g[i];
            }
            return res.into_iter().rev().collect()
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    
    #[test]
    fn it_works() {
        assert_eq!(Solution::largest_divisible_subset(vec![1,2,3]), vec![1,3]);
        assert_eq!(Solution::largest_divisible_subset(vec![1,2,4,8]), vec![1,2,4,8]);
        assert_eq!(Solution::largest_divisible_subset(vec![9,18,54,90,108,180,360,540,720]), vec![9,18,90,180,360,720]);
        assert_eq!(Solution::largest_divisible_subset(vec![3,4,16,8]), vec![4,8,16]);

    }
}
