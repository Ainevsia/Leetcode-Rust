fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32 - 2;
        while i >= 0 {
            if nums[i as usize] < nums[i as usize + 1] {
                break
            } else {
                i -= 1;
            }
        }
        if i < 0 {
            nums.reverse()
        } else {
            let mut j = nums.len() - 1;
            while i < j as i32 {
                if nums[i as usize] < nums[j] {
                    break
                } else {
                    j -= 1;
                }
            }
            nums.swap(i as usize, j);
            nums[i as usize + 1..j].reverse()
        }
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
}
