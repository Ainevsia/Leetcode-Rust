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

接下来看一下LeetCode上一道简单的[旋转链表题](https://leetcode.com/problems/rotate-list/)：

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

![buf](https://res.cloudinary.com/ainevsia/image/upload/v1581051584/61buf.png)

链表只有一条，当将头节点指针`&mut Box<ListNode>`推入队列时，这根指针就已经有了修改**整条**链表的权利了。显然，第二个指针无法拥有mut权限了，因为不能同时存在同一内存的两个可变引用。

所以说，我的这种只需遍历一遍的方法是无法在Rust中轻松实现的。下面给出AC代码，遍历了两次链表。

```rust
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k <= 0 { return head }
        // Step 1 - loop the linked-list and count total length (Don't need mut)
        let mut ptr: Option<&Box<ListNode>> = head.as_ref();
        let mut list_len = 0;
        while let Some(node) = ptr {
            ptr = node.next.as_ref();
            list_len += 1;
        }

        // Step 2 - calculate the curoff place and reach that node using &mut Box
        // because this time we want to mutate the list
        let cutoff_cnt = list_len - k % list_len;
        if cutoff_cnt == list_len { return head }
        let mut ptr: &mut Box<ListNode> = head.as_mut().unwrap();
        let mut i = 1;
        while i < cutoff_cnt {
            ptr = ptr.next.as_mut().unwrap();
            i += 1;
        }

        // Step 3 - Split into two list and then concatenate
        // head owns one list and new_head owns the other
        let mut new_head: Option<Box<ListNode>> = ptr.next.take();  // split
        let mut ptr: Option<&mut Box<ListNode>> = new_head.as_mut();
        while let Some(node) = ptr {
            if node.next.is_none() { ptr = Some(node); break }
            ptr = node.next.as_mut();
        }
        ptr.unwrap().next = head; // concatenate
        new_head
    }
}
```

在上面的代码中，我第一遍老实地使用不可变引用遍历一遍链表统计链表长，什么也没有发生。

第二次遍历要用可变引用，但是并没有存储，所以是可以的。到达要切开链表的地方，我用了take()方法将这之后的结点（原先被`head`所拥有）转移到了由`new_head`所拥有。

```rust
let mut new_head: Option<Box<ListNode>> = ptr.next.take();  // split
```

![split](https://res.cloudinary.com/ainevsia/image/upload/v1581053510/61split.png)

最后将剩余的链表遍历完，将最后的`None`修改成`head`。现在head失去所有权，整条链表由`new_head`所拥有。

```rust
ptr.unwrap().next = head; // concatenate
```

总结：在Rust中定义链表，不要再认为节点是孤立的，一个结点代表的就是之后整条链。要对链进行操作，想清楚分裂后的两条链分别归谁拥有。Rust有效地防止了环链表的出现，比如说，一个链表只有一个next字段指向自己的节点。（XD