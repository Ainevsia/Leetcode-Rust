fn main() {
    let n5 = ListNode { val: 5, next: None };
    let n4 = ListNode { val: 4, next: Some(Box::new(n5)) };
    let n3 = ListNode { val: 3, next: Some(Box::new(n4)) };
    let n2 = ListNode { val: 2, next: Some(Box::new(n3)) };
    let n1 = ListNode { val: 1, next: Some(Box::new(n2)) };
    let head = Some(Box::new(n1));
    Solution::remove_nth_from_end(head, 2);
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut buf: Vec<Box<ListNode>> = Vec::with_capacity((n + 1) as usize);
        let mut cnt = 0;
        let mut p = &head;
        let mut target = *loop {
            match p {
                None => { break buf[0].clone() }
                Some(ptr) => {
                    if cnt <= n { cnt += 1 }
                    else { buf.remove(0); }
                    buf.push(ptr.clone());
                    p = &(*ptr).next;
                }
            }
        };
        // let x = target.next.unwrap();
        println!("target = {:?}", target);
        target.next = (*(target.next.unwrap())).next;
        println!("target = {:?}", target);
        println!("head = {:?}", head);
        head
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