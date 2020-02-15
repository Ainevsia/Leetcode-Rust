fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    let ret_2: f64 = obj.find_median();
}


use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        println!("Self = {:#?}", self);
        match (self.max_heap.peek(), self.min_heap.peek()) {
            (None, _) => self.max_heap.push(num),
            (Some(_), Some(&r)) => {
                let r = r.0;
                if num > r { self.min_heap.push(Reverse(num)) }
                else { self.max_heap.push(num) }
            }
            (Some(&l), None) => {
                if num >= l { self.min_heap.push(Reverse(num)) }
                else { self.max_heap.push(num) }
            }
        }
        println!("self = {:#?}", self);
        while self.max_heap.len() != self.min_heap.len() + 1 &&
              self.max_heap.len() != self.min_heap.len() {
            if self.max_heap.len() > self.min_heap.len() {
                let x = self.max_heap.pop().unwrap();
                self.min_heap.push(Reverse(x));
            } else {
                let x = self.min_heap.pop().unwrap().0;
                self.max_heap.push(x);
            }
        }
        println!("Self = {:#?}", self);
    }
    
    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() { // divide 2
            (self.min_heap.peek().unwrap().0 + self.max_heap.peek().unwrap()) as f64 / 2.0
        } else { *self.max_heap.peek().unwrap() as f64 }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);obj.add_num(2);obj.add_num(3);
        let ret_2: f64 = obj.find_median();
        assert_eq!(ret_2, 2.0);
    }
}