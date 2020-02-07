在Leetcode中Rust的链表定义为

```
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
```

需要理解的是next字段的类型为`Option<Box<ListNode>>`，这个类型不存在任何的引用，暗含的意思就是：链表头是整个链表的拥有者，负责整个链表所占据内存的管理（包括最终销毁）。

进一步说，Rust中这样实现的链表和用C++实现的链表是完全不同的：每个节点不再是独立存在的了，而是被先驱节点所管理，同时也管理着它的`next`字段后所有的后驱节点。

接下来看一下一道简单的旋转链表题：

```
Given a linked list, rotate the list to the right by k places, where k is non-negative.

Example 1:

Input: 1->2->3->4->5->NULL, k = 2
Output: 4->5->1->2->3->NULL


Example 2:

Input: 0->1->2->NULL, k = 4
Output: 2->0->1->NULL
```

简单分析，题意所要求的操作就是从链表中间某处切开后前后互换