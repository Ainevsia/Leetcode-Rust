fn main() {
    assert_eq!(Solution::missing_number(vec![3,0,1]), 2);
}

struct Solution {}

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.push(-1);
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == i as i32 || nums[i] == -1 { i += 1; }
            else {
                let target = nums[i] as usize;
                nums.swap(i, target);
            }
        }
        nums.iter().position(|&x| x == -1).unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::missing_number(vec![3,0,1]), 2);
        assert_eq!(Solution::missing_number(vec![9,6,4,2,3,5,7,0,1]), 8);

    }
}