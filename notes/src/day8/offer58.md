# 剑指 Offer 58 - II. 左旋转字符串

## 题目描述

字符串的左旋转操作是把字符串前面的若干个字符转移到字符串的尾部。请定义一个函数实现字符串左旋转操作的功能。比如，输入字符串"abcdefg"和数字2，该函数将返回左旋转两位得到的结果"cdefgab"。

## 解题思路

想不到的思路

```rust

struct Solution {}

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let mut v: Vec<char> = s.chars().collect();
        Self::reverse2(&mut v[..n as usize]);
        Self::reverse2(&mut v[n as usize..]);
        Self::reverse2(&mut v[..]);
        v.iter().collect()
    }
    
    pub fn reverse2(s: &mut [char]) {
        let n = s.len();
        for i in 0..n/2 {
            let tmp = s[i];
            s[i] = s[n-1-i];
            s[n-1-i] = tmp;
        }
    }
}
```

## 学习感想