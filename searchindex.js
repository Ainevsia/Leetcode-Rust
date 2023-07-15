Object.assign(window.search, {"doc_urls":["day1.html#第一章--数组part01","day1.html#今日任务","day1.html#704-二分查找","day1.html#27-移除元素","day1/lc704.html#704-二分查找","day1/lc704.html#题目描述","day1/lc704.html#解题思路","day1/lc704.html#学习感想"],"index":{"documentStore":{"docInfo":{"0":{"body":0,"breadcrumbs":3,"title":1},"1":{"body":2,"breadcrumbs":2,"title":0},"2":{"body":7,"breadcrumbs":3,"title":1},"3":{"body":3,"breadcrumbs":3,"title":1},"4":{"body":0,"breadcrumbs":4,"title":1},"5":{"body":24,"breadcrumbs":3,"title":0},"6":{"body":62,"breadcrumbs":3,"title":0},"7":{"body":46,"breadcrumbs":3,"title":0}},"docs":{"0":{"body":"","breadcrumbs":"Day 1 » 第一章 数组part01","id":"0","title":"第一章 数组part01"},"1":{"body":"数组理论基础，704. 二分查找，27. 移除元素 文章链接：https://programmercarl.com/%E6%95%B0%E7%BB%84%E7%90%86%E8%AE%BA%E5%9F%BA%E7%A1%80.html 题目建议： 了解一下数组基础，以及数组的内存空间地址，数组也没那么简单。","breadcrumbs":"Day 1 » 今日任务","id":"1","title":"今日任务"},"2":{"body":"题目建议： 大家能把 704 掌握就可以，35.搜索插入位置 和 34. 在排序数组中查找元素的第一个和最后一个位置 ，如果有时间就去看一下，没时间可以先不看，二刷的时候在看。 先把 704写熟练，要熟悉 根据 左闭右开，左闭右闭 两种区间规则 写出来的二分法。 题目链接：https://leetcode.cn/problems/binary-search/ 文章讲解：https://programmercarl.com/0704.%E4%BA%8C%E5%88%86%E6%9F%A5%E6%89%BE.html 视频讲解：https://www.bilibili.com/video/BV1fA4y1o715","breadcrumbs":"Day 1 » 704. 二分查找","id":"2","title":"704. 二分查找"},"3":{"body":"题目建议： 暴力的解法，可以锻炼一下我们的代码实现能力，建议先把暴力写法写一遍。 双指针法 是本题的精髓，今日需要掌握，至于拓展题目可以先不看。 题目链接：https://leetcode.cn/problems/remove-element/ 文章讲解：https://programmercarl.com/0027.%E7%A7%BB%E9%99%A4%E5%85%83%E7%B4%A0.html 视频讲解：https://www.bilibili.com/video/BV12A4y1Z7LP","breadcrumbs":"Day 1 » 27. 移除元素","id":"3","title":"27. 移除元素"},"4":{"body":"","breadcrumbs":"Day 1 » 704. 二分查找 » 704. 二分查找","id":"4","title":"704. 二分查找"},"5":{"body":"给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。 示例 1: 输入: nums = [-1,0,3,5,9,12], target = 9 输出: 4 解释: 9 出现在 nums 中并且下标为 4 示例 2: 输入: nums = [-1,0,3,5,9,12], target = 2 输出: -1 解释: 2 不存在 nums 中因此返回 -1","breadcrumbs":"Day 1 » 704. 二分查找 » 题目描述","id":"5","title":"题目描述"},"6":{"body":"直接使用标准库的做法，slice的partition_point没有找到的时候返回数组的长度 struct Solution {}\nimpl Solution { pub fn search(nums: Vec<i32>, target: i32) -> i32 { let x = nums.partition_point(|&x| x < target); if x == nums.len() { return -1 } if nums[x] == target { return x as i32 } -1 }\n} 手写的二分查找 struct Solution {}\nimpl Solution { pub fn search(nums: Vec<i32>, target: i32) -> i32 { let mut left = 0; let mut right = nums.len(); while left < right { let mid = left + (right - left) / 2; if nums[mid] < target { left = mid + 1 } else if nums[mid] > target { right = mid; } else { return mid as i32 } } -1 }\n}","breadcrumbs":"Day 1 » 704. 二分查找 » 解题思路","id":"6","title":"解题思路"},"7":{"body":"对区间的定义没有想清楚， 区间的定义就是不变量 ，在操作的过程中 保持不变量 在左闭右闭区间的情况下 由于right 要 -1，所以要考虑right=0 - 1的情况 struct Solution {}\nimpl Solution { pub fn search(nums: Vec<i32>, target: i32) -> i32 { let mut left = 0isize; let mut right = nums.len() as isize - 1; while left <= right { let mid = (left + (right - left) / 2) as usize; if nums[mid] < target { left = mid as isize + 1 } else if nums[mid] > target { right = mid as isize - 1; } else { return mid as i32 } } -1 }\n}","breadcrumbs":"Day 1 » 704. 二分查找 » 学习感想","id":"7","title":"学习感想"}},"length":8,"save":true},"fields":["title","body","breadcrumbs"],"index":{"body":{"root":{"0":{"df":1,"docs":{"6":{"tf":1.0}},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"z":{"df":1,"docs":{"7":{"tf":1.0}}}}}}},"1":{",":{"0":{",":{"3":{",":{"5":{",":{"9":{",":{"1":{"2":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":3,"docs":{"5":{"tf":2.0},"6":{"tf":2.0},"7":{"tf":2.23606797749979}},"，":{"df":0,"docs":{},"所":{"df":0,"docs":{},"以":{"df":0,"docs":{},"要":{"df":0,"docs":{},"考":{"df":0,"docs":{},"虑":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"=":{"0":{"df":1,"docs":{"7":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}}}}},"2":{"7":{"df":2,"docs":{"1":{"tf":1.0},"3":{"tf":1.0}}},"df":3,"docs":{"5":{"tf":1.7320508075688772},"6":{"tf":1.0},"7":{"tf":1.0}}},"3":{"4":{"df":1,"docs":{"2":{"tf":1.0}}},"5":{"df":1,"docs":{"2":{"tf":1.0}}},"df":0,"docs":{}},"4":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"7":{"0":{"4":{"df":3,"docs":{"1":{"tf":1.0},"2":{"tf":1.7320508075688772},"4":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"9":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"3":{"tf":1.0}}}}}}}}},"f":{"df":0,"docs":{},"n":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"p":{"df":0,"docs":{},"s":{":":{"/":{"/":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"c":{"df":0,"docs":{},"o":{"d":{"df":0,"docs":{},"e":{".":{"c":{"df":0,"docs":{},"n":{"/":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"b":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"s":{"/":{"b":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"2":{"tf":1.0}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"v":{"df":1,"docs":{"3":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}},"w":{"df":0,"docs":{},"w":{"df":0,"docs":{},"w":{".":{"b":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{".":{"c":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"/":{"df":0,"docs":{},"v":{"df":0,"docs":{},"i":{"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"o":{"/":{"b":{"df":0,"docs":{},"v":{"1":{"2":{"a":{"4":{"df":0,"docs":{},"y":{"1":{"df":0,"docs":{},"z":{"7":{"df":0,"docs":{},"l":{"df":0,"docs":{},"p":{"df":1,"docs":{"3":{"tf":1.0}}}}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{},"f":{"a":{"4":{"df":0,"docs":{},"y":{"1":{"df":0,"docs":{},"o":{"7":{"1":{"5":{"df":1,"docs":{"2":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"df":0,"docs":{}}}},"df":0,"docs":{}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"i":{"3":{"2":{"df":2,"docs":{"6":{"tf":2.449489742783178},"7":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}}},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"z":{"df":1,"docs":{"7":{"tf":1.7320508075688772}}}}}},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":2.23606797749979},"7":{"tf":2.23606797749979}}}}}},"m":{"df":0,"docs":{},"i":{"d":{"df":2,"docs":{"6":{"tf":2.0},"7":{"tf":2.0}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}}}},"n":{"df":1,"docs":{"5":{"tf":1.0}},"u":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":2.449489742783178}},"s":{".":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"_":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"(":{"df":0,"docs":{},"|":{"&":{"df":0,"docs":{},"x":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}}}},"df":0,"docs":{}}}}}}}}},"df":0,"docs":{}}},"[":{"df":0,"docs":{},"m":{"df":0,"docs":{},"i":{"d":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}},"df":0,"docs":{}}},"x":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"0":{"1":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"df":0,"docs":{},"u":{"b":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}},"df":0,"docs":{}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"n":{"df":2,"docs":{"6":{"tf":1.7320508075688772},"7":{"tf":1.0}}}}}}},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":2.0},"7":{"tf":2.23606797749979}}}}}}},"s":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"r":{"c":{"df":0,"docs":{},"h":{"(":{"df":0,"docs":{},"n":{"df":0,"docs":{},"u":{"df":0,"docs":{},"m":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}}}},"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":2.0},"7":{"tf":1.4142135623730951}}}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"u":{"c":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}},"df":0,"docs":{}}}}},"t":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":3,"docs":{"5":{"tf":2.0},"6":{"tf":2.449489742783178},"7":{"tf":1.7320508075688772}}}}}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"z":{"df":1,"docs":{"7":{"tf":1.0}}}}}},"v":{"df":0,"docs":{},"e":{"c":{"<":{"df":0,"docs":{},"i":{"3":{"2":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"x":{"df":1,"docs":{"6":{"tf":2.0}}}}},"breadcrumbs":{"root":{"0":{"df":1,"docs":{"6":{"tf":1.0}},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"z":{"df":1,"docs":{"7":{"tf":1.0}}}}}}},"1":{",":{"0":{",":{"3":{",":{"5":{",":{"9":{",":{"1":{"2":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":8,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"2":{"tf":1.0},"3":{"tf":1.0},"4":{"tf":1.0},"5":{"tf":2.23606797749979},"6":{"tf":2.23606797749979},"7":{"tf":2.449489742783178}},"，":{"df":0,"docs":{},"所":{"df":0,"docs":{},"以":{"df":0,"docs":{},"要":{"df":0,"docs":{},"考":{"df":0,"docs":{},"虑":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"=":{"0":{"df":1,"docs":{"7":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}}}}},"2":{"7":{"df":2,"docs":{"1":{"tf":1.0},"3":{"tf":1.4142135623730951}}},"df":3,"docs":{"5":{"tf":1.7320508075688772},"6":{"tf":1.0},"7":{"tf":1.0}}},"3":{"4":{"df":1,"docs":{"2":{"tf":1.0}}},"5":{"df":1,"docs":{"2":{"tf":1.0}}},"df":0,"docs":{}},"4":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"7":{"0":{"4":{"df":6,"docs":{"1":{"tf":1.0},"2":{"tf":2.0},"4":{"tf":1.7320508075688772},"5":{"tf":1.0},"6":{"tf":1.0},"7":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"9":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}},"d":{"a":{"df":0,"docs":{},"y":{"df":8,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"2":{"tf":1.0},"3":{"tf":1.0},"4":{"tf":1.0},"5":{"tf":1.0},"6":{"tf":1.0},"7":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"3":{"tf":1.0}}}}}}}}},"f":{"df":0,"docs":{},"n":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"p":{"df":0,"docs":{},"s":{":":{"/":{"/":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"c":{"df":0,"docs":{},"o":{"d":{"df":0,"docs":{},"e":{".":{"c":{"df":0,"docs":{},"n":{"/":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"b":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"s":{"/":{"b":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"2":{"tf":1.0}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"v":{"df":1,"docs":{"3":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}},"w":{"df":0,"docs":{},"w":{"df":0,"docs":{},"w":{".":{"b":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{".":{"c":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"/":{"df":0,"docs":{},"v":{"df":0,"docs":{},"i":{"d":{"df":0,"docs":{},"e":{"df":0,"docs":{},"o":{"/":{"b":{"df":0,"docs":{},"v":{"1":{"2":{"a":{"4":{"df":0,"docs":{},"y":{"1":{"df":0,"docs":{},"z":{"7":{"df":0,"docs":{},"l":{"df":0,"docs":{},"p":{"df":1,"docs":{"3":{"tf":1.0}}}}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{},"f":{"a":{"4":{"df":0,"docs":{},"y":{"1":{"df":0,"docs":{},"o":{"7":{"1":{"5":{"df":1,"docs":{"2":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"df":0,"docs":{}}}},"df":0,"docs":{}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"i":{"3":{"2":{"df":2,"docs":{"6":{"tf":2.449489742783178},"7":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}}},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"z":{"df":1,"docs":{"7":{"tf":1.7320508075688772}}}}}},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"f":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":2.23606797749979},"7":{"tf":2.23606797749979}}}}}},"m":{"df":0,"docs":{},"i":{"d":{"df":2,"docs":{"6":{"tf":2.0},"7":{"tf":2.0}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}}}},"n":{"df":1,"docs":{"5":{"tf":1.0}},"u":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":2.449489742783178}},"s":{".":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"_":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"(":{"df":0,"docs":{},"|":{"&":{"df":0,"docs":{},"x":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}}}},"df":0,"docs":{}}}}}}}}},"df":0,"docs":{}}},"[":{"df":0,"docs":{},"m":{"df":0,"docs":{},"i":{"d":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}},"df":0,"docs":{}}},"x":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"0":{"1":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"df":0,"docs":{},"u":{"b":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}},"df":0,"docs":{}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"n":{"df":2,"docs":{"6":{"tf":1.7320508075688772},"7":{"tf":1.0}}}}}}},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":2.0},"7":{"tf":2.23606797749979}}}}}}},"s":{"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"r":{"c":{"df":0,"docs":{},"h":{"(":{"df":0,"docs":{},"n":{"df":0,"docs":{},"u":{"df":0,"docs":{},"m":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}}}},"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":2.0},"7":{"tf":1.4142135623730951}}}}}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"u":{"c":{"df":0,"docs":{},"t":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}}},"df":0,"docs":{}}}}},"t":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":3,"docs":{"5":{"tf":2.0},"6":{"tf":2.449489742783178},"7":{"tf":1.7320508075688772}}}}}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"z":{"df":1,"docs":{"7":{"tf":1.0}}}}}},"v":{"df":0,"docs":{},"e":{"c":{"<":{"df":0,"docs":{},"i":{"3":{"2":{"df":2,"docs":{"6":{"tf":1.4142135623730951},"7":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"x":{"df":1,"docs":{"6":{"tf":2.0}}}}},"title":{"root":{"2":{"7":{"df":1,"docs":{"3":{"tf":1.0}}},"df":0,"docs":{}},"7":{"0":{"4":{"df":2,"docs":{"2":{"tf":1.0},"4":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"0":{"1":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"df":0,"docs":{}}}}},"lang":"English","pipeline":["trimmer","stopWordFilter","stemmer"],"ref":"id","version":"0.9.5"},"results_options":{"limit_results":30,"teaser_word_count":30},"search_options":{"bool":"OR","expand":true,"fields":{"body":{"boost":1},"breadcrumbs":{"boost":1},"title":{"boost":2}}}});