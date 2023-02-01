pub struct Solution {}

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        if num == 1 { return 0 }
        let x = num.to_string();
        let mut sum = 0;
        for c in x.chars() {
            let n = c as u8 - '0' as u8;
            if n & 1 == 1 { sum ^= 1 }
        }
        if sum == 1 {
            (num - 1) / 2
        } else {
            num / 2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_even(1), 0); // even
        assert_eq!(Solution::count_even(2), 1);
        assert_eq!(Solution::count_even(3), 1);
        assert_eq!(Solution::count_even(4), 2);
        assert_eq!(Solution::count_even(5), 2);
        assert_eq!(Solution::count_even(6), 3); // even
        assert_eq!(Solution::count_even(7), 3);
        assert_eq!(Solution::count_even(8), 4); // even
        assert_eq!(Solution::count_even(9), 4); // odd   same root o
        assert_eq!(Solution::count_even(10), 4); // even same root o
        assert_eq!(Solution::count_even(11), 5);
        assert_eq!(Solution::count_even(12), 5);
        assert_eq!(Solution::count_even(13), 6);
        assert_eq!(Solution::count_even(14), 6);
        assert_eq!(Solution::count_even(15), 7);
        assert_eq!(Solution::count_even(16), 7);
        assert_eq!(Solution::count_even(17), 8);
        assert_eq!(Solution::count_even(18), 8);    // even
        assert_eq!(Solution::count_even(19), 9);    // odd   same root e
        assert_eq!(Solution::count_even(20), 10);    // even same root e

        assert_eq!(Solution::count_even(30), 14);
        assert_eq!(Solution::count_even(63), 31);
    }
}
