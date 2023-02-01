
pub fn main() {
    let s = String::from("hello, world");
    // s在这里被转移给a
    let a =Rc::new(s);
    
    dbg!(Rc::strong_count(&a));
    // dbg!(Rc::strong_count(&b));
    dbg!(Rc::weak_count(&a));
    let b =a.clone();
    te(b);
    dbg!(Rc::strong_count(&a));
    dbg!(Rc::weak_count(&a));


//  for _ in 0usize..1{
//     let x = Rc::new(RefCell::new(TreeNode::new(1))); // The internal counter of x is initialized to 1.
//     let y = Rc::new(RefCell::new(TreeNode::new(2))); // The internal counter of y is initialized to 1.

//     x.borrow_mut().left.replace(Rc::clone(&y));
//     y.borrow_mut().left.replace(Rc::clone(&x));
//     // Now x owns y and y owns x. Both counters count to 2.

//     drop(x); // The counter of x decrements to 1, because y still owns x.
//     drop(y); // PRBLEMATIC: We no longer own y, so the counter of y decrement by 1.
// }
// dbg!("ok looping");
// loop {}
}

fn te(a: Rc<String>) {

    let c = a.clone();
    // assert_eq!(2, Rc::strong_count(&a));
    // assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
    
    dbg!(Rc::strong_count(&a));
    dbg!(Rc::strong_count(&c));
    dbg!(Rc::weak_count(&a));
    dbg!(Rc::weak_count(&c));
    let b = Rc::downgrade(&c);
    dbg!(Rc::strong_count(&a));
    dbg!(Rc::weak_count(&a));
    dbg!(b.strong_count());
    dbg!(b.weak_count());
    drop(b);
    dbg!(Rc::strong_count(&a));
    dbg!(Rc::weak_count(&a));
    dbg!(Rc::weak_count(&c));
}













// mod list;
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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
      TreeNode {
        val,
        left: None,
        right: None
      }
    }
  }
  use std::rc::Rc;
  use std::cell::RefCell;
  
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(x) => {
                let mut a = x.borrow_mut();
                let b = x.borrow_mut().left.take();
                let c = x.borrow_mut().right.take();
                let b = Self::prune_tree(b);
                let c = Self::prune_tree(c);
                if b.is_none() && c.is_none() && x.borrow().val == 0 {
                    return None
                }
                x.borrow_mut().left = b;
                x.borrow_mut().right = c;
                drop(a);
                return Some(x);
            },
            None => None
        }
    }
}
pub struct A {
    pub a: i32,
}

impl A {
    pub fn a(&mut self) -> &mut A {
        self
    }
}

impl ListNode {
    pub fn iter(&self) -> Iter {
        Iter { iter: Some(self) }
    }
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            iter: Some(self),
            marker: std::marker::PhantomData,
        }
    }
}

pub unsafe fn very_bad_function<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}

pub struct B {
    b: Option<String>,
}

struct Foo {
    field: String
}

struct Bar {
    field: Option<Foo>
}

pub fn g() {
    let a = Foo { field: "sss".to_string()};
    let mut b = Bar { field: Some(a)};
    let a = b.field.take();
    let mut c = b;
    let d = &mut c;
    let e = c;
    // let x = d.field; 
    // let bar_mut: &mut Bar = &mut b;
    // bar_mut.field = None;
    // let field_value_ownership_would_be_taken = bar_mut.field.take();
    // bar_mut.field = None;
    // let b = B{b:Some("()".to_string())};
    // let a = b.b;
}

pub struct Iter<'a> {
    iter: Option<&'a ListNode>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a ListNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_none() {
            return None;
        }
        let res = self.iter;
        if let Some(x) = self.iter {
            self.iter = x.next.as_deref();
        }
        return res;
    }
}

pub struct IterMut<'a> {
    iter: Option<*mut ListNode>,
    marker: std::marker::PhantomData<&'a mut ListNode>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut ListNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.iter {
            unsafe {
                self.iter = (*x).next.as_deref_mut().map(|x| x as *mut ListNode);
                return Some(&mut *x);
            }
        } else {
            None
        }
    }
}



type Z = ListNode;
use std::cmp::Ordering;
impl PartialOrd<Z>for Z{fn partial_cmp(&self,o:&Z)->Option<Ordering>{self.val.partial_cmp(&o.val)}}
impl Ord for Z{fn cmp(&self,o:&Z)->Ordering{self.val.cmp(&o.val)}}
impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let phead = Some(Box::new(ListNode { val: 0, next: head }));
        let mut hash_map = HashMap::new();
        let mut head_clone = phead.clone();
        let mut head_refer = &phead;
        let mut sum = 0;
        while head_refer.is_some() {
            sum = sum + &head_refer.as_ref()?.val;
            hash_map.insert(sum, head_refer.clone());
            head_refer = &head_refer.as_ref()?.next;
        }
        let mut head_refer = &mut head_clone;
        sum = 0;
        while head_refer.is_some() {
            sum += &head_refer.as_mut()?.val;
            let node = hash_map.get_mut(&sum)?;
            head_refer.as_mut()?.next = node.as_mut()?.next.take();
            head_refer = &mut head_refer.as_mut()?.next;
        }
        head_clone?.next
    }

    pub fn remove_zero_sum_sublists1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v = vec![];
        while let Some(mut x) = head {
            head = x.next.take();
            if x.val != 0
            {v.push(x);}
        }
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();
        *map.entry(0).or_default() = 0;
        let mut i = 0;
        let mut sum = 0;
        while i < v.len() {
            sum += v[i].val;
            let x = map.get(&sum);
            if let Some(&y) = x {
                dbg!(&map);
                let mut past = sum;
                for w in v.drain(y..=i) {
                    past += w.val;
                    dbg!(past);
                    if past != sum
                    {map.remove_entry(&past);}
                }
                i = y;
                dbg!(&map);
            } else {
                *map.entry(sum).or_default() = i + 1;
                i += 1;
            }
        }
        let mut dummy = ListNode::new(1);
        let mut ptr = &mut dummy;
        for i in v {
            ptr.next = Some(i);
            ptr = ptr.next.as_deref_mut()?;
        }
        dummy.next
    }
}

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v = vec![];
        while let Some(mut x) = head {
            head = x.next.take();
            v.push(x);
        }
        v.sort();
        let mut dummy = ListNode::new(1);
        let mut ptr = &mut dummy;
        for i in v {
            ptr.next = Some(i);
            ptr = ptr.next.as_deref_mut()?;
        }
        dummy.next
    }
}

#[test]
    pub fn basic() {
    let head = [2,2,-2,1,-1,-1].iter().rev().fold(None, |acc,&x| {
        let mut a = ListNode::new(x);
        a.next = acc;
        Some(Box::new(a))
    });
    let a = Solution::remove_zero_sum_sublists(head);
    dbg!(a);
    unimplemented!()
}

#[allow(unused)]
use std::collections::VecDeque;
use std::collections::HashMap;
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut ptr = &mut dummy;
        loop {
            let (sub, tail) = Self::reverse_once(head, k);
            head = tail;
            ptr.next = sub;
            while ptr.next.is_some() {
                ptr = ptr.next.as_deref_mut()?;
            }
            if head.is_none() {
                return dummy.next
            }
        }
    }
    
    pub fn reverse_once(mut head: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut ptr = head.as_deref();
        for _ in 0..k {
            if ptr.is_none() { return (head, None) }
            ptr = ptr.unwrap().next.as_deref();
        }
        let mut dummy = ListNode::new(-1);
        for _ in 0..k {
            if head.is_none() { return (dummy.next, None) }
            let mut node = head.take().unwrap(); // node is not none
            head = node.next.take();
            node.next = dummy.next.take();
            dummy.next = Some(node);
        }
        (dummy.next, head)
    }
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    dummy.next = head.take();
    let mut r_cur = &mut dummy;
    loop {
        let mut o_start = r_cur.next.take();    // 1
        let mut p = &mut o_start;
        let mut cnt = 0;
        while p.is_some() {
            p = &mut p.as_deref_mut()?.next;
            cnt += 1;if cnt == k { break }
        }
        if cnt != k { // restore the head part
            r_cur.next = o_start.take();
            return dummy.next
        }
        let mut o_tail = p.take(); // 2
        
        for _ in 0..k-1 {
            let mut mid = o_start.as_deref_mut()?;
            let nxt = mid.next.take();
            mid.next = o_tail.take();
            let temp = o_start;
            o_start = nxt;
            o_tail = temp;
        }
        r_cur.next = o_start.take();
        let mid = r_cur.next.as_deref_mut()?;
        mid.next = o_tail.take();
        
        let mut p = r_cur;
        for _ in 0..k {
            p = p.next.as_deref_mut()?;
        }
        r_cur = p;
    }
}

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p = &mut head;
    while let Some(x) = p {
        while let Some(y) = x.next.as_deref_mut() {
            if x.val == y.val {
                let a = y.next.take();
                x.next = a;
            } else { break }
        }
        if let Some(x) = p {
            p = &mut x.next;
        }
    }
    head
}

// use std::cmp::Ordering;
// type Z = ListNode;
// /// implement the PartialOrd and Ord trait reversely to make a min heap
// impl PartialOrd<Z>for Z{fn partial_cmp(&self,o:&Z)->Option<Ordering>{o.val.partial_cmp(&self.val)}}
// impl Ord for Z{fn cmp(&self,o:&Z)->Ordering{o.val.cmp(&self.val)}}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::collections::BinaryHeap;
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;
    let mut h = BinaryHeap::new();
    for mut node in lists {
        if node.is_none() { continue }
        let a = node.take()?;
        h.push(a);
    }
    while h.len() > 0 {
        let a = h.pop()?;
        cur.next = Some(a);
        cur = cur.next.as_deref_mut()?;
        let a = cur.next.take();
        if a.is_some() {
            h.push(a?);
        }
    }
    dummy.next
}

pub fn merge_k_lists1(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;
    loop {
        let mut min_node: &mut Option<Box<ListNode>> = &mut None;
        for n in lists.iter_mut() {
            if let Some(x) = n {
                if let Some(y) = min_node {
                    if x.val <  y.val {
                        min_node = n
                    }
                } else {
                    min_node = n
                }
            }
        }
        if min_node.is_none() { break }
        cur.next = min_node.take();
        cur = cur.next.as_deref_mut()?;
        *min_node = cur.next.take();
    }
    dummy.next
}


pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;
    while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
        let l = if l1.val <= l2.val { &mut list1 } else { &mut list2 };
        cur.next = l.take();
        cur = cur.next.as_deref_mut()?;
        *l = cur.next.take();
    }
    cur.next = list1.or(list2);
    dummy.next
}
pub struct Solution {}
impl Solution {
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // I: 找到 a 的位置.
        let mut cur = (0..a - 1).fold(list1.as_deref_mut()?, |acc, _| {
            acc.next.as_deref_mut().unwrap()
        });

        // II: 把 a 替换为 list2.
        let ptr_a = std::mem::replace(&mut cur.next, list2);

        // III: cur 游走到链表末尾.
        while cur.next.is_some() {
            cur = cur.next.as_deref_mut()?;
        }

        // IV: 找到 b 的位置, 把 cur.next 设为 ptr_b.
        cur.next = (0..=b - a).fold(ptr_a, |acc, _| acc.unwrap().next);

        // V: 返回头.
        list1
    }
}
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(v) = node {
            let v = v.borrow();

            helper(&v.left, ret);
            ret.push(v.val);
            helper(&v.right, ret);
        }
    }

    let mut ret = vec![];

    if let Some(v) = root {
        helper(&Some(v), &mut ret);
    }

    ret
}
pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort();
    let n = arr.len();
    if arr[0] != 1 { arr[0] = 1 }
    for i in 1..n {
        if arr[i] > arr[i-1] + 1 {
            arr[i] = arr[i-1] + 1
        }
    }
    arr[n-1]
}


impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // let mut dummy = ListNode::new(-1);
        // dummy.next = head;
        // let mut ptr = Some(& dummy);
        // for _ in 0..n { ptr = ptr.unwrap().next.as_deref() }
        // let mut ptr2 = &mut dummy;
        // while let Some(x) = ptr {

        // }
        todo!()
    }
    pub fn remove_nth_from_end1(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        while let Some(mut x) = head {
            head = x.next;
            x.next = dummy.next;
            dummy.next = Some(x);
        }
        let mut ptr = &mut dummy;
        for _ in 1..n { ptr = ptr.next.as_deref_mut().unwrap() }
        let drop = ptr.next.take();
        ptr.next = drop?.next.take();
        head = dummy.next.take();
        while let Some(mut x) = head {
            head = x.next;
            x.next = dummy.next;
            dummy.next = Some(x);
        }
        dummy.next
    }
}

pub fn decode_message(key: String, message: String) -> String {
    use std::collections::HashMap;
    let mut map: HashMap<char,char> = HashMap::new();
    let mut v = 'a'..='z';
    map.insert(' ', ' ');
    key.chars().filter(|&x| x != ' ').for_each(|ch| {
        if map.get(&ch).is_none() {
            map.insert(ch, v.next().unwrap());
        }
    });
    message.chars().map(|x| map.get(&x).unwrap()).collect()
}

// impl Solution {
//     pub fn merge_in_between(
//         mut list1: Option<Box<ListNode>>,
//         a: i32,
//         b: i32,
//         list2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         let mut p = list1.as_mut();
//         let mut cnt = -1;
//         while let Some(mut x) = p {
//             cnt += 1;
//             if cnt + 1 == a {
//                 let mut d = x.next.take();
//                 while let Some(y) = d.as_mut() {
//                     d = y.next.take();
//                     cnt += 1;
//                     if cnt == b {
//                         break
//                     }
//                 }
//                 x.next = list2;
//                 while let Some(_) = x.next.as_mut() {
//                     x = x.next.as_mut().unwrap();
//                 }
//                 x.next= d;
//                 return list1
//             }
//             p = x.next.as_mut();
//         }
//         unreachable!()
//     }
// }
pub fn main1() {
    dbg!(std::mem::size_of::<Foo>());
    dbg!(std::mem::size_of::<String>());
    let head = [2,2,-2,1,-1,-1].iter().rev().fold(None, |acc,&x| {
        let mut a = ListNode::new(x);
        a.next = acc;
        Some(Box::new(a))
    });
    // let a = Solution::remove_zero_sum_sublists(head);
    let b = head.clone();
    let c = Box::into_raw(head.unwrap()) as usize;
    println!("{:x}", c);
    let c = Box::into_raw(b.unwrap()) as usize;
    println!("{:x}", c);
    // dbg!(a);

    let mut v = 'a'..='z';
    for i in v {
        dbg!(i);
    }
    unimplemented!()
}
pub fn a() {
    dbg!(core::mem::size_of::<Option<Box<ListNode>>>());
    dbg!(core::mem::size_of::<Option<Box<&ListNode>>>());
    dbg!(core::mem::size_of::<Option<&Box<ListNode>>>());
    dbg!(core::mem::size_of::<&Option<Box<ListNode>>>());
    dbg!(core::mem::size_of::<ListNode>());
    dbg!(core::mem::size_of::<&ListNode>());
    let a = ListNode::new(0x11111112);
    let b = Box::new(a);
    let c = Some(b.clone());
    let d = Some(&b);
    let e = &c;
    unsafe {
        let a = &c as *const _ as *const usize;
        let b = *a as *mut usize;
        println!("{:x}", *a);
        println!("{:x}", *b);
        println!("{:x}", (*((*a) as *const ListNode)).val);
        let a = &d as *const _ as *const usize;
        let b = *a as *mut usize;
        println!("{:x}", *a);
        println!("{:x}", *b);
        println!("{:x}", (*((*b) as *const ListNode)).val);
        let a = &e as *const _ as *const usize;
        let b = *a as *mut usize;
        println!("{:x}", *a);
        println!("{:x}", *b);
    }
    println!("===================");
    let a = ListNode::new(0x11111112);
    println!("{:p}", &a);
    let b = Some(a);
    println!("{:p}", &b);
    let c = match b {
        Some(ref x) => Some(x),
        None => None,
    };
    dbg!(core::mem::size_of::<Option<ListNode>>());
    unsafe {
        let x = &b as *const _ as *const usize;
        println!("{:x}", *(x.offset(2)));
        println!("{:x}", x as usize);
        let x = &c as *const _ as *const usize;
        println!("{:x}", *x);
    }
    let a = ListNode::new(0x11111112);
    let b = Box::new(a);
    let c = Some(b.clone());
    // let d = Some(&b);
    // let e = &c;
    // let f = c.as_deref();
    dbg!(c);
}
