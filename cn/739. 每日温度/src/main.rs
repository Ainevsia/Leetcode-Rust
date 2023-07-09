fn main() {
    Solution::largest_rectangle_area(vec! [1,1]);
}

struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(v: Vec<i32>) -> i32 {
        let n = v.len();
        let mut left_smaller_idx = vec![-1; n];
        let mut right_smaller_idx = vec![n as i32; n];
        for i in 1..n {
            let mut mid = i as i32 - 1;
            while mid >= 0 && v[mid as usize] >= v[i] {
                mid = left_smaller_idx[mid as usize];
            }
            left_smaller_idx[i] = mid;
        }
        for i in (0..n-1).rev() {
            let mut mid = i + 1;
            while mid < n && v[mid] >= v[i] {
                mid = right_smaller_idx[mid] as usize;
            }
            right_smaller_idx[i] = mid as i32;
        }
        let mut res = 0;
        for (idx, &e) in v.iter().enumerate() {
            res = res.max((right_smaller_idx[idx] - left_smaller_idx[idx] - 1) * e);
        }
        dbg!(res)
    }
}


impl Solution {
    pub fn largest_rectangle_area1(mut v: Vec<i32>) -> i32 {
        v.insert(0, 0);  // 便于使第一个元素能够有左侧<=它的值
        v.push(0);                      // 便于在结束处理最后一个元素后清空残留在栈中的值
        let mut res = 0;
        let mut stack = vec![]; // 递增的栈
        for (idx, &e) in v.iter().enumerate() {
            while !stack.is_empty() && v[*stack.last().unwrap()] > e {
                let pos = stack.pop().unwrap();
                let prev_pos = *stack.last().unwrap();
                let s = (idx - prev_pos - 1) as i32 * v[pos];
                res = res.max(s);
            }
            stack.push(idx);
        }
        res
    }  
}


impl Solution {
    pub fn largest_rectangle_area2(mut v: Vec<i32>) -> i32 {
        v.insert(0, -1);
        v.push(-1);
        let n = v.len();
        let mut right_idx = vec![0; n];
        let mut stack = vec![]; // 递增的栈
        // 找到右侧第一个比e小的元素下表
        for (idx, &e) in v.iter().enumerate() {
            while !stack.is_empty() && v[*stack.last().unwrap()] > e {
                let pos = stack.pop().unwrap();
                right_idx[pos] = idx as i32;
            }
            stack.push(idx);
        }
        let mut left_idx = vec![n as i32; n];
        let mut stack = vec![];
        for (idx, &e) in v.iter().enumerate().rev() {
            while !stack.is_empty() && v[*stack.last().unwrap()] > e {
                let pos = stack.pop().unwrap();
                left_idx[pos] = idx as i32;
            }
            stack.push(idx);
        }
        let mut res = 0;
        for (idx, &e) in v.iter().enumerate() {
            res = res.max(e);
            let width = right_idx[idx] - left_idx[idx] - 1;
            if width > 0 {
                res = res.max(e*width);
            }
        }
        // dbg!(res);
        // dbg!(left_idx);
        // dbg!(right_idx);
        res
    }  
}

impl Solution {
    pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut prefix_sum = vec![0; n];
        prefix_sum[0] = nums[0];
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i-1] + nums[0];
        }
        let s: i32 = nums.iter().sum();
        if s % k != 0 {
            return false
        }
        let target = s / k;
        for i in 0..n {
            // if 
        }
        true
    }
}

impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut dp = vec![vec![0; 2]; n];
        dp[0][0] = 1;
        dp[0][1] = 1;
        for i in 1..n {
            if nums1[i] >= nums1[i-1] && nums1[i] >= nums2[i-1] {
                dp[i][0] = dp[i-1][1].max(dp[i-1][0]) + 1;
            } else if nums1[i] >= nums1[i-1] {
                dp[i][0] = dp[i-1][0] + 1
            } else if nums1[i] >= nums2[i-1] {
                dp[i][0] = dp[i-1][1] + 1
            } else {
                dp[i][0] = 1;
            }
            if nums2[i] >= nums1[i-1] && nums2[i] >= nums2[i-1] {
                dp[i][1] = dp[i-1][1].max(dp[i-1][0]) + 1;
            } else if nums2[i] >= nums1[i-1] {
                dp[i][1] = dp[i-1][0] + 1
            } else if nums2[i] >= nums2[i-1] {
                dp[i][1] = dp[i-1][1] + 1
            } else {
                dp[i][1] = 1;
            }
        }
        let mut res = 1;
        for i in 0..n {
            res = res.max(dp[i][0].max(dp[i][1]))
        }
        res
    }
}

impl Solution {
    pub fn max_non_decreasing_length2(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut res = 1;
        let mut choose_min = vec![0; n];
        for i in 0..n {
            if choose_min[i] == 1 { continue }
            let mut last_elem = nums1[i].min(nums2[i]);
            for j in i + 1..n {
                let cmin = nums1[j].min(nums2[j]);
                let cmax = nums1[j].max(nums2[j]);
                if cmax < last_elem {
                    break
                } else {
                    if cmin < last_elem {
                        last_elem = cmax;
                    } else if cmin >= last_elem {
                        last_elem = cmin;
                        choose_min[j] = 1;
                    }
                    res = res.max(j - i + 1);
                }
            }
        }
        res as i32
    }
}

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        // 初始 不可达
        let mut dp = vec![-1; n];    // 表示从这个位置到目标状态的最大跳跃次数
        dp[n-1] = 0;
        for i in (0..n-1).rev() {   // 从倒数第二课开始往前遍历
            let mut m = -1;
            for j in i+1..n {
                if (nums[j]-nums[i]).abs() <= target {
                    m = m.max(dp[j]);
                }
            }
            if m > -1 {
                dp[i] = m + 1;
            }
        }
        println!("{:?}", dp);
        dp[0]
    }
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_left = vec![0; height.len()];
        let mut max_right = vec![0; height.len()];
        max_left.iter_mut().zip(max_right.iter_mut().rev()).enumerate().fold((0, 0), |(lm, rm), (idx, (x, y))| {
            let lmax = lm.max(height[idx]);
            let rmax = rm.max(height[n - 1 - idx]);
            *x = lmax; *y = rmax;
            (lmax, rmax)
        });
        height.iter().enumerate().fold(0, |acc, (idx, x)| {
            let h = max_left[idx].min(max_right[idx]);
            if h > 0 { h - x + acc } else { acc }
        })
    }
}
impl Solution {
    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut ans = 0;
        for (right_pos, &right_h) in height.iter().enumerate() {
            while !stack.is_empty() && height[*stack.last().unwrap()] <= right_h {
                let mid_pos = stack.pop().unwrap();
                if !stack.is_empty() {
                    let left_pos = *stack.last().unwrap();
                    let left_h = height[left_pos];
                    let top = std::cmp::min(left_h, right_h);
                    if top > height[mid_pos] {
                        ans += (top - height[mid_pos]) * (right_pos - left_pos - 1) as i32;
                    }
                }
            }
            stack.push(right_pos);
        }
        ans
    }

}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums.len() * 2];
        let mut stack = vec![];
        let double = nums.repeat(2);
        for (idx, &i) in double.iter().enumerate() {
            while !stack.is_empty() && double[*stack.last().unwrap()] < i {
                let pos = stack.pop().unwrap();
                ans[pos] = i;
            }
            stack.push(idx);
        }
        ans.into_iter().take(nums.len()).collect()
    }
}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums1.len()];
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (idx, &i) in nums1.iter().enumerate() {
            map.insert(i, idx);
        }
        let mut stack = vec![];
        for (idx, &i) in nums2.iter().enumerate() {
            while !stack.is_empty() && nums2[*stack.last().unwrap()] < i {
                let pos = stack.pop().unwrap();
                if let Some(&jdx) = map.get(&nums2[pos]) {
                    ans[jdx] = i;
                }
            }
            stack.push(idx);
        }
        ans
    }
}


#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn basic1() {
        assert_eq!(
            Solution::next_greater_elements(
                vec![1,2,1]), 
            vec![2,-1,2]
        );
    }

    #[test]
    fn basic() {
        assert_eq!(
            Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]), 
            vec![1,1,4,2,1,1,0,0]
        );
    }
}

impl Solution {
    /// 单调栈的本质是以空间换时间，记录之前已访问过的非递增子序列下标
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack = vec![];
        for (idx, &value) in temperatures.iter().enumerate() {
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] < value {
                // 弹出，并计算res中对应位置的值
                let i = stack.pop().unwrap();
                res[i] = (idx - i) as i32;
            }
            // 入栈
            stack.push(idx)
        }
        res
    }
}
