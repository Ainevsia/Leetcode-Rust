fn main() {
    Solution::sorted_squares(vec![-4,-1,0,3,10]);
}

struct Solution {}

impl Solution {
    /// two pointers
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        if a.len() < 1 { return vec![] }
        let (mut start, mut end) = (0, a.len()-1);
        let mut vec = vec![];
        loop {
            if a[start].abs() > a[end].abs() {
                vec.insert(0, a[start].pow(2));
                if start == end { break vec }
                start += 1;
            } else {
                vec.insert(0, a[end].pow(2));
                if start == end { break vec }
                end -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
        assert_eq!(Solution::sorted_squares(vec![-7,-3,2,3,11]), vec![4,9,9,49,121]);
        assert_eq!(Solution::sorted_squares(vec![-7]), vec![49]);

    }
}