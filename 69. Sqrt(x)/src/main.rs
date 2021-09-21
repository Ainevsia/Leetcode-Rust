fn main() {
    assert_eq!(Solution::my_sqrt(17),4);
}

struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 { return 0 }
        // 16 10000
        let idx = std::mem::size_of::<i32>() as u32 * 8 - 1 - x.leading_zeros();
        // x is not zero, x.leading_zeros() <= 31
        let mut cur_idx = idx / 2;
        let target = x as usize;
        let mut res = 0;
        loop {
            res |= 1 << cur_idx;
            let cur_res = res * res;
            if cur_res == target { break }
            else if cur_res > target { res &= !(1 << cur_idx) }
            if cur_idx == 0 { break }
            cur_idx -= 1;
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::my_sqrt(0),0);
        assert_eq!(Solution::my_sqrt(1),1);
        assert_eq!(Solution::my_sqrt(15),3);
        assert_eq!(Solution::my_sqrt(16),4);
        assert_eq!(Solution::my_sqrt(17),4);
        assert_eq!(Solution::my_sqrt(18),4);
        assert_eq!(Solution::my_sqrt(i32::MAX),46340);
    }
}