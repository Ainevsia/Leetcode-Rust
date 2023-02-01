use std::collections::BTreeSet;

pub struct ExamRoom {
    set: BTreeSet<i32>,
    n: i32,
}


impl ExamRoom {

    pub fn new(n: i32) -> Self {
        Self { set: BTreeSet::new(), n: n }
    }
    
    pub fn seat(&mut self) -> i32 { // O(n)
        if self.set.len() == 0 { self.set.insert(0); return 0; } // nobody 
        let mut pre = *self.set.iter().next().unwrap(); // set.len() != 0, cannot panic
        let mut lol = pre;
        let mut idx = 0;
        for &rgt in &self.set {
            if lol < (rgt - pre) / 2 {
                lol = (rgt - pre) / 2;
                idx = (rgt + pre) / 2;
            }
            pre = rgt;
        }
        // right hand empty
        if lol < self.n - 1 - pre {
            idx = self.n - 1;
        }
        self.set.insert(idx);
        return idx;
    }
    
    pub fn leave(&mut self, p: i32) { // O(logn)
        self.set.remove(&p);
    }
}

#[cfg(test)]
mod tests {
    use crate::ExamRoom;

    #[test]
    fn basic() {
    }

    #[test]
    fn it_works() {
        let mut e = ExamRoom::new(10);
        let res = e.seat();
        assert_eq!(res, 0);
        let res = e.seat();
        assert_eq!(res, 9);
        let res = e.seat();
        assert_eq!(res, 4);
        let res = e.seat();
        assert_eq!(res, 2);
        e.leave(4);
        let res = e.seat();
        assert_eq!(res, 5);
        for i in [0,9,4,5,2] {
            e.leave(i);
        }
        let res = e.seat();
        assert_eq!(res, 0);
        let res = e.seat();
        assert_eq!(res, 9);
        // unimplemented!()
    }
}
