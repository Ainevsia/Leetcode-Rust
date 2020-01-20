fn main() {
    let n5 = ListNode { val: 5, next: None };
    let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
    let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
    let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
    let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
    let head = Some(Box::new(n1));
    // if let Some(mut b) = head {
    //     head = b.next.take();
    //     let x = b;
    //     println!("x = {:#?}", x);
    // }
    // println!("head = {:#?}", head);
    // let x = &mut head;
    // // let y = x.unwrap();
    // x.as_mut().unwrap().next.as_mut().unwrap().val = 4;
    // println!("x = {:#?}", x);
    let head = Solution::remove_nth_from_end(head, 2);
    println!("head = {:#?}", head);
    // let x = Box::new(vec![1,2,3]);
    // let y = Some(x);
    // match y {
    //     None => {},
    //     Some(ss) => {},
    // };
    // // println!("x = {:?}", x); // cannot do : x moved to y
    // println!("y = {:?}", y);
}

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]   // what kind of macro is this?
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
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
            head = node.next.take();    // head takes the ownership of the rest list
            buf.push(node.val);
        }   // the owned node (1 struct ListNode) dropped here
        buf.remove(0);
        for i in buf {
            *new_node = Some(Box::new(ListNode::new(i)));
            new_node = &mut new_node.as_mut().unwrap().next;
        }
        new_head
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let n5 = ListNode { val: 5, next: None };
        let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
        let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head = Some(Box::new(n1));

        let n5 = ListNode { val: 5, next: None };
        let n3 = ListNode { val: 3, next: Some(Box::new(n5)) };
        let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
        let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
        let head_ = Some(Box::new(n1));

        assert_eq!(
            Solution::remove_nth_from_end(head, 2),
            head_
        )
    }
}