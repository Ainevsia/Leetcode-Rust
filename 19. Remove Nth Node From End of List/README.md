# 题解

并不是多困难的题，想法也十分直接，但是在rust实现的时候遇到了巨大的困难。趁机学习一波借用规则。

```rust
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
```

这是类似c风格的list定义方法，头节点的类型为`Option<Box<ListNode>>`，头节点是拥有整条链表的所有权的。

```rust
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) 
    -> Option<Box<ListNode>> 
{
    let mut buf = Vec::with_capacity(n as usize);
    let mut cnt = 0;
    let mut new_head = None;
    let mut new_node = &mut new_head;
    while let Some(mut node) = head {  // node takes ownership of the box
        if cnt < n { cnt += 1 }
        else {
            *new_node = Some(Box::new(ListNode::new(buf.remove(0))));
            new_node = &mut new_node.as_mut().unwrap().next;
        }
        head = node.next.take();
        buf.push(node.val);
    }   // the owned node dropped here
    buf.remove(0);
    for i in buf {
        *new_node = Some(Box::new(ListNode::new(i)));
        new_node = &mut new_node.as_mut().unwrap().next;
    }
    new_head
}
```

这里的function signiature表明head的所有权是move进来的，也要返回所有权。

默认的match some（x）会把所有权转移进入x。