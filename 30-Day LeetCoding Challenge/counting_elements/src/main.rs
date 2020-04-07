fn main() {
    assert_eq!(Solution::count_elements(vec![1,2,3]), 2);
}

struct Solution {}

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for x in arr {
            // if not in the map: first insert 0 and then add 1
            *map.entry(x).or_insert(0) += 1;
        }
        let mut res = 0;
        for (idx, _) in &map {
            if map.contains_key(&(idx + 1)) { res += map.get(idx).unwrap(); }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::count_elements(vec![1,2,3]), 2);
        assert_eq!(Solution::count_elements(vec![1,1,3,3,5,5,7,7]), 0);
        assert_eq!(Solution::count_elements(vec![1,3,2,3,5,0]), 3);
        assert_eq!(Solution::count_elements(vec![1,1,2,2]), 2);
    }
}