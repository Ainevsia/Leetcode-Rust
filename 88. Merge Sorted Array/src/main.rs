fn main() {
    let mut input = vec![1,2,3,0,0,0];
    Solution::merge(&mut input, 3,&mut vec![2,5,6],3);
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let mut buffer = Vec::<i32>::with_capacity(m + n);
        let (mut i, mut j) = (0, 0);
        while i < m || j < n {
            if j == n || (i < m && nums1[i] <= nums2[j]) {
                buffer.push(nums1[i]);
                i += 1;
            } else {
                buffer.push(nums2[j]);
                j += 1;
            }
        }
        *nums1 = buffer;    // move into nums1
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let mut input = vec![1,2,3,0,0,0];
        Solution::merge(&mut input, 3,&mut vec![2,5,6],3);
        assert_eq!(input,vec![1,2,2,3,5,6]);

        // submit 1
        let mut input = vec![1];
        Solution::merge(&mut input, 1,&mut vec![],0);
        assert_eq!(input,vec![1]);
    }
}

