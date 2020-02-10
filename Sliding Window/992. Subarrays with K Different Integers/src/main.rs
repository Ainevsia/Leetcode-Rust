fn main() {
    Solution::subarrays_with_k_distinct(vec![1,2,1,2,3], 2);
}

struct Solution {}

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let (mut start, mut end, mut res) = (0, 0, 0);
        while end < a.len() {
            map.entry(a[end]).and_modify(|x| *x += 1).or_insert(1);
            end += 1;
            
            while map.len() > k {   // if too long, shrink to k
                map.entry(a[start]).and_modify(|x| *x -= 1);
                if map.get(&a[start]).unwrap() == &0 {
                    map.remove(&a[start]);
                }
                start += 1;
            }
            
            if map.len() == k {
                let mut tmap = map.clone();
                let mut inc = 0;
                while tmap.len() == k {
                    tmap.entry(a[start + inc]).and_modify(|x| *x -= 1);
                    if tmap.get(&a[start + inc]).unwrap() == &0 {
                        tmap.remove(&a[start + inc]);
                    }
                    inc += 1;
                }
                res += inc;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1,2,1,2,3], 2), 7);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1,2,1,3,4], 3), 3);

    }

    #[test]
    fn edge() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1,2,1,2,3], 3), 3);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1], 3), 0);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![], 3), 0);


    }
}