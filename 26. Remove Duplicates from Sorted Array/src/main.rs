fn main() {
    Solution::remove_duplicates(&mut vec![1,1,2]);
}

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut vec = vec![];
        for i in nums.iter() {
            if Some(i) != vec.last() {
                vec.push(*i);
            }
        }
        for i in 0..vec.len() {
            nums[i] = vec[i];
        }
        vec.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1,1,2]), 2);
    }
}