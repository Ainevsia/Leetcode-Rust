pub struct Solution {}

const HIGH_BIT: i32 = 14;

#[derive(Debug)]
pub struct Trie {
    sum: i32,
    left: Option<Box<Trie>>,
    right: Option<Box<Trie>>,
}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        Self::f(&nums, high) - Self::f(&nums, low-1)
    }

    pub fn f(nums: &Vec<i32>, x: i32) -> i32 {
        let mut root: Trie = Trie { sum: 0, left: None, right: None };
        let mut res = 0;
        for i in 1..nums.len() {
            Self::add(&mut root, nums[i - 1]);
            res += Self::get(&mut root, nums[i], x);
        }
        // println!("{:#?}", root);
        // let a: Trie = root;
        res
    }

    pub fn get(root: &mut Trie, num: i32, x: i32) -> i32 {
        let mut sum = 0;
        let mut cur: &mut Trie = root;
        for k in (0..=HIGH_BIT).rev() {
            let r = (num >> k) & 1;
            if (x >> k) & 1 == 0 {
                if r == 0 {
                    // left
                    if cur.left.is_none() { return sum }
                    cur = cur.left.as_mut().unwrap();
                } else {
                    // right
                    if cur.right.is_none() { return sum }
                    cur = cur.right.as_mut().unwrap();
                }
            } else {
                if r == 0 { // left
                    if let Some(x) = &cur.left {
                        sum += x.sum;
                    }
                    if cur.right.is_none() { return sum }
                    cur = cur.right.as_mut().unwrap(); 
                } else {
                    if let Some(x) = &cur.right { sum += x.sum }
                    if cur.left.is_none() { return sum }
                    cur = cur.left.as_mut().unwrap();
                }
            }
        }
        sum += cur.sum;
        sum
    }

    pub fn add(root: &mut Trie, num: i32) {
        let mut cur: &mut Trie = root;
        for k in (0..=HIGH_BIT).rev() {
            let bit = (num >> k) & 1;
            if bit == 1 && cur.right.is_none() {
                cur.right = Some(Box::new(Trie { sum: 0, left: None, right: None }));
            } else if bit == 0 && cur.left.is_none() {
                cur.left = Some(Box::new(Trie { sum: 0, left: None, right: None }));
            }
            if bit == 1 {
                let a: &mut Box<Trie> = cur.right.as_mut().unwrap();
                cur = a;
            } else if bit == 0 {
                let a: &mut Box<Trie> = cur.left.as_mut().unwrap();
                cur = a;
            }
            cur.sum += 1;
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_trie() {
        Solution::f(&vec![1,2,3], 1);
        // unimplemented!()
    }

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_pairs(vec![1,4,2,7],2,6),6);
        assert_eq!(Solution::count_pairs(vec![9,8,4,2,1],5,14),8);
    }
}
