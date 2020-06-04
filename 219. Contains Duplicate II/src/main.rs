fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(prev) = map.get_mut(&nums[i]) {
                if i - prev[prev.len() - 1] <= k as usize {
                    return true
                } else {
                    prev.push(i);
                }
            } else {
                map.insert(nums[i], vec![i]);
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
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,2,3,1], 3), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,0,1,1], 1), true);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2), false);
    }
}