# 1. 两数之和

## 题目描述

给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。

你可以按任意顺序返回答案。

 

## 解题思路

```rust
# struct Solution {}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (idx, i) in nums.iter().enumerate() {
            if map.contains_key(&(target - i)) {
                return vec![*map.get(&(target - i)).unwrap(), idx as i32]
            } else {
                map.insert(*i, idx as i32);
            }
        }
        todo!()
    }
}
```


## 学习感想