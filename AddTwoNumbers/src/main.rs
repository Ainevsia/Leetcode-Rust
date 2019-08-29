// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

// impl Solution {
//     pub fn add_two_numbers(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
        
//         let result: Option<Box<ListNode>> = None;
//         let mut p1 = l1;
//         let p2 = l2.as_ref();

//         while let None = p1 {
//             let x: i32 = p1.unwrap();
//             p1 = (p1.unwrap().next);
//         } 
//         unimplemented!()
//     }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // take l1, l2
        let mut box1 = l1.or(Some(Box::new(ListNode::new(0)))).unwrap();
        let mut box2 = l2.or(Some(Box::new(ListNode::new(0)))).unwrap();
        
        // unwrap val1, val2: i32
        let mut val1 = box1.val;
        let mut val2 = box2.val;

        // make the first node and the head node
        let mut node_and_carry = Solution::make_node(val1 + val2);
        let mut headnode = ListNode::new(0);

        // attach the firstnoe to the headnode
        headnode.next = Some(Box::new(node_and_carry.0));

        // use cur_box to keep track
        let mut cur_box = &mut headnode.next;

        while box1.next.is_some() || box2.next.is_some() {
            box1 = box1.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
            box2 = box2.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
            
            val1 = box1.val;
            val2 = box2.val;

            node_and_carry = Solution::make_node(val1 + val2 + node_and_carry.1);

            // attach the next node to the end of cur_node
            // cur_box.unwrap().next.get_or_insert(Box::new(node_and_carry.0));
            // cur_box = &mut cur_box.unwrap().next;
            cur_box.get_or_insert(Box::new(ListNode{val: 0, next: None}))
                .next.get_or_insert(Box::new(node_and_carry.0));
            cur_box = &mut cur_box.get_or_insert(Box::new(ListNode{val: 0, next: None}))
                .next;

        }

        if node_and_carry.1 != 0 {
            // attach the final node
            cur_box.get_or_insert(Box::new(ListNode{val: 0, next: None}))
                .next.get_or_insert(Box::new(ListNode::new(node_and_carry.1)));
        }
        
        headnode.next
        
    }

    fn make_node(mut sum: i32) -> (ListNode, i32) {
        let mut carry = 0;
        if sum > 9 {
            sum -= 10;
            carry = 1;
        }
        (ListNode::new(sum), carry)
    }
}


fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };
        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };
        let result = ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        };
        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))),
            Some(Box::new(result))
        );
    }

    #[test]
    fn makenode() {
        assert_eq!(Solution::make_node(10), (ListNode{val:0, next:None}, 1));
        assert_eq!(Solution::make_node(9), (ListNode{val:9, next:None}, 0));
        assert_eq!(Solution::make_node(20), (ListNode{val:10, next:None}, 1));
    }

    #[test]
    fn init() {
        assert_eq!(ListNode::new(123), ListNode{val:123, next: None});
    }

    #[test]
    fn hund() {
        let l1 = ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: None,
            })),
        };
        let l2 = ListNode {
            val: 1,
            next: None,
        };
        let result = ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        };
        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))),
            Some(Box::new(result))
        );
    }
}
