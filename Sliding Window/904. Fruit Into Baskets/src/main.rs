fn main() {
    Solution::total_fruit(vec![1,2,1]);
}

struct Solution {}

impl Solution {
    pub fn total_fruit(tree: Vec<i32>) -> i32 {
        if tree.len() <= 1 { return tree.len() as i32 }
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(3);
        let (mut s, mut e, k) = (0, 0, 2);
        let mut max = 0;
        while e < tree.len() {
            println!("s = {:#?}, e = {:#?}", s, e);
            println!("map = {:#?}", map);
            map.entry(tree[e]).and_modify(|x| *x += 1).or_insert(1);
            e += 1;
            if map.len() <= k {
                println!("max = {:#?}", max);
                max = if max < e - s { e - s } else { max }
            }
            while map.len() > k {
                map.entry(tree[s]).and_modify(|x| *x -= 1);
                if let Some(0) = map.get(&tree[s]) {
                    map.remove(&tree[s]);
                }
                s += 1;
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::total_fruit(vec![1,2,1]), 3);
        assert_eq!(Solution::total_fruit(vec![0,1,2,2]), 3);
        assert_eq!(Solution::total_fruit(vec![1,2,3,2,2]), 4);
        assert_eq!(Solution::total_fruit(vec![3,3,3,1,2,1,1,2,3,3,4]), 5);
        assert_eq!(Solution::total_fruit(vec![3]), 1);
        assert_eq!(Solution::total_fruit(vec![]), 0);
        assert_eq!(Solution::total_fruit(vec![3,3]), 2);
        assert_eq!(Solution::total_fruit(vec![3,4]), 2);


    }

    #[test]
    fn fail() {
        assert_eq!(Solution::total_fruit(vec![3,3,3,1,2,1,1,2,3,3,4]), 5);
    }
}