fn main() {
    let x = Solution::trap(vec![2,0,2]);
    println!("x = {:#?}", x);
}

struct Solution {}

impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        while height.len() > 2 && height[0] <= height[1] { height.remove(0); }
        while height.len() > 2 && height[height.len()-1] <= height[height.len()-2] { height.pop(); }
        if height.len() <= 2 { return 0 }
        let mut l_max_height = height[0];
        let mut r_max_height = height[height.len() - 1];
        let mut h = vec![];
        for i in 0..height.len() {
            if height[i] <= l_max_height {
                h.push(l_max_height);
            } else if height[i] > l_max_height {
                h.push(l_max_height);
                l_max_height = height[i];
            }
        }
        for i in (0..height.len()).rev() {
            if height[i] <= r_max_height {
                h[i] = std::cmp::min(h[i], r_max_height);
            } else if height[i] > r_max_height {
                h[i] = std::cmp::min(h[i], r_max_height);
                r_max_height = height[i];
            }
        }
        for i in 0..height.len() {
            if height[i] >= h[i] { h[i] = 0 }
            else if height[i] < h[i] { h[i] -= height[i] }
        }
        h.iter().fold(0, |acc, &x| acc + x)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(Solution::trap(vec![0,1,2,0,1,3,1,2,0,3,0]), 9);
        assert_eq!(Solution::trap(vec![3,1,2,0,1,3,1,2,0,3,0]), 14);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::trap(vec![0,1,2,3,2,1,0]), 0);
        assert_eq!(Solution::trap(vec![0,0]), 0);
        assert_eq!(Solution::trap(vec![0,1,2,0]), 0);
    }

    #[test]
    fn leet() {
        assert_eq!(Solution::trap(vec![2,0,2]), 2);
        assert_eq!(Solution::trap(vec![5,4,1,2]), 1);
    }
}
