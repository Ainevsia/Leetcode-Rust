# 剑指 Offer 05. 替换空格

## 题目描述

请实现一个函数，把字符串 s 中的每个空格替换成"%20"。

## 解题思路

一开始没有想到用双指针

```rust
struct Solution {}
impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let cnt = s.iter().filter(|&&x| x == ' ').count();
        let n = s.len();
        s.resize(n + 2 * cnt, ' ');
        let mut tail = s.len() - 1;
        let mut head = n as isize - 1;
        while head >= 0 {
            if s[head as usize] == ' ' {
                s[tail] = '0';
                s[tail - 1] = '2';
                s[tail - 2] = '%';
                tail -= 3;
            } else {
                s[tail] = s[head as usize];
                tail -= 1;
            }
            head -= 1;
        }
        s.iter().collect()
    }
}
```
## 学习感想