fn main() {
    Solution::median_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3);
}

struct Solution {}

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut heap = TwoHeap::new();
        for i in 0..(k-1) as usize { heap.insert(nums[i]); }
        let mut res = vec![];
        for i in (k-1) as usize..nums.len() {
            heap.insert(nums[i]);
            // println!("i = {:#?}, map = {:#?}", i, heap);
            res.push(heap.median());
            heap.remove(nums[i + 1 - k as usize]);
        }
        res
    }
}

#[derive(Debug)]
pub struct TwoHeap {
    max_heap: BinaryHeap<i32>,
    l_valid: usize,
    l_trash: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
    r_valid: usize,
    r_trash: BinaryHeap<Reverse<i32>>,
}

impl TwoHeap {
    pub fn new() -> TwoHeap {
        TwoHeap {
            max_heap: BinaryHeap::new(),
            l_valid: 0,
            l_trash: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
            r_valid: 0,
            r_trash: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, n: i32) {
        // first insert (regardless of balance)
        match (self.l_valid, self.r_valid) {
            (0, _) => { self.max_heap.push(n); self.l_valid = 1 }
            _ => if *self.max_heap.peek().unwrap() < n {
                self.min_heap.push(Reverse(n));
                self.r_valid += 1;
            } else {
                self.max_heap.push(n);
                self.l_valid += 1;
            }
        }
        self.balance();
    }

    pub fn remove(&mut self, n: i32) {
        // l_valid cannot be 0
        if *self.max_heap.peek().unwrap() == n
            { self.max_heap.pop(); self.l_valid -= 1 }
        else if self.r_valid > 0 && self.min_heap.peek().unwrap().0 == n
            { self.min_heap.pop(); self.r_valid -= 1 }
        else if *self.max_heap.peek().unwrap() > n {
            self.l_valid -= 1; self.l_trash.push(n);
        } else { self.r_valid -= 1; self.r_trash.push(Reverse(n)); }
        while let Some(n) = self.l_trash.peek() {
            if n == self.max_heap.peek().unwrap() {
                self.max_heap.pop();
                self.l_trash.pop();
            } else { break }
        }
        while let Some(n) = self.r_trash.peek() {
            if n == self.min_heap.peek().unwrap() {
                self.min_heap.pop();
                self.r_trash.pop();
            } else { break }
        }
        self.balance()
    }

    pub fn balance(&mut self) {
        // not a good heap, need balance
        while self.l_valid != self.r_valid && self.l_valid != self.r_valid + 1 {
            if self.l_valid > self.r_valid {
                let tmp = self.max_heap.pop().unwrap();
                self.l_valid -= 1;
                self.min_heap.push(Reverse(tmp));
                self.r_valid += 1;
            } else {
                let tmp = self.min_heap.pop().unwrap().0;
                self.r_valid -= 1;
                self.max_heap.push(tmp);
                self.l_valid += 1;
            }
            while let Some(n) = self.l_trash.peek() {
                if n == self.max_heap.peek().unwrap() {
                    self.max_heap.pop();
                    self.l_trash.pop();
                } else { break }
            }
            while let Some(n) = self.r_trash.peek() {
                if n == self.min_heap.peek().unwrap() {
                    self.min_heap.pop();
                    self.r_trash.pop();
                } else { break }
            }
        }
    }

    pub fn median(&self) -> f64 {
        if self.l_valid == self.r_valid {
            let l = *self.max_heap.peek().unwrap() as f64;
            let r = self.min_heap.peek().unwrap().0 as f64;
            return (l + r) / 2.0
        } else {
            return *self.max_heap.peek().unwrap() as f64
        }
    }
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::median_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3),
            vec![1,-1,-1,3,5,6].into_iter().map(|x| x as f64).collect::<Vec<f64>>()
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::median_sliding_window(vec![7,8,8,3,8,1,5,3,5,4], 3),
            vec![8.0,8.0,8.0,3.0,5.0,3.0,5.0,4.0].into_iter().map(|x| x as f64).collect::<Vec<f64>>()
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::median_sliding_window(vec![1,2,3,4,2,3,1,4,2], 3),
            vec![2.0,3.0,3.0,3.0,2.0,3.0,2.0].into_iter().map(|x| x as f64).collect::<Vec<f64>>()
        );
    }
}