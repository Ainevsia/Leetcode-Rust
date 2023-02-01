pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        if n == 1 { return if x == nums[0] { 1 } else { -1 } }
        let mut ascending = vec![0; n];
        let mut desending = vec![0; n];
        let mut lsum = 0;
        let mut rsum = 0;
        for i in 0..n {
            lsum += nums[i];
            ascending[i] = lsum;
            rsum += nums[n - 1 - i];
            desending[n - 1  - i] = rsum;
        }
        if x > desending[0] { return -1 }
        if x == desending[0] { return n as i32 }
        let mut jdx = 0;
        let mut res = usize::MAX;
        while jdx < n - 1 && desending[jdx + 1] >= x { jdx += 1 }
        if desending[jdx] == x { res = res.min(n - jdx) }
        let mut idx = 0;
        // dbg!(&ascending, &desending);
        // dbg!(res);
        while idx < n && ascending[idx] <= x {
            while jdx < n && desending[jdx] + ascending[idx] > x { jdx += 1 }
            // dbg!(idx, jdx);
            if jdx == n {
                if ascending[idx] == x {
                    res = res.min(idx + 1);
                    return res as i32
                }
            } else if ascending[idx] + desending[jdx] == x {
                res = res.min(idx + 1 + n - jdx);
            }
            // dbg!(res);
            idx += 1;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(Solution::min_operations(vec![1,1,4,2,3], 5), 2);
        // assert_eq!(Solution::min_operations(vec![5,6,7,8,9], 4), -1);
        // assert_eq!(Solution::min_operations(vec![3,2,20,1,1,3], 10), 5);
        // assert_eq!(Solution::min_operations(vec![1,1,3,2,5], 5), 1);
        // assert_eq!(Solution::min_operations(vec![1,1,3,2,5], 12), 5);
        assert_eq!(Solution::min_operations(vec![5,2,3,1,1], 5), 1);
    }
}
