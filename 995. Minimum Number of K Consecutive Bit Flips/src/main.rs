fn main() {
    Solution::min_k_bit_flips(vec![0,1,0], 1);
    println!("-1%2 = {:#?}", -1%2);
}

struct Solution {}

impl Solution {
    /// do you know what "code is comment itself" means ?
    pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut res = 0;
        let mut flipped = vec![0; a.len()];
        let mut flip_in_pre_wnd = 0;
        for i in 0..a.len() {
            if i >= k {
                // minus 1 if needed
                flip_in_pre_wnd ^= flipped[i - k];
            }
            if flip_in_pre_wnd == a[i] {
                // need to flip this bit
                flip_in_pre_wnd ^= 1;   // pluss 1
                flipped[i] = 1;
                res += 1;
                if i + k > a.len() { return -1 }
            }
        }
        if a.len() >0 { res } else { - 1 }
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