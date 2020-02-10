fn main() {
    Solution::min_k_bit_flips(vec![0,1,0], 1);
}

struct Solution {}

impl Solution {
    pub fn min_k_bit_flips(mut a: Vec<i32>, k: i32) -> i32 {
        if a.len() < k as usize { return -1 }
        let mut res = 0;
        for i in 0..=a.len() - k as usize {
            if a[i] == 0 {
                // flip
                for j in 0..k as usize {
                    a[i + j] = a[i + j] ^ 1;
                }
                res += 1;
            }
        }
        for i in a.len() - k as usize..a.len() {
            if a[i] != 1 {
                return -1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::min_k_bit_flips(vec![0,1,0], 1), 2);
        assert_eq!(Solution::min_k_bit_flips(vec![1,1,0], 2), -1);
        assert_eq!(Solution::min_k_bit_flips(vec![0,0,0,1,0,1,1,0], 3), 3);
        assert_eq!(Solution::min_k_bit_flips(vec![0], 3), -1);
        assert_eq!(Solution::min_k_bit_flips(vec![0,0,0], 3), 1);
        assert_eq!(Solution::min_k_bit_flips(vec![0,0,1], 3), -1);
        assert_eq!(Solution::min_k_bit_flips(vec![], 3), -1);
        assert_eq!(Solution::min_k_bit_flips(vec![1,1,1,1,1,1,1,0], 3), -1);





    }
}