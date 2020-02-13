fn main() {
    let mut input = vec![2,0,2,1,1,0];
    Solution::sort_colors(&mut input);
}

struct Solution {}

impl Solution {
    /// [one..=two] (both inclusive) are unordered
    /// three pointers
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 { return }
        let (mut zero, mut two) = (0, nums.len() - 1);
        while nums[two] == 2 {
            if let Some(t) = two.checked_sub(1) {
                two = t;
            } else { return }
        }
        while zero < nums.len() && nums[zero] == 0 {
            zero += 1;
        }
        let mut one = zero;
        while one <= two {
            if one == two {
                if nums[one] == 0 { nums.swap(zero, one) }
                return
            }
            if nums[one] == 2 {
                nums.swap(one, two);
                two -= 1;   // two cannot be 0 cause one < two
            } else if nums[one] == 0 {
                nums.swap(zero, one);
                zero += 1;
                one += 1;
            } else {
                one += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let mut input = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut input);
        let output = vec![0,0,1,1,2,2];
        assert_eq!(input, output);
    }

    #[test]
    fn edge() {
        let mut input = vec![2,2];
        Solution::sort_colors(&mut input);
        let output = vec![2,2];
        assert_eq!(input, output);
        let mut input = vec![1,1];
        Solution::sort_colors(&mut input);
        let output = vec![1,1];
        assert_eq!(input, output);
        let mut input = vec![0,0];
        Solution::sort_colors(&mut input);
        let output = vec![0,0];
        assert_eq!(input, output);

        let mut input = vec![0];
        Solution::sort_colors(&mut input);
        let output = vec![0];
        assert_eq!(input, output);
        let mut input = vec![];
        Solution::sort_colors(&mut input);
        let output = vec![];
        assert_eq!(input, output);
    }
}