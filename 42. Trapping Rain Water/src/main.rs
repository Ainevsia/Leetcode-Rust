fn main() {
    let x = Solution::trap(vec![2,0,2]);
    println!("x = {:#?}", x);
}

struct Solution {}

impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        while height.len() >= 2 && height[0] <= height[1] { height.remove(0); }
        while height.len() >= 2 && height[height.len()-1] <= height[height.len()-2] { height.pop(); }
        if height.len() <= 2 { return 0 }
        let mut hidx = 0;
        let mut h = height[hidx];
        let mut water = 0;
        for i in 2..height.len() {
            let origin = height[i];
            let target = std::cmp::min(h, height[i]);
            for j in hidx+1..i {
                if height[j] > target { continue }
                water += target - height[j];
                height[j] = target;
            }
            if origin >= h {
                hidx = i;
                if origin > h { h = height[hidx] }
            }
        }
        water
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
