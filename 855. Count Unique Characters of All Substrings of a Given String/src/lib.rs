use std::{collections::{BTreeSet, BinaryHeap}, cmp::Ordering};

#[derive(Eq, PartialEq)]
pub struct Zone {
    pub l: i32,
    pub r: i32,
}

impl Zone {
    pub fn new(l: i32, r: i32) -> Self {
        Self { l, r }
    }
}

pub struct ExamRoom {
    set: BTreeSet<i32>,
    pq : BinaryHeap<Zone>,
    n  : i32,
}

// impl PartialEq for Zone {
//     fn eq(&self, other: &Self) -> bool {
//         (self.l, self.r) == (other.l, other.r)
//     }
// }

// impl Eq for Zone {}

impl PartialOrd for Zone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Some(self.cmp(other))
        let x = (self.r - self.l) / 2;
        let y = (other.r - other.l) / 2;
        if x == y { Some(other.l.cmp(&self.l)) }
        else { Some(x.cmp(&y)) }
    }
}

impl Ord for Zone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl ExamRoom {

    pub fn new(n: i32) -> Self {
        Self {
            set: BTreeSet::new(),
            pq : BinaryHeap::new(),
            n,
        }
    }
    
    pub fn seat(&mut self) -> i32 { // O(logN)
        if self.set.len() == 0 {
            self.set.insert(0);
            return 0;
        }
        let left = *self.set.iter().next().unwrap();    // self.set.len() != 0, cannot panic
        let right= self.n - 1 - *self.set.iter().rev().next().unwrap();
        // for i in &self.pq {
            // dbg!(&self.pq);
        // }
        while self.set.len() >= 2 && self.pq.len() > 0 {
            let z = self.pq.peek().unwrap();
            // check zone is valid
            if !self.set.contains(&z.l) || !self.set.contains(&z.r) { self.pq.pop(); continue }
            if &z.r != self.set.iter().skip_while(|&&x| x<=z.l).next().unwrap() { self.pq.pop(); continue }
            let zone_len = (z.r - z.l) / 2;
            if zone_len <= left || zone_len < right { break }
            // now pop it
            let z = self.pq.pop().unwrap();
            // take this zone
            let idx = z.l + zone_len;
            if z.l + 1 != idx { self.pq.push(Zone::new(z.l, idx)) }
            if idx + 1 != z.r { self.pq.push(Zone::new(idx, z.r)) }
            self.set.insert(idx);
            return idx;
        }
        // choose from left or right
        if right > left {
            // take right
            let idx = self.n - 1;
            let rdx = self.n - 1 - right;
            if rdx + 1 != idx { self.pq.push(Zone::new(rdx, idx)) }
            self.set.insert(idx);
            return idx;
        } else {
            // take left
            let idx = 0;
            if idx + 1 != left { self.pq.push(Zone::new(idx, left)) }
            self.set.insert(idx);
            return idx;
        }
    }
    
    pub fn leave(&mut self, p: i32) { // O(logN)
        let left = *self.set.iter().next().unwrap();    // self.set.len() != 0, cannot panic
        let right= *self.set.iter().rev().next().unwrap();
        if p!=left && p!=right {
            // in the middle, add a new zone
           let higher = *self.set.iter().skip_while(|&&x| x<=p).next().unwrap();
           let lower  = *self.set.iter().rev().skip_while(|&&x| x>=p).next().unwrap();
           self.pq.push(Zone::new(lower, higher));
        }
        self.set.remove(&p);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use crate::{ExamRoom, Zone};

    #[test]
    fn map() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(1, 1);
        unimplemented!()
    }

    #[test]
    fn basic() {
        let mut pq = BinaryHeap::new();
        pq.push(Zone::new(0, 4));
        pq.push(Zone::new(4, 10));
        // pq.push(Zone::new(1, 9));
        // pq.push(Zone::new(2, 9));
        // let x = pq.pop();
        // let mut pq = BinaryHeap::new();
        // pq.push(1);
        // pq.push(2);
        // let x = pq.pop();
        // println!("{:#?}", x);
        dbg!(Zone::new(5, 10).cmp(&Zone::new(4, 111)));
        // unimplemented!()
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
    }

    #[test]
    fn hard() {
        let mut e = ExamRoom::new(10);
        let res = e.seat();
        assert_eq!(res, 0);
        let res = e.seat();
        assert_eq!(res, 9);
        let res = e.seat();
        assert_eq!(res, 4);
        e.leave(0);e.leave(4);
        let res = e.seat();
        assert_eq!(res, 0);
        let res = e.seat();
        assert_eq!(res, 4);
        let res = e.seat();
        assert_eq!(res, 2);
        let res = e.seat();
        assert_eq!(res, 6);
        let res = e.seat();
        assert_eq!(res, 1);
        let res = e.seat();
        assert_eq!(res, 3);
        let res = e.seat();
        assert_eq!(res, 5);
        let res = e.seat();
        assert_eq!(res, 7);
        let res = e.seat();
        assert_eq!(res, 8);
        e.leave(0);e.leave(4);
        let res = e.seat();
        assert_eq!(res, 0);
        let res = e.seat();
        assert_eq!(res, 4);
    }
}
