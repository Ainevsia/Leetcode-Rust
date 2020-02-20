fn main() {
    assert_eq!(Solution::super_egg_drop(1, 2), 2);
}

struct Solution {}

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {

        assert_eq!(Solution::super_egg_drop(1, 2), 2);
        assert_eq!(Solution::super_egg_drop(2, 6), 3);
        assert_eq!(Solution::super_egg_drop(3, 14), 4);
    }
}