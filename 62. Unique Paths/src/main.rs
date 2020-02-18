fn main() {
    assert_eq!(Solution::unique_paths(3, 2), 3);
}

struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut vec = vec![1; n as usize];
        for _ in 0..m as usize - 1 {
            for i in 1..n as usize {
                vec[i] = vec[i] + vec[i-1];
            }
        }
        vec[n as usize - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(1, 2), 1);
        assert_eq!(Solution::unique_paths(2, 1), 1);
    }
}