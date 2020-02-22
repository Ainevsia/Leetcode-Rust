fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len() - 1;
        loop {
            if digits[i] != 9 { digits[i] += 1; break digits }
            /* digits[i] == 9 */
            digits[i] = 0;
            if i == 0 { digits.insert(0, 1); break digits }
            i -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![9,9]), vec![1,0,0]);
    }
}