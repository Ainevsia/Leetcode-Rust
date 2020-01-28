fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 { return }
        let mut i = nums.len() - 2;
        while let false = nums[i] < nums[i + 1] {
            if i != 0 {
                i -= 1
            } else {
                nums.reverse();
                return
            }
        }
        let mut j = nums.len() - 1;
        while let false = nums[i] < nums[j] {
            j -= 1;
        }
        nums.swap(i, j);
        nums[i+1..].reverse()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic1() {
        let mut v1 = vec![1,2,3];
        let v2 = vec![1,3,2];
        Solution::next_permutation(&mut v1);
        assert_eq!(v1,v2);
    }

    #[test]
    fn basic2() {
        let mut v1 = vec![3,2,1];
        let v2 = vec![1,2,3];
        Solution::next_permutation(&mut v1);
        assert_eq!(v1,v2);
    }

    #[test]
    fn basic3() {
        let mut v1 = vec![1,1,5];
        let v2 = vec![1,5,1];
        Solution::next_permutation(&mut v1);
        assert_eq!(v1,v2);
    }

    #[test]
    fn edge() {
        let mut v1 = vec![1,1,1];
        let v2 = vec![1,1,1];
        Solution::next_permutation(&mut v1);
        assert_eq!(v1,v2);

        let mut v1 = vec![1];
        let v2 = vec![1];
        Solution::next_permutation(&mut v1);
        assert_eq!(v1,v2);

        let mut v1 = vec![];
        let v2 = vec![];
        Solution::next_permutation(&mut v1);
        assert_eq!(v1,v2);
    }
}
