fn main() {
    Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]);
}

struct Solution {}

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

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]), 
            vec![1,1,4,2,1,1,0,0]);
    }
}