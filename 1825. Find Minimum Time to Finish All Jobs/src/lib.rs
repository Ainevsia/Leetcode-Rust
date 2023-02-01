use std::collections::BTreeMap;
use std::collections::VecDeque;

pub struct MKAverage {
    q: VecDeque<i32>,
    t: BTreeMap<i32, usize>,
    m: usize,
    k: usize,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
pub fn a() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    println!("a指向的节点 = {:?}", a.tail());

    // 创建`b`到`a`的引用
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.tail());

    // 利用RefCell的可变性，创建了`a`到`b`的引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    // 下面一行println!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    println!("a next item = {:?}", a.tail());
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {

    pub fn new(m: i32, k: i32) -> Self {
        Self { 
            q: VecDeque::with_capacity(m as usize), 
            t: BTreeMap::new(),
            m: m as usize,
            k: k as usize,
        }
    }
    
    pub fn add_element(&mut self, num: i32) {
        let n = self.q.len();
        if n < self.m {
            self.q.push_back(num);
            *self.t.entry(num).or_default() += 1;
        } else {
            let e = self.q.pop_front().unwrap();
            self.q.push_back(num);

            let prev = self.t.get_mut(&e).unwrap();
            *prev -= 1;
            if *prev == 0 {
                self.t.remove_entry(&e);
            }
            *self.t.entry(num).or_default() += 1;
        }
    }
    
    pub fn calculate_mk_average(&self) -> i32 {
        if self.q.len() < self.m { return -1 }
        let k = self.k;
        let mut cnt = 0;
        let mut sub = 0;
        for (&x, &y) in self.t.iter() {
            if cnt + y >= k {
                sub += x as usize * (k - cnt);
                break
            }
            cnt += y;
            sub += x as usize * y;
        }
        cnt = 0;
        for (&x, &y) in self.t.iter().rev() {
            if cnt + y >= k {
                sub += x as usize * (k - cnt);
                break
            }
            cnt += y;
            sub += x as usize * y;
        }
        // dbg!(&self.q);
        // dbg!(&self.t);
        let res = self.q.iter()
            .fold(0usize, |acc, &x| acc + x as usize) - sub;
        (res / (self.m - 2 * self.k)) as i32
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        a();
    }

    #[test]
    fn it_works() {
        let mut obj = MKAverage::new(3, 1);
        obj.add_element(3);        // 当前元素为 [3]
        obj.add_element(1);        // 当前元素为 [3,1]
        obj.calculate_mk_average(); // 返回 -1 ，因为 m = 3 ，但数据流中只有 2 个元素
        let result: i32 = obj.calculate_mk_average();
        assert_eq!(result, -1);
        obj.add_element(5);        // 当前元素为 [3,1,10,5]
        obj.add_element(5);        // 当前元素为 [3,1,10,5,5]
        obj.add_element(5);        // 当前元素为 [3,1,10,5,5,5]
        let result: i32 = obj.calculate_mk_average();
        assert_eq!(result, 5);
    }
}
