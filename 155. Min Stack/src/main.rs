fn main() {
    println!("Hello, world!");
}

struct MinStack {
    s: Vec<i32>,    /* the normal stack */
    ms: Vec<i32>,   /* the min stack */
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            s: Vec::new(),
            ms: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.s.push(x);
        if self.ms.is_empty() || self.ms.last().unwrap() >= &x {
            self.ms.push(x);
        }
    }
    
    fn pop(&mut self) {
        if let Some(x) = self.s.pop() {
            if self.ms.last() == Some(&x) {
                self.ms.pop();
            }
        }
    }
    
    fn top(&self) -> i32 {
        self.s.last().copied().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        self.ms.last().copied().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let mut obj = MinStack::new();
        obj.push(6);
        obj.push(3);
        obj.push(5);
        obj.push(4);
        obj.push(2);
        assert_eq!(2, obj.get_min());
        obj.pop();
        assert_eq!(3, obj.get_min());
    }
}