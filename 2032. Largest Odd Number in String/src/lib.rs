pub struct Solution {}

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let s1: HashSet<i32> = HashSet::from_iter(nums1);   // nums1 moved
        let s2 = HashSet::from_iter(nums2);
        let s3 = HashSet::from_iter(nums3);
        let s12: HashSet<i32> = s1.intersection(&s2).copied().collect();
        let s13 = s1.intersection(&s3).copied().collect();
        let s23 = s2.intersection(&s3).copied().collect();
        let u: HashSet<i32> = s12.union(&s13).copied().collect();
        let u: HashSet<i32> = u.union(&s23).copied().collect();
        u.iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::two_out_of_three(vec![1,1,3,2], vec![2,3], vec![3]).len(), 2);
        assert_eq!(Solution::two_out_of_three(vec![1,3], vec![2,3], vec![1,2]).len(), 3);
        assert_eq!(Solution::two_out_of_three(vec![1,2,2], vec![4,3,3], vec![5]).len(), 0);
    
    }
}
