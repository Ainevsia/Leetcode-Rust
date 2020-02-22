fn main() {
    assert_eq!(Solution::max_number(vec![6,7],vec![6,0,4],5),vec![6,7,6,0,4]);
}

struct Solution {}

impl Solution {
    pub fn max_number(n1: Vec<i32>, n2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut i = if k <= n2.len() { 0 } else { k - n2.len() };
        let mut max_number_i = vec![];
        loop {
            let x = Self::max_n(&n1, i);
            let y = Self::max_n(&n2, k - i);
            let tmp = Self::merge(x, y);
            max_number_i = if tmp > max_number_i { tmp } else { max_number_i };
            // println!("i = {:#?}, max_number_i = {:#?}", i, max_number_i);
            i += 1;
            if i > n1.len() || k < i { break max_number_i }
        }
    }

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

    /// calculate the max `stack_len` digit number of the vector `nums`
    pub fn max_n(nums: &Vec<i32>, stack_len: usize) -> Option<Vec<i32>> {
        if stack_len == 0 { return None }
        let mut stack: Vec<i32> = Vec::with_capacity(stack_len);
        stack.push(nums[0]);
        for i in 1..nums.len() {
            while let Some(&x) = stack.last() {
                if stack.len() == stack_len && nums[i] < x { break }
                // stack not full
                if  nums[i] > x && /* remaining digits */
                    nums.len() - i + stack.len() - 1 >= stack_len {
                    stack.pop();
                } else { break }
            }
            // discard smaller ones
            if stack.len() < stack_len { stack.push(nums[i]) }
        }
        Some(stack)
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

    }
}