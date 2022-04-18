fn main() {
    println!("Hello, world!");
    Solution::lexical_order(13);
}

struct Solution {}

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..=9 {
            if i <= n {
                res.push(i);
                Self::add(n,&mut res,i);
            } else { break }
        }
        res
    }

    fn add(n: i32, res: &mut Vec<i32>, base: i32) {
        for i in 0..=9 {
            let next_num = base * 10 + i;
            if next_num <= n {
                res.push(next_num);
                Self::add(n, res, next_num);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::lexical_order(13),vec![1,10,11,12,13,2,3,4,5,6,7,8,9]);
        assert_eq!(Solution::lexical_order(2),vec![1,2]);
        assert_eq!(Solution::lexical_order(18),vec![1,10,11,12,13,14,15,16,17,18,2,3,4,5,6,7,8,9]);
    }
}

