fn main() {
    assert_eq!(Solution::max_number(vec![6,7],vec![6,0,4],5),vec![6,7,6,0,4]);
}

struct Solution {}

impl Solution {
    /// O(kn) by using more space
    pub fn max_number(n1: Vec<i32>, n2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut i = if k <= n2.len() { 0 } else { k - n2.len() };
        let iend = n1.len();
        let (n1, n2) = (MaxNumber::new(n1), MaxNumber::new(n2));
        let mut max_vec = vec![];
        loop {
            let tmp = Self::merge(n1.get(i), n2.get(k - i));
            max_vec = if tmp > max_vec { tmp } else { max_vec };
            i += 1;
            if i > iend || k < i { break max_vec }
        }
    }

    /// merge two vectors to get the max number, use vector comparsion
    /// O(n) time
    pub fn merge(x: Option<Vec<i32>>, y: Option<Vec<i32>>) -> Vec<i32> {
        match (x, y) {
            (None, Some(x)) => return x,
            (Some(x), None) => return x,
            (Some(x), Some(y)) => {
                let (n1, n2) = (x.len(), y.len());
                let mut res = vec![];
                let (mut i, mut j) = (0, 0);
                loop {
                    if i == n1 { res.append(&mut Vec::from(&y[j..])); return res }
                    if j == n2 { res.append(&mut Vec::from(&x[i..])); return res }
                    if x[i] > y[j] { res.push(x[i]); i += 1 }
                    else if x[i] < y[j] { res.push(y[j]); j += 1 }
                    else /* x[i] == y[j] */ {
                        if &x[i..] > &y[j..] { res.push(x[i]); i += 1 }
                        else { res.push(y[j]); j += 1 }
                    }
                }
            }
            _ => unreachable!()
        }
    }

    /// calculate the max `digit_cnt` digit number of the vector `nums`
    /// O(n) time
    pub fn max_n(nums: &Vec<i32>, digit_cnt: usize) -> Option<Vec<i32>> {
        if digit_cnt == 0 { return None }
        let mut stack: Vec<i32> = Vec::with_capacity(digit_cnt);
        stack.push(nums[0]);
        for i in 1..nums.len() {
            while let Some(&x) = stack.last() { // may pop a few times
                if stack.len() == digit_cnt && nums[i] < x { break }
                // stack not full
                if  nums[i] > x && /* remaining digits can fill the stack */
                    nums.len() - i + stack.len() - 1 >= digit_cnt {
                    stack.pop();
                } else { break }
            }
            // if full, discard smaller ones
            if stack.len() < digit_cnt { stack.push(nums[i]) }
        }
        Some(stack)
    }
}

pub struct MaxNumber {
    max_vec: Vec<Option<Vec<i32>>>,
}

impl MaxNumber {
    pub fn new(mut stack: Vec<i32>) -> MaxNumber {
        let mut tmp = MaxNumber { max_vec: Vec::with_capacity(stack.len() + 1) };
        // Question: is push_front of VecDeque a O(1) operation ?
        let mut i = 0;
        tmp.max_vec.push(Some(stack.clone()));
        for _ in 1..stack.len() {
            while i + 1 < stack.len() && stack[i + 1] <= stack[i] {
                i += 1
            }
            if i >= stack.len() { stack.pop(); tmp.max_vec.insert(0, Some(stack.clone())) }
            else { stack.remove(i); tmp.max_vec.insert(0, Some(stack.clone())) }
            if i > 0 { i -= 1 }
        }
        tmp.max_vec.insert(0, None);
        tmp
    }

    pub fn get(&self, i: usize) -> Option<Vec<i32>> {
        self.max_vec[i].clone()
    }

    pub fn debug(&self) {
        println!("self.max = {:#?}", self.max_vec);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn max_n() {
        assert_eq!(Solution::max_n(&vec![9,1,2,5,8,3],3),Some(vec![9,8,3]));
        assert_eq!(Solution::max_n(&vec![4,5,3,2,1,6,0,8],2),Some(vec![6,8]));
        assert_eq!(Solution::max_n(&vec![3,4,6,5],1),Some(vec![6]));
    }

    #[test]
    fn basic() {
        assert_eq!(Solution::max_number(vec![3,4,6,5],vec![9,1,2,5,8,3],5),vec![9,8,6,5,3]);
        assert_eq!(Solution::max_number(vec![6,7],vec![6,0,4],5),vec![6,7,6,0,4]);
        assert_eq!(Solution::max_number(vec![3,9],vec![8,9],3),vec![9,8,9]);
        assert_eq!(Solution::max_number(vec![2,5,6,4,4,0],vec![7,3,8,0,6,5,7,6,2],15),vec![7,3,8,2,5,6,4,4,0,6,5,7,6,2,0]);
        assert_eq!(Solution::max_number(vec![2,8,0,4,5,1,4,8,9,9,0,8,2,9],vec![5,9,6,6,4,1,0,7],22),vec![5,9,6,6,4,2,8,1,0,7,0,4,5,1,4,8,9,9,0,8,2,9]);
        assert_eq!(Solution::max_number(vec![8,6,9],vec![1,7,5],3),vec![9,7,5]);

    }

    #[test]
    fn fail() {
        assert_eq!(Solution::max_number(vec![5,5,1],vec![4,0,1],3),vec![5,5,4]);
    }
}