fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    ///  You should remember two things from this:
    // x xor x = 0
    // 0 xor x = x.
    // Now you have two more xor properties you can use to solve problems. Good Luck!
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |s, x| s^x)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::single_number(vec![1,2,3,2,1]), 3);
    }
}