pub struct ExamRoom {
    s: Vec<i32>, // 0 -> already seated
    cnt: usize,
    n: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {

    pub fn new(n: i32) -> Self {
        Self { s: vec![i32::MAX; n as usize], cnt: 0, n: n as usize }
    }
    
    pub fn seat(&mut self) -> i32 {
        self.cnt += 1;
        // if self.cnt == 1 {
        //     self.s[0] = 0;
        //     return 0;
        // }
        let (idx, _) = self.s.iter().enumerate().rev().max_by_key(|x| x.1).unwrap();
        self.s[idx] = 0;
        for (i, cnt) in (0..idx).rev().zip(1..) {
            if self.s[i] <= cnt { break }
            self.s[i] = cnt;
        }
        for (i, cnt) in (idx+1..self.n).zip(1..) {
            if self.s[i] <= cnt { break }
            self.s[i] = cnt;
        }
        
        println!("seat = {:#?}", self.s);
        idx as i32
    }
    
    pub fn leave(&mut self, p: i32) {
        self.cnt -= 1;
        if self.cnt == 0 {
            self.s = vec![i32::MAX; self.n as usize];
            return;
        }
        let p = p as usize;
        let (mut idx, mut jdx) = (usize::MAX, usize::MAX);
        for i in (0..p).rev() { if self.s[i] == 0 { idx = i; break; } }
        for j in p+1..self.n { if self.s[j] == 0 { jdx = j; break; } }
        match (idx, jdx) {
            (usize::MAX, jdx) => {
                for (i, cnt) in (0..jdx).rev().zip(1..) { self.s[i] = cnt; }
            }
            (idx, usize::MAX) => {
                for (i, cnt) in (idx+1..self.n).zip(1..) { self.s[i] = cnt; }
            }
            (idx, jdx) => {
                for (i, cnt) in (idx+1..jdx).zip(1..) { self.s[i] = cnt; }
                for (j, cnt) in (idx+1..jdx).rev().zip(1..) {
                    if self.s[j] <= cnt { break }
                    self.s[j] = cnt;
                }
            }
        }
        println!("leave = {:#?}", self.s);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

#[cfg(test)]
mod tests {
    use crate::ExamRoom;

    #[test]
    fn basic() {
        let x = vec![1,2,3,3,2,1];
        let v = x.iter().enumerate().rev().max_by_key(|x| x.1);
        println!("{:#?}", v);
        for _ in 1..0 { 
            println!("123");
        }
        unimplemented!()
    }

    #[test]
    fn it_works() {
        let mut e = ExamRoom::new(10);
        let res = e.seat();
        assert_eq!(res, 0);
        let res = e.seat();
        assert_eq!(res, 9);
        // let res = e.seat();
        // assert_eq!(res, 4);
        // let res = e.seat();
        // assert_eq!(res, 2);
        // e.leave(4);
        // let res = e.seat();
        // assert_eq!(res, 5);
        for i in [0,9] {
            e.leave(i);
        }
        let res = e.seat();
        assert_eq!(res, 0);
        let res = e.seat();
        assert_eq!(res, 9);
        // unimplemented!()
    }
}
