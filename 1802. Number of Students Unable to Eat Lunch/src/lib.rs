pub struct Solution {}

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let (n, index, max_sum) = (n as usize, index as usize, max_sum as usize);
        let l = index.min(n - index - 1);
        let r = n - 1 - l;
        let s = max_sum - n;
        let s1 = l * l;
        let s2 = r * (r - 1) / 2 + (l + 1) * (2 * r - l) / 2; // i32 may mul overflow
        // dbg!(l, r, s, s1, s2);
        if s <= s1 { return (s as f64).sqrt() as i32 + 1 }
        else if s >= s2 { return (r + (s - s2) / n + 1) as i32 }
        else {
            let delta_s = (s - s1) as f64;
            let t = (4 * l + 1) as f64 / 2f64;
            let delta_x = ((2f64 * delta_s + t * t).sqrt() - t) as i32;
            // dbg!(delta_x);
            delta_x + 1 + l as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn adv() {
        assert_eq!(Solution::max_value(4116541,2948244,392357701), 19704);
    }

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_value(4,2,6), 2);
        assert_eq!(Solution::max_value(6,1,10), 3);
        assert_eq!(Solution::max_value(16,4,200), 17);
        assert_eq!(Solution::max_value(2,1,865959216), 432979608);
    }

    #[test]
    fn my_test() {
        assert_eq!(Solution::max_value(7,2,7+3), 2);
        // 1 1 1 1 1 1 1
        //     ^
        // 1 1 2 1 1 1 1 (8)
        assert_eq!(Solution::max_value(7,2,7+4), 3);
        // 1 2 3 2 1 1 1 (11)
        assert_eq!(Solution::max_value(7,2,7+15), 4+1);
        // 2 3 4 3 2 1 0 (15) delta
        assert_eq!(Solution::max_value(7,2,7+15+7), 4+1+1);
        
        assert_eq!(Solution::max_value(7,2,7+4+4), 3);
        assert_eq!(Solution::max_value(7,2,7+4+5), 4);
        assert_eq!(Solution::max_value(7,2,7+4+5+5), 4);
        assert_eq!(Solution::max_value(7,2,7+4+5+6), 5);
    }
}
