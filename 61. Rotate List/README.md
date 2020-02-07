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

简单分析例子，题意所要求的操作就是从链表中间某处切开后前后互换（inplace）。

![image](https://res.cloudinary.com/ainevsia/image/upload/v1581048900/61.png)

针对这个例子，我第一想法是用一个长度为3的队列遍历这条链表，每经过一个节点就将指向该节点的智能指针`Box`从队尾加入队列，超出队列长度3的话队头节点离开。这样的话，遍历结束后，队列中剩下指针指向的元素是3，4，5。加上头节点，可以完成所有的操作了，好处是只需遍历一遍。

然而这样的想法用C++写简单方便，但是并不是think in Rust，而且Rust也不允许这么做。

首先我们需要使用队列中的指针对原来的链表进行操作，则推入队列的必须是`&mut Box<ListNode>`。

到这里已经出现问题了。把一开始的思路用rust写出来，实际得到的是：



总结：在这样定义的链表中，不要再想结点的概念，一个结点代表的就是整条链。要对链进行分裂，想清楚分裂后的两条链分别归谁管理。