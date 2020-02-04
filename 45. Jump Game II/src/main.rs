fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut vec = vec![0];
        for idx in 1..nums.len() {
            let mut jmp = i32::max_value();
            println!("idx = {:#?}", idx);
            for i in 0..idx {
                println!("i = {:#?}", i);
                if i + nums[i] as usize >= idx {
                    if vec[i] + 1 < jmp {
                        jmp = vec[i] + 1;
                    }
                }
            }
            vec.push(jmp);
        }
        vec[vec.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
        assert_eq!(Solution::jump(vec![1,1,1,1,4]), 4);
        assert_eq!(Solution::jump(vec![1000,3,1,1,4]), 1);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::jump(vec![2]), 0);
        assert_eq!(Solution::jump(vec![1,1]), 1);
        
    }
}