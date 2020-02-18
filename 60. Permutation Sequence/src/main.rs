fn main() {
    assert_eq!(Solution::get_permutation(3, 3), "213");
}

struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let range = '1'..'9';
        let x = range.count();
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
    }
}