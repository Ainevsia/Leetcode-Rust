fn main() {
    assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
}

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            if map.contains_key(&i) {
                return true;
            } else {
                map.insert(i, 1);
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
}